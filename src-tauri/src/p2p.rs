use libp2p::{
    core::upgrade,
    gossipsub::{self, MessageId, ValidationMode},
    identify, kad, mdns, noise, ping, swarm::NetworkBehaviour, tcp, yamux, PeerId, Swarm,
    identity,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use tokio::sync::mpsc;
use futures::StreamExt;

#[derive(NetworkBehaviour)]
pub struct MyceliumBehaviour {
    pub kad: kad::Behaviour<kad::store::MemoryStore>,
    pub identify: identify::Behaviour,
    pub ping: ping::Behaviour,
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

pub struct P2PNode {
    swarm: Swarm<MyceliumBehaviour>,
    event_sender: mpsc::UnboundedSender<P2PEvent>,
}

#[derive(Debug, Clone)]
pub enum P2PEvent {
    PeerConnected { peer_id: String },
    PeerDisconnected { peer_id: String },
    StatusUpdate { status_text: String },
    PeerCount { count: usize },
}

impl P2PNode {
    pub async fn new() -> Result<(Self, mpsc::UnboundedReceiver<P2PEvent>), Box<dyn std::error::Error>> {
        let mut swarm = {
            let local_key = identity::Keypair::generate_ed25519();
            let local_peer_id = PeerId::from(local_key.public());
            log::info!("Local peer id: {local_peer_id:?}");

            let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
                .into_authentic(&local_key)
                .expect("Signing libp2p-noise static DH keypair failed.");

            let transport = tcp::tokio::Transport::new(tcp::Config::default().nodelay(true))
                .upgrade(upgrade::Version::V1)
                .authenticate(noise::NoiseAuthenticated::xx(noise_keys).into_authenticated())
                .multiplex(yamux::YamuxConfig::default())
                .boxed();

            let mut behaviour = MyceliumBehaviour {
                kad: kad::Behaviour::new(
                    kad::store::MemoryStore::new(local_peer_id),
                ),
                identify: identify::Behaviour::new(identify::Config::new(
                    "/mycelium/1.0.0".to_string(),
                    local_key.public(),
                )),
                ping: ping::Behaviour::new(ping::Config::default()),
                gossipsub: gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(local_key),
                    gossipsub::Config::default(),
                )?,
                mdns: mdns::tokio::Behaviour::new(mdns::Config::default())?,
            };

            // Bootstrap nodes for Kademlia
            let bootstrap_nodes = vec![
                "/dnsaddr/bootstrap.mycelium.network/tcp/4001/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBYjJ7L6AWF",
            ];

            for addr in bootstrap_nodes {
                if let Ok(addr) = addr.parse() {
                    behaviour.kad.add_address(&PeerId::random(), addr);
                }
            }

            Swarm::new(transport, behaviour, local_peer_id)
        };

        let (event_sender, event_receiver) = mpsc::unbounded_channel();

        Ok((P2PNode { swarm, event_sender }, event_receiver))
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Listen on all interfaces and whatever port the OS assigns
        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        // Listen on all interfaces and whatever port the OS assigns for IPv6
        self.swarm.listen_on("/ip6/::/tcp/0".parse()?)?;

        // Listen on all interfaces and whatever port the OS assigns for WebSocket
        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0/ws".parse()?)?;

        // Listen on all interfaces and whatever port the OS assigns for WebSocket IPv6
        self.swarm.listen_on("/ip6/::/tcp/0/ws".parse()?)?;

        log::info!("P2P node started");
        self.event_sender.send(P2PEvent::StatusUpdate {
            status_text: "P2P node started".to_string(),
        })?;

        Ok(())
    }

    pub async fn run_event_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            match self.swarm.next().await {
                Some(swarm_event) => {
                    match swarm_event {
                        libp2p::swarm::SwarmEvent::Behaviour(MyceliumBehaviourEvent::Kad(kad::Event::OutboundQueryCompleted { result, .. })) => {
                            match result {
                                kad::QueryResult::Bootstrap(Ok(kad::BootstrapOk { peer, .. })) => {
                                    log::info!("Bootstrap completed with peer: {peer}");
                                    self.event_sender.send(P2PEvent::StatusUpdate {
                                        status_text: format!("Bootstrap completed with peer: {peer}"),
                                    })?;
                                }
                                kad::QueryResult::Bootstrap(Err(kad::BootstrapError::NoPeers)) => {
                                    log::warn!("Bootstrap failed: no peers found");
                                    self.event_sender.send(P2PEvent::StatusUpdate {
                                        status_text: "Bootstrap failed: no peers found".to_string(),
                                    })?;
                                }
                                _ => {}
                            }
                        }
                        libp2p::swarm::SwarmEvent::Behaviour(MyceliumBehaviourEvent::Identify(identify::Event::Received { peer_id, info })) => {
                            log::info!("Received identify info from {peer_id}: {info:?}");
                        }
                        libp2p::swarm::SwarmEvent::Behaviour(MyceliumBehaviourEvent::Ping(ping::Event { peer, result })) => {
                            match result {
                                ping::PingResult::Pong { latency } => {
                                    log::debug!("Pong from {peer}: {latency:?}");
                                }
                                ping::PingResult::Timeout { peer } => {
                                    log::debug!("Ping timeout for {peer}");
                                }
                            }
                        }
                        libp2p::swarm::SwarmEvent::Behaviour(MyceliumBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                            for (peer_id, _) in list {
                                log::info!("mDNS discovered peer: {peer_id}");
                                self.event_sender.send(P2PEvent::PeerConnected {
                                    peer_id: peer_id.to_string(),
                                })?;
                            }
                        }
                        libp2p::swarm::SwarmEvent::Behaviour(MyceliumBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                            for (peer_id, _) in list {
                                log::info!("mDNS expired peer: {peer_id}");
                                self.event_sender.send(P2PEvent::PeerDisconnected {
                                    peer_id: peer_id.to_string(),
                                })?;
                            }
                        }
                        libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                            log::info!("Listening on {address}");
                        }
                        libp2p::swarm::SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                            log::info!("Connected to {peer_id}");
                            self.event_sender.send(P2PEvent::PeerConnected {
                                peer_id: peer_id.to_string(),
                            })?;
                        }
                        libp2p::swarm::SwarmEvent::ConnectionClosed { peer_id, .. } => {
                            log::info!("Disconnected from {peer_id}");
                            self.event_sender.send(P2PEvent::PeerDisconnected {
                                peer_id: peer_id.to_string(),
                            })?;
                        }
                        _ => {}
                    }
                }
                None => break,
            }
        }

        Ok(())
    }

    pub fn get_peer_count(&self) -> usize {
        // This is a simplified implementation
        // In a real implementation, you'd track connected peers
        0
    }
} 