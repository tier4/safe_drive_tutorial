use safe_drive::{
    context::Context, error::DynError, logger::Logger, pr_info, topic::subscriber::Subscriber,
};

#[async_std::main]
async fn main() -> Result<(), DynError> {
    // Create a context and a node.
    let ctx = Context::new()?;
    let node = ctx.create_node("subscribers", None, Default::default())?;

    // Create subscribers.
    let subscriber1 = node.create_subscriber::<std_msgs::msg::String>("topic1", None)?;
    let subscriber2 = node.create_subscriber::<std_msgs::msg::String>("topic2", None)?;

    // Receive messages.
    let task1 = async_std::task::spawn(receiver(subscriber1));
    let task2 = async_std::task::spawn(receiver(subscriber2));

    task1.await?;
    task2.await?;

    Ok(())
}

async fn _receiver(mut subscriber: Subscriber<std_msgs::msg::String>) -> Result<(), DynError> {
    let logger = Logger::new(subscriber.get_topic_name());

    loop {
        // Receive a message
        let msg = subscriber.recv().await?;
        pr_info!(logger, "{}", msg.data);
    }
}

use async_std::future::timeout;
use safe_drive::pr_warn;
use std::time::Duration;
async fn receiver(mut subscriber: Subscriber<std_msgs::msg::String>) -> Result<(), DynError> {
    let logger = Logger::new(subscriber.get_topic_name());

    loop {
        // Receive a message with 3s timeout.
        match timeout(Duration::from_secs(3), subscriber.recv()).await {
            Ok(result) => pr_info!(logger, "received: {}", result?.data),
            Err(_) => {
                // Timed out. Finish receiving.
                pr_warn!(logger, "timeout");
                break;
            }
        }
    }

    Ok(())
}
