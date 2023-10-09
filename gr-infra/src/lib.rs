use aws_sdk_cloudformation::types::Parameter;
use std::path::PathBuf;
use std::process::Command;

pub mod aws;
use aws::cloudformation;
use aws::ec2;

extern crate strum;
#[macro_use]
extern crate strum_macros;

fn git_email_name() -> Result<String, std::io::Error> {
    let output = Command::new("git")
        .args(&["config", "user.email"])
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout)
            .trim()
            .split("@")
            .next()
            .unwrap()
            .to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to retrieve git email",
        ))
    }
}

pub async fn deploy_cf_yaml(stack_name: &str, fp: &str, parameters: Option<Vec<Parameter>>) {
    let pb = PathBuf::from(env!("CARGO_MANIFEST_DIR").to_owned()).join(String::from("../") + fp);
    let result = cloudformation::create_or_update_stack(stack_name, &pb, parameters).await;
    match result {
        Ok(_) => println!("Cloudformation create stack worked successfully"),
        Err(error) => eprintln!("Error: {:?}", error),
    }
}

fn dev_instance_name() -> String {
    format!("dev-instance-{}", git_email_name().unwrap())
}

/// Deploys cf stack that creates an AWS::ImageBuilder::Image instance
/// can be referenced when creating dev instances.
pub async fn deploy_dev_image() {
    deploy_cf_yaml("gr-dev-image", "gr-infra/resources/dev_image.yaml", None).await;
}

/// Deploy miscellaneous shared common infra between all stacks
pub async fn deploy_common_infra() {
    deploy_cf_yaml("gr-dev-image", "gr-infra/resources/common_infra.yaml", None).await;
}

pub async fn dev_instance_cf_deploy() {
    let instance_name = format!("dev-instance-{}", git_email_name().unwrap());
    deploy_cf_yaml(
        &instance_name,
        "gr-infra/resources/dev_instance.yaml",
        Some(vec![cloudformation::parameter(
            "TagInstanceName",
            &instance_name,
        )]),
    )
    .await;
}

pub async fn dev_instance_up() {
    let instance_name = format!("dev-instance-{}", git_email_name().unwrap());
    match ec2::get_dev_instance_id(instance_name.as_str(), ec2::InstanceStateName::Stopped).await {
        Some(instance_id) => {
            let _ = ec2::start_instance(instance_id.as_str()).await;
        }
        None => {}
    };
}

pub async fn dev_instance_down() {
    let instance_name = format!("dev-instance-{}", git_email_name().unwrap());
    match ec2::get_dev_instance_id(instance_name.as_str(), ec2::InstanceStateName::Running).await {
        Some(instance_id) => {
            let _ = ec2::stop_instance(instance_id.as_str()).await;
        }
        None => println!("No dev instance is up matching {}", instance_name),
    }
}

pub async fn dev_instance_rm() {
    let stack_name = dev_instance_name();
    let result = cloudformation::delete_stack(stack_name.as_str()).await;
    match result {
        Ok(_) => println!("Tore down dev instance {} {:?}", stack_name, result),
        Err(error) => eprintln!("Error: {:?}", error),
    }
}
