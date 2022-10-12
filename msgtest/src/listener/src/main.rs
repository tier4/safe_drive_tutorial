use safe_drive::{context::Context, error::DynError, logger::Logger, pr_info};

fn main() -> Result<(), DynError> {
    // Create a context.
    let ctx = Context::new()?;

    // Create a node.
    let node = ctx.create_node("listener", None, Default::default())?;

    // Create a subscriber.
    let subscriber = node.create_subscriber::<my_interfaces::msg::MyMsgs>("my_topic", None)?;

    // Create a logger.
    let logger = Logger::new("listener");

    // Create a selector.
    let mut selector = ctx.create_selector()?;

    pr_info!(logger, "listening");

    // Add a callback function.
    selector.add_subscriber(
        subscriber,
        Box::new(move |msg| {
            let msg = &msg.msg1;

            pr_info!(logger, "message: {}", msg.integer_value);

            for msg in msg.five_integers_array.iter() {
                pr_info!(logger, "five_integers_array: {}", msg);
            }

            for msg in msg.unbounded_integer_array.iter() {
                pr_info!(logger, "unbounded_integer_array: {}", msg);
            }

            for msg in msg.up_to_five_integers_array.iter() {
                pr_info!(logger, "up_to_five_integers_array: {}", msg);
            }
        }),
    );

    // Spin.
    loop {
        selector.wait()?;
    }
}
