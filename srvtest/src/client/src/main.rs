use safe_drive::{
    context::Context, error::DynError, logger::Logger, pr_error, pr_info, pr_warn, qos::Profile,
};
use srvmsg::srv::{AddTwoInts, AddTwoInts_Request};
use std::time::Duration;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> Result<(), DynError> {
    // Create a context and a node.
    let ctx = Context::new()?;
    let node = ctx.create_node("client", None, Default::default())?;

    let logger = Logger::new("client");

    // Create a client.
    let mut client = node.create_client::<AddTwoInts>("my_service", Some(Profile::default()))?;

    let mut n = 0;
    loop {
        let mut request = AddTwoInts_Request::new().unwrap();
        request.x = n;
        request.y = n + 1;
        n += 1;

        // Send a request.
        let client_rcv = client.send(&request)?;

        // Receive a request.
        let mut receiver = client_rcv.recv();

        match timeout(Duration::from_secs(1), &mut receiver).await {
            Ok(Ok((c, response, _header))) => {
                pr_info!(logger, "received {:?}", response);
                client = c;
            }
            Ok(Err(e)) => {
                pr_error!(logger, "error: {e}");
                return Err(e);
            }
            Err(elapsed) => {
                pr_warn!(logger, "timeout: {elapsed}");
                client = receiver.give_up();
            }
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
