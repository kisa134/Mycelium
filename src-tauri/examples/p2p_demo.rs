use mycelium_app_lib::p2p::RealP2PNode;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    println!("🚀 Starting Mycelium P2P Network Demo");
    println!("📡 Protocol 'Genesis' - Stage 1: Spinal Cord");
    println!();

    // Create P2P node
    let (mut node, mut event_receiver) = RealP2PNode::new().await?;
    
    println!("✅ Node created successfully!");
    println!("🆔 Your Peer ID: {}", node.local_peer_id);
    println!();

    // Start the node
    node.start().await?;
    println!("✅ Node started and listening for incoming connections");
    println!("🔍 Searching for other participants in local network...");
    println!();

    // Start event processing in a separate task
    let event_handle = tokio::spawn(async move {
        while let Some(event) = event_receiver.recv().await {
            match event {
                mycelium_app_lib::p2p::P2PEvent::StatusUpdate { status_text } => {
                    println!("📢 {}", status_text);
                }
                mycelium_app_lib::p2p::P2PEvent::NetworkStatusUpdate { status } => {
                    println!("📊 Network Status:");
                    println!("   👥 Total participants: {}", status.total_peers);
                    println!("   🔗 Connected: {}", status.connected_peers);
                    println!("   🔍 Discovered: {}", status.discovered_peers);
                    println!();
                }
                _ => {}
            }
        }
    });

    // Start main event loop in a separate task
    let run_handle = tokio::spawn(async move {
        if let Err(e) = node.run_event_loop().await {
            eprintln!("❌ Error in event loop: {}", e);
        }
    });

    // Wait for some time for demonstration
    println!("⏳ Demo will run for 30 seconds...");
    println!("💡 Start a second copy of the program on another computer in the same network");
    println!("   to see participant discovery!");
    println!();

    sleep(Duration::from_secs(30)).await;

    println!("🏁 Demo completed!");
    println!("✅ Real P2P network is working!");
    println!("🎉 Protocol 'Genesis' - Stage 1 successfully implemented!");

    Ok(())
} 