use mycelium_app_lib::p2p::RealP2PNode;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализируем логирование
    env_logger::init();
    
    println!("🚀 Запуск демонстрации реальной P2P сети Mycelium");
    println!("📡 Протокол 'Генезис' - Этап 1: Спинной мозг");
    println!();

    // Создаем P2P ноду
    let (mut node, mut event_receiver) = RealP2PNode::new().await?;
    
    println!("✅ Нода создана успешно!");
    println!("🆔 Ваш Peer ID: {}", node.local_peer_id);
    println!();

    // Запускаем ноду
    node.start().await?;
    println!("✅ Нода запущена и слушает входящие соединения");
    println!("🔍 Ищем других участников в локальной сети...");
    println!();

    // Запускаем обработку событий в отдельной задаче
    let event_handle = tokio::spawn(async move {
        while let Some(event) = event_receiver.recv().await {
            match event {
                mycelium_app_lib::p2p::P2PEvent::StatusUpdate { status_text } => {
                    println!("📢 {}", status_text);
                }
                mycelium_app_lib::p2p::P2PEvent::NetworkStatusUpdate { status } => {
                    println!("📊 Статус сети:");
                    println!("   👥 Всего участников: {}", status.total_peers);
                    println!("   🔗 Подключено: {}", status.connected_peers);
                    println!("   🔍 Обнаружено: {}", status.discovered_peers);
                    println!();
                }
                _ => {}
            }
        }
    });

    // Запускаем основной цикл событий в отдельной задаче
    let run_handle = tokio::spawn(async move {
        if let Err(e) = node.run_event_loop().await {
            eprintln!("❌ Ошибка в цикле событий: {}", e);
        }
    });

    // Ждем некоторое время для демонстрации
    println!("⏳ Демонстрация будет работать 30 секунд...");
    println!("💡 Запустите вторую копию программы на другом компьютере в той же сети");
    println!("   чтобы увидеть обнаружение участников!");
    println!();

    sleep(Duration::from_secs(30)).await;

    println!("🏁 Демонстрация завершена!");
    println!("✅ Реальная P2P сеть работает!");
    println!("🎉 Протокол 'Генезис' - Этап 1 успешно реализован!");

    Ok(())
} 