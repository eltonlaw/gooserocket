use std::path::PathBuf;
use gr_infra::aws::cloudformation;
use tokio::runtime::Runtime;

fn deploy_cf_yaml(stack_name: &str, pb: PathBuf) {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(
        cloudformation::create_or_update_stack(stack_name, &pb));
    match result {
        Ok(_) => println!("Cloudformation create stack worked successfully"),
        Err(error) => eprintln!("Error: {:?}", error),
    }
}

pub fn deploy_jupyter_notebook() {
    let pb = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../assets/jupyter_notebook.yaml");
    deploy_cf_yaml("gr-jupyter-nb-1", pb);
}

pub fn deploy_common_infra() {
    let pb = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../assets/common_infra.yaml");
    deploy_cf_yaml("gr-common-infra", pb);
}

pub fn shutdown() {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(cloudformation::delete_stack("gr-jupyter-nb-1"));
    match result {
        Ok(_) => println!("Tore down jupyter notebook"),
        Err(error) => eprintln!("Error: {:?}", error),
    }
}
