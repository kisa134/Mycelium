use tokio::sync::mpsc;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub enum P2PEvent {
    PeerConnected { peer_id: String },
    PeerDisconnected { peer_id: String },
    StatusUpdate { status_text: String },
    PeerCount { count: usize },
}

pub struct P2PNode {
    event_sender: mpsc::UnboundedSender<P2PEvent>,
    is_running: bool,
}

impl P2PNode {
    pub async fn new() -> Result<(Self, mpsc::UnboundedReceiver<P2PEvent>), Box<dyn std::error::Error>> {
        let (event_sender, event_receiver) = mpsc::unbounded_channel();
        
        Ok((P2PNode { 
            event_sender, 
            is_running: false 
        }, event_receiver))
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.is_running = true;
        
        log::info!("P2P node started (simulated)");
        self.event_sender.send(P2PEvent::StatusUpdate {
            status_text: "P2P node started (simulated)".to_string(),
        })?;

        Ok(())
    }

    pub async fn run_event_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut peer_count = 0;
        
        while self.is_running {
            // Simulate peer discovery
            if peer_count < 5 {
                sleep(Duration::from_secs(2)).await;
                peer_count += 1;
                
                let peer_id = format!("12D3KooW{}", peer_count);
                self.event_sender.send(P2PEvent::PeerConnected {
                    peer_id: peer_id.clone(),
                })?;
                
                self.event_sender.send(P2PEvent::StatusUpdate {
                    status_text: format!("Discovered peer: {}", peer_id),
                })?;
                
                self.event_sender.send(P2PEvent::PeerCount {
                    count: peer_count,
                })?;
            }
            
            sleep(Duration::from_secs(5)).await;
        }

        Ok(())
    }

    pub fn get_peer_count(&self) -> usize {
        0
    }
} 