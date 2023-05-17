use clap::{Arg, ArgAction, command, Command};
use gr_engine;

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("deploy")
                .about("Deploy a new service")
                .arg(Arg::new("target")
                     .help("deploy target")
                     .action(ArgAction::Set)
                     .required(true))
        )
        .subcommand(
            Command::new("shutdown")
                .about("Stop all services")
        )
        .get_matches();
    match matches.subcommand() {
        Some(("deploy", submatches)) => match submatches.get_one::<String>("target").unwrap().as_str() {
            "jupyter-notebook" => gr_engine::deploy_jupyter_notebook(),
            "jupyter-image-pipeline" => gr_engine::deploy_jupyter_image_pipeline(),
            "common-infra" => gr_engine::deploy_common_infra(),
            _ => unreachable!("Exhausted list of deploy subcommands"),
        },
        Some(("shutdown", _sub_matches)) => gr_engine::shutdown(),
        _ => unreachable!("Exhaused list of subcommands"),
    }
}
