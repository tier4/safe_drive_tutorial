use safe_drive::{context::Context, error::DynError, logger::Logger, pr_info};
use std::time::Duration;

fn main() -> Result<(), DynError> {
    // Create a context.
    let ctx = Context::new()?;

    // Create a node.
    let node = ctx.create_node("my_talker", None, Default::default())?;

    // Create a publisher.
    let publisher = node.create_publisher::<std_msgs::msg::String>("my_topic", None, true)?;

    // Create a logger.
    let logger = Logger::new("my_talker");

    let mut cnt = 0; // Counter.
    let mut msg = std_msgs::msg::String::new().unwrap();
    loop {
        // Create a message to be sent.
        let data = format!("Hello, World!: cnt = {cnt}");
        msg.data.assign(&data);

        pr_info!(logger, "send: {}", msg.data); // Print log.

        // Send a message.
        publisher.send(&msg)?;

        // Sleep 1s.
        cnt += 1;
        std::thread::sleep(Duration::from_secs(1));
    }
}
