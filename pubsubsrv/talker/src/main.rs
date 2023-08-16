use safe_drive::{context::Context, error::DynError, msg::common_interfaces::std_msgs};
use std::time::Duration;

fn main() -> Result<(), DynError> {
    // Create a context.
    let ctx = Context::new()?;

    // Create a node.
    let node = ctx.create_node("pubsubsrv_talker", None, Default::default())?;

    // Create a publisher.
    let publisher = node.create_publisher::<std_msgs::msg::Empty>("pubsubsrv_topic", None, true)?;

    let msg = std_msgs::msg::Empty::new().unwrap();
    loop {
        // Send a message.
        publisher.send(&msg)?;

        // Sleep 1 sec.
        std::thread::sleep(Duration::from_secs(1));
    }
}
