use safe_drive::{context::Context, error::DynError, logger::Logger, parameter::Value, pr_info};

fn main() -> Result<(), DynError> {
    // Create a context and a node.
    let ctx = Context::new()?;
    let node = ctx.create_node("param_server", None, Default::default())?;

    // Create a parameter server.
    let param_server = node.create_parameter_server()?;
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

    // Create a logger and a selector.
    let logger = Logger::new("param_server");
    let mut selector = ctx.create_selector()?;

    // Add a callback function to the parameter server.
    selector.add_parameter_server(
        param_server,
        Box::new(move |params, updated| {
            // Print updated parameters.
            let mut keys = String::new();
            for key in updated.iter() {
                let value = &params.get_parameter(key).unwrap().value;
                keys = format!("{keys}{key} = {}, ", value);
            }
            pr_info!(logger, "updated parameters: {keys}");
        }),
    );

    // Spin.
    loop {
        selector.wait()?;
    }
}
