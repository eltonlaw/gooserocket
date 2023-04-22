use std::path::PathBuf;
use gr_infra::aws::cloudformation;
use tokio::runtime::Runtime;

pub fn deploy_jupyter_notebook() {
    let rt = Runtime::new().unwrap();
    let stack_fp = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../assets/jupyter_notebook.yaml");
    let result = rt.block_on(cloudformation::create_or_update_stack(
            "gr-jupyter-nb-1",
            &stack_fp,
        ));

    match result {
        Ok(_) => println!("Cloudformation create stack worked successfully"),
        Err(error) => eprintln!("Error: {:?}", error),
    }
}

pub fn shutdown() {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(cloudformation::delete_stack("gr-jupyter-nb-1"));
    match result {
        Ok(_) => println!("Tore down jupyter notebook"),
        Err(error) => eprintln!("Error: {:?}", error),
    }
}
