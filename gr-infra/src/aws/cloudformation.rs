use std::error;
use std::env;
use std::fs;
use std::path::PathBuf;
use aws_config;
use aws_sdk_cloudformation as cf;

pub async fn create_stack(stack_name: String, stack_fp: String)
    -> Result<(), Box<dyn error::Error>> {
    let mut stack_abs_fp = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    stack_abs_fp.push(stack_fp.as_str());
    let template_body = fs::read_to_string(&stack_abs_fp)?;
    println!("Creating new stack {} from fp {:?} with body\n{}", stack_name, stack_abs_fp, template_body);

    let config = aws_config::from_env().load().await;
    let client = cf::Client::new(&config);

    client
        .create_stack()
        .stack_name(stack_name)
        .template_body(template_body)
        .send()
        .await?;

    Ok(())
}
