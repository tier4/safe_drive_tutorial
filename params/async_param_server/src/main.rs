use safe_drive::{
    context::Context,
    error::DynError,
    logger::Logger,
    parameter::{ParameterServer, Value},
    pr_info,
};

#[tokio::main]
async fn main() -> Result<(), DynError> {
    // Create a context and a node.
    let ctx = Context::new()?;
    let node = ctx.create_node("async_param_server", None, Default::default())?;

    // Create a parameter server.
    let mut param_server = ParameterServer::new(node)?;
    {
        // Set parameters.
        let mut params = param_server.params.write(); // Write lock

        // Statically typed parameter.
        params.set_parameter(
            "my_flag".to_string(),                     // parameter name
            Value::Bool(false),                        // value
            false,                                     // read only?
            Some("my flag's description".to_string()), // description
        )?;

        // Dynamically typed parameter.
        params.set_dynamically_typed_parameter(
            "my_dynamic_type_flag".to_string(), // parameter name
            Value::Bool(false),                 // value
            false,                              // read only?
            Some("my dynamic type flag's description".to_string()), // description
        )?;
    }

    // Create a logger.
    let logger = Logger::new("async_param_server");

    loop {
        // Wait update asynchronously.
        let updated = param_server.wait().await?;

        let params = param_server.params.read(); // Read lock

        // Print updated parameters.
        let mut keys = String::new();
        for key in updated.iter() {
            let value = &params.get_parameter(key).unwrap().value;
            keys = format!("{keys}{key} = {}, ", value);
        }
        pr_info!(logger, "updated parameters: {keys}");
    }
}
