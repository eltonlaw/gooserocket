use aws_config;
use aws_sdk_cloudformation as cf;
use aws_sdk_cloudformation::types::{Capability, Parameter};
use std::error;
use std::fs;
use std::path::PathBuf;

pub fn parameter<S: Into<String>>(pkey: S, pval: S) -> Parameter {
    Parameter::builder()
        .parameter_key(pkey)
        .parameter_value(pval)
        .build()
}

pub async fn new_client() -> cf::Client {
    let config = aws_config::from_env().load().await;
    cf::Client::new(&config)
}

pub async fn is_stack_existing(stack_name: &str) -> bool {
    let client = new_client().await;
    match client.describe_stacks().stack_name(stack_name).send().await {
        Ok(res) => {
            println!("Got stacks: {:?}", res);
            true
        }
        Err(_) => {
            println!("Couldn't find stack");
            false
        }
    }
}

pub async fn create_stack(
    stack_name: &str,
    stack_fp: &PathBuf,
    parameters: Option<Vec<Parameter>>,
) -> Result<(), Box<dyn error::Error>> {
    let template_body = fs::read_to_string(stack_fp)?;
    let client = new_client().await;
    let mut builder = client
        .create_stack()
        .stack_name(stack_name)
        .template_body(template_body)
        .capabilities(Capability::CapabilityIam);
    if let Some(params) = parameters {
        builder = builder.set_parameters(Some(params));
    }
    let res = builder.send().await?;
    println!("create_stack res: {:?}", res);
    Ok(())
}

pub async fn update_stack(
    stack_name: &str,
    stack_fp: &PathBuf,
    parameters: Option<Vec<Parameter>>,
) -> Result<(), Box<dyn error::Error>> {
    let template_body = fs::read_to_string(stack_fp)?;
    let client = new_client().await;
    let mut builder = client
        .update_stack()
        .stack_name(stack_name)
        .template_body(template_body)
        .capabilities(Capability::CapabilityIam);
    if let Some(params) = parameters {
        builder = builder.set_parameters(Some(params));
    }
    let res = builder.send().await?;
    println!("update_stack res: {:?}", res);
    Ok(())
}

pub async fn delete_stack(stack_name: &str) -> Result<(), Box<dyn error::Error>> {
    let client = new_client().await;
    let res = client.delete_stack().stack_name(stack_name).send().await?;
    println!("delete_stack res: {:?}", res);
    Ok(())
}

/// Create the stack if it doesn't exist, otherwise recreate it.
pub async fn create_or_update_stack(
    stack_name: &str,
    stack_fp: &PathBuf,
    parameters: Option<Vec<Parameter>>,
) -> Result<(), Box<dyn error::Error>> {
    if !is_stack_existing(stack_name).await {
        println!(
            "Creating new stack {} from fp {} with body",
            stack_name,
            stack_fp.to_string_lossy()
        );
        create_stack(stack_name, &stack_fp, parameters).await?;
    } else {
        println!(
            "Stack {} already exists. Running update_stack instead with {}",
            stack_name,
            stack_fp.to_string_lossy()
        );
        update_stack(stack_name, &stack_fp, parameters).await?;
    }
    Ok(())
}
