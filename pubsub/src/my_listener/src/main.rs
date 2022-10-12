use safe_drive::{context::Context, error::DynError, logger::Logger, pr_info};

fn main() -> Result<(), DynError> {
    // Create a context.
    let ctx = Context::new()?;

    // Create a node.
    let node = ctx.create_node("my_listener", None, Default::default())?;

    // Create a subscriber.
    let subscriber = node.create_subscriber::<std_msgs::msg::String>("my_topic", None)?;

    // Create a logger.
    let logger = Logger::new("my_listener");

    // Create a selector.
    let mut selector = ctx.create_selector()?;

    // Add a callback function.
    selector.add_subscriber(
        subscriber,
        Box::new(move |msg| {
            pr_info!(logger, "receive: {}", msg.data);
        }),
    );

    // Spin.
    loop {
        selector.wait()?;
    }
}
