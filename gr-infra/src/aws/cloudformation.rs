use std::error;
use std::env;
use std::fs;
use std::path::PathBuf;
use aws_config;
use aws_sdk_cloudformation as cf;

pub async fn new_client() -> cf::Client {
    let config = aws_config::from_env().load().await;
    cf::Client::new(&config)
}

pub async fn is_stack_existing(stack_name: &str) -> bool  {
    let client = new_client().await;

    match client
        .describe_stacks()
        .stack_name(stack_name)
        .send()
        .await {
        Ok(res) => {
            println!("Got stacks: {:?}", res);
            true
        },
        Err(_) => {
            println!("Couldn't find stack");
            false
        }
    }
}

pub async fn create_stack(stack_name: &str, stack_fp: &str)
    -> Result<(), Box<dyn error::Error>> {
    let mut stack_abs_fp = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    stack_abs_fp.push(stack_fp);
    let template_body = fs::read_to_string(&stack_abs_fp)?;

    let client = new_client().await;

    if !is_stack_existing(stack_name).await {
        println!("Creating new stack {} from fp {:?} with body\n{}", stack_name, stack_abs_fp, template_body);
        client
            .create_stack()
            .stack_name(stack_name)
            .template_body(template_body)
            .send()
            .await?;
    } else {
        println!("Stack already exists. Skipping stack creation");
    }

    Ok(())
}
