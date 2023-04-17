use aws_config;
use aws_sdk_cloudformation as cf;

pub async fn create_stack() -> Result<(), cloudformation::Error> {
    println!("Creating new stack");
    let config = aws_config::from_env().load().await;
    let client = cf::Client::new(&config);

    Ok(())
}
