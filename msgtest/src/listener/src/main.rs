use my_interfaces_rs::my_interfaces;
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

            pr_info!(logger, "message: {}", msg.message);

            for msg in msg.static_array_str.iter() {
                pr_info!(logger, "static_array_str: {}", msg);
            }

            if let Some(slice) = msg.dynamic_array_str.as_slice() {
                for msg in slice {
                    pr_info!(logger, "dynamic_array_str: {}", msg);
                }
            }
        }),
        false,
    );

    // Spin.
    loop {
        selector.wait()?;
    }
}
