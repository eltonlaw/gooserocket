use clap::{Arg, ArgAction, command, Command};
use gr_engine;
use gr_local_dev;

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
            Command::new("local")
                .about("Local development")
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
            "jupyter-image" => gr_engine::deploy_jupyter_image(),
            "common-infra" => gr_engine::deploy_common_infra(),
            _ => unreachable!("Exhausted list of deploy subcommands"),
        },
        Some(("local", submatches)) => match submatches.get_one::<String>("target").unwrap().as_str() {
            "start-jupyter" => gr_local_dev::start_jupyter_notebook(),
            _ => unreachable!("Exhausted list of local subcommands"),
        },
        Some(("shutdown", _sub_matches)) => gr_engine::shutdown(),
        _ => unreachable!("Exhaused list of subcommands"),
    }
}
