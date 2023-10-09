use aws_sdk_ec2::operation::describe_instances::DescribeInstancesOutput;
use aws_sdk_ec2::types::Filter;

async fn new_client() -> aws_sdk_ec2::Client {
    let config = aws_config::load_from_env().await;
    aws_sdk_ec2::Client::new(&config)
}

pub async fn start_instance(id: &str) -> Result<(), aws_sdk_ec2::Error> {
    let client = new_client().await;
    client.start_instances().instance_ids(id).send().await?;
    println!("Started instance {}", id);

    Ok(())
}

pub async fn stop_instance(id: &str) -> Result<(), aws_sdk_ec2::Error> {
    let client = new_client().await;
    client.stop_instances().instance_ids(id).send().await?;

    println!("Stopped instance {}", id);

    Ok(())
}

#[derive(Display)]
pub enum InstanceStateName {
    #[strum(serialize = "pending")]
    Pending,
    #[strum(serialize = "running")]
    Running,
    #[strum(serialize = "shutting-down")]
    ShuttingDown,
    #[strum(serialize = "terminated")]
    Terminated,
    #[strum(serialize = "stopping")]
    Stopping,
    #[strum(serialize = "stopped")]
    Stopped,
}

pub async fn get_dev_instance(
    instance_name: &str,
    instance_state: InstanceStateName,
) -> Result<DescribeInstancesOutput, aws_sdk_ec2::Error> {
    let client = new_client().await;
    let builder = client.describe_instances().set_filters(Some(vec![
        Filter::builder()
            .name("tag:Name")
            .values(instance_name)
            .build(),
        Filter::builder()
            .name("instance-state-name")
            .values(instance_state.to_string())
            .build(),
    ]));
    Ok(builder.send().await?)
}

pub async fn get_dev_instance_id(
    instance_name: &str,
    instance_state: InstanceStateName,
) -> Option<String> {
    let res = get_dev_instance(&instance_name, instance_state)
        .await
        .ok()?;
    let reservations = res.reservations?;

    if reservations.is_empty() {
        return None;
    }

    reservations[0]
        .instances
        .as_ref()?
        .get(0)?
        .instance_id
        .clone()
        .or_else(|| None)
}
