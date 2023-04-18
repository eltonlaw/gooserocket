use std::error;
use std::fs;
use aws_config;
use aws_sdk_cloudformation as cf;

pub async fn create_stack(stack_name: String, stack_fp: String)
    -> Result<(), Box<dyn error::Error>> {
    let template_body = fs::read_to_string(&stack_fp)?;
    println!("Creating new stack {} from fp {} with body\n{}", stack_name, stack_fp, template_body);

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
