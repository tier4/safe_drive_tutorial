use safe_drive::{
    context::Context, error::DynError, logger::Logger, msg::common_interfaces::std_msgs, pr_info,
};
use std::{rc::Rc, time::Duration};

fn main() -> Result<(), DynError> {
    // Create a context, a node, a subscriber, a publisher, and a selector.
    let ctx = Context::new()?;
    let node = ctx.create_node("my_node", None, Default::default())?;
    let subscriber = node.create_subscriber::<std_msgs::msg::UInt64>("my_topic", None)?;
    let publisher = node.create_publisher::<std_msgs::msg::UInt64>("my_topic", None)?;
    let mut selector = ctx.create_selector()?;

    // Create a logger.
    // To share this by multiple callback functions, use Rc.
    let logger = Rc::new(Logger::new("wall timer example"));

    // Add a wall timer to publish periodically.
    let mut cnt = Box::new(0);
    let mut msg = std_msgs::msg::UInt64::new().unwrap();
    let logger1 = logger.clone();

    selector.add_wall_timer(
        "publisher", // the name of timer
        Duration::from_secs(1),
        Box::new(move || {
            msg.data = *cnt;
            *cnt += 1;
            publisher.send(&msg).unwrap();
            pr_info!(logger1, "send: msg.data = {}", msg.data);
        }),
    );

    // Add a subscriber.
    selector.add_subscriber(
        subscriber,
        Box::new(move |msg| {
            pr_info!(logger, "received: msg.data = {}", msg.data);
        }),
    );

    // Spin.
    loop {
        selector.wait()?;
    }
}
