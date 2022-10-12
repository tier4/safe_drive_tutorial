use safe_drive::{context::Context, error::DynError};
use std::time::Duration;

#[async_std::main]
async fn main() -> Result<(), DynError> {
    // Create a context and a node.
    let ctx = Context::new()?;
    let node = ctx.create_node("publishers", None, Default::default())?;

    // Create publishers.
    let publisher1 = node.create_publisher::<std_msgs::msg::String>("topic1", None)?;
    let publisher2 = node.create_publisher::<std_msgs::msg::String>("topic2", None)?;

    // Create a task which sends "Hello, World!".
    let task1 = async_std::task::spawn(async move {
        let mut msg = std_msgs::msg::String::new().unwrap();
        msg.data.assign("Hello, World!");
        for _ in 0..50 {
            publisher1.send(&msg).unwrap(); // Send a message.
            async_std::task::sleep(Duration::from_millis(500)).await; // Sleep 500ms.
        }
    });

    // Create a task which sends "Hello, Universe!".
    let task2 = async_std::task::spawn(async move {
        let mut msg = std_msgs::msg::String::new().unwrap();
        msg.data.assign("Hello, Universe!");
        for _ in 0..100 {
            publisher2.send(&msg).unwrap(); // Send a message.
            async_std::task::sleep(Duration::from_millis(250)).await; // Sleep 250ms.
        }
    });

    task1.await;
    task2.await;

    Ok(())
}
