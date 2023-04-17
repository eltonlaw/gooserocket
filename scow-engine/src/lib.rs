use scow_infra::aws::cloudformation;
use tokio::runtime::Runtime;

pub fn deploy_jupyter_notebook() {
    let rt = Runtime::new().unwrap();
    rt.block_on(cloudformation::create_stack());
}
