use clap::{command, Command};
use scow_engine;

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("deploy-jupyter-notebook")
                .about("Creates a new ec2 instance with jupyter notebook running and deployed at port 8080")
        )
        .get_matches();
    match matches.subcommand() {
        Some(("deploy-jupyter-notebook", _sub_matches)) => scow_engine::deploy_jupyter_notebook(),
        _ => unreachable!("Exhaused list of subcommands"),
    }
}
