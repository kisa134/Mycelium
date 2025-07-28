use libp2p::{
    identity,
    mdns::{self, tokio::Behaviour},
    swarm::{Swarm, SwarmEvent},
    PeerId,
};
use std::error::Error;
use tokio::time::{self, Duration};
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};

// Наше поведение сети: пока что только mDNS для локального обнаружения.
#[derive(libp2p::swarm::NetworkBehaviour)]
#[behaviour(to_swarm = "MyEvent")]
struct MyBehaviour {
    mdns: Behaviour,
}

// События, которые будет генерировать наше поведение
#[derive(Debug)]
enum MyEvent {
    Mdns(mdns::Event),
}

impl From<mdns::Event> for MyEvent {
    fn from(event: mdns::Event) -> Self {
        MyEvent::Mdns(event)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PeerStatus {
    Discovered,
    Connected,
    Disconnected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub id: String,
    pub status: PeerStatus,
    pub last_seen: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    pub total_peers: usize,
    pub connected_peers: usize,
    pub discovered_peers: usize,
    pub peers: Vec<Peer>,
    pub local_peer_id: String,
}

#[derive(Debug, Clone)]
pub enum P2PEvent {
    PeerConnected { peer_id: String },
    PeerDisconnected { peer_id: String },
    StatusUpdate { status_text: String },
    PeerCount { count: usize },
    NetworkStatusUpdate { status: NetworkStatus },
}

pub struct RealP2PNode {
    swarm: Swarm<MyBehaviour>,
    pub event_sender: mpsc::UnboundedSender<P2PEvent>,
    is_running: bool,
    peers: Vec<Peer>,
    local_peer_id: String,
}

impl RealP2PNode {
    pub async fn new() -> Result<(Self, mpsc::UnboundedReceiver<P2PEvent>), Box<dyn Error>> {
        // Создаем уникальную криптографическую личность для нашей ноды
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        println!("Local peer id: {}", local_peer_id);

        // Создаем транспортный уровень (TCP)
        let transport = libp2p::tokio_development_transport(local_key)?;

        // Создаем наше сетевое поведение
        let behaviour = MyBehaviour {
            mdns: mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)?,
        };

        // Создаем Swarm - это и есть наша нода
        let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

        // Указываем Swarm слушать входящие соединения
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        let (event_sender, event_receiver) = mpsc::unbounded_channel();

        Ok((RealP2PNode {
            swarm,
            event_sender,
            is_running: false,
            peers: Vec::new(),
            local_peer_id: local_peer_id.to_string(),
        }, event_receiver))
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.is_running = true;
        
        log::info!("P2P node started. Local ID: {}", self.local_peer_id);
        self.event_sender.send(P2PEvent::StatusUpdate {
            status_text: format!("Сеть запущена. Ваш ID: {}", self.local_peer_id),
        })?;

        // Send initial network status
        self.send_network_status_update().await?;

        Ok(())
    }

    async fn send_network_status_update(&mut self) -> Result<(), Box<dyn Error>> {
        let connected_count = self.peers.iter().filter(|p| matches!(p.status, PeerStatus::Connected)).count();
        let discovered_count = self.peers.iter().filter(|p| matches!(p.status, PeerStatus::Discovered)).count();
        
        let status = NetworkStatus {
            total_peers: self.peers.len(),
            connected_peers: connected_count,
            discovered_peers: discovered_count,
            peers: self.peers.clone(),
            local_peer_id: self.local_peer_id.clone(),
        };

        self.event_sender.send(P2PEvent::NetworkStatusUpdate { status })?;
        Ok(())
    }

    async fn add_or_update_peer(&mut self, peer_id: String, status: PeerStatus) -> Result<(), Box<dyn Error>> {
        let now = chrono::Utc::now().format("%H:%M:%S").to_string();
        
        if let Some(peer) = self.peers.iter_mut().find(|p| p.id == peer_id) {
            peer.status = status;
            peer.last_seen = now;
        } else {
            self.peers.push(Peer {
                id: peer_id,
                status,
                last_seen: now,
            });
        }

        self.send_network_status_update().await?;
        Ok(())
    }

    pub async fn run_event_loop(&mut self) -> Result<(), Box<dyn Error>> {
        while self.is_running {
            // Опрашиваем Swarm на предмет новых событий
            match self.swarm.select_next_some().await {
                SwarmEvent::Behaviour(MyEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, _multiaddr) in list {
                        println!("mDNS discovered a new peer: {}", peer_id);
                        
                        // Добавляем пира в наш список
                        self.add_or_update_peer(peer_id.to_string(), PeerStatus::Discovered).await?;
                        
                        self.event_sender.send(P2PEvent::StatusUpdate {
                            status_text: format!("Обнаружен новый участник: {}", peer_id),
                        })?;
                    }
                }
                SwarmEvent::Behaviour(MyEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer_id, _multiaddr) in list {
                        println!("mDNS discover peer has expired: {}", peer_id);
                        
                        // Помечаем пира как отключенного
                        self.add_or_update_peer(peer_id.to_string(), PeerStatus::Disconnected).await?;
                        
                        self.event_sender.send(P2PEvent::StatusUpdate {
                            status_text: format!("Участник {} покинул сеть.", peer_id),
                        })?;
                    }
                }
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Local node is listening on {}", address);
                    self.event_sender.send(P2PEvent::StatusUpdate {
                        status_text: format!("Нода слушает на адресе: {}", address),
                    })?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn get_peer_count(&self) -> usize {
        self.peers.iter().filter(|p| matches!(p.status, PeerStatus::Connected)).count()
    }
}

// Для обратной совместимости
pub type P2PNode = RealP2PNode;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_p2p_node_creation() {
        let result = RealP2PNode::new().await;
        assert!(result.is_ok(), "Failed to create P2P node: {:?}", result.err());
        
        let (node, _receiver) = result.unwrap();
        assert!(!node.local_peer_id.is_empty(), "Local peer ID should not be empty");
        println!("Test passed: Created P2P node with ID: {}", node.local_peer_id);
    }

    #[tokio::test]
    async fn test_p2p_node_start() {
        let (mut node, _receiver) = RealP2PNode::new().await.unwrap();
        let result = node.start().await;
        assert!(result.is_ok(), "Failed to start P2P node: {:?}", result.err());
        println!("Test passed: P2P node started successfully");
    }
} 