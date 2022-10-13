use safe_drive::{context::Context, error::DynError, logger::Logger, pr_error, qos::Profile};
use srvmsg::srv::{AddTwoInts, AddTwoIntsResponse};

fn main() -> Result<(), DynError> {
    // Create a context.
    let ctx = Context::new()?;

    // Create a node.
    let node = ctx.create_node("server_node", None, Default::default())?;

    // Create a server.
    let server = node.create_server::<AddTwoInts>("my_service", Some(Profile::default()))?;

    // Create a selector.
    let mut selector = ctx.create_selector()?;

    // Create a logger.
    let logger = Logger::new("server");

    selector.add_server(
        server,
        Box::new(move |msg, _header| {
            let mut response = AddTwoIntsResponse::new().unwrap();
            pr_error!(logger, "recv: {:?}", msg);
            response.result = msg.x + msg.y;
            response
        }),
    );

    loop {
        selector.wait()?; // Spin.
    }
}
