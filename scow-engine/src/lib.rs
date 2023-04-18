use scow_infra::aws::cloudformation;
use tokio::runtime::Runtime;

pub fn deploy_jupyter_notebook() {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(cloudformation::create_stack(
            String::from("scow-jupyter-nb-1"),
            String::from("./assets/jupyter_notebook.yaml")
        ));

    match result {
        Ok(_) => println!("Cloudformation create stack worked successfully"),
        Err(error) => eprintln!("Error: {}", error),
    }
}
