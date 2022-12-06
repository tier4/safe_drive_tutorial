use safe_drive::{context::Context, error::DynError, msg::common_interfaces::std_srvs};

fn main() -> Result<(), DynError> {
    // Create a context.
    let ctx = Context::new()?;

    // Create a node.
    let node = ctx.create_node("pubsubsrv_server", None, Default::default())?;

    // Create a server.
    let server = node.create_server::<std_srvs::srv::Empty>("pubsubsrv_service", None)?;

    // Create a selector.
    let mut selector = ctx.create_selector()?;

    selector.add_server(
        server,
        Box::new(move |_msg, _header| std_srvs::srv::EmptyResponse::new().unwrap()),
    );

    loop {
        selector.wait()?; // Spin.
    }
}
