use clap::{command, Arg, ArgAction, Command};
use gr_infra;
use gr_local_dev;
use tokio::runtime::Runtime;

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("deploy").about("Deploy a new service").arg(
                Arg::new("target")
                    .help("deploy target")
                    .action(ArgAction::Set)
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("local").about("Local development").arg(
                Arg::new("target")
                    .help("deploy target")
                    .action(ArgAction::Set)
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("dev-instance")
                .about("Manage a dev instance")
                .arg(
                    Arg::new("subcmd")
                        .help("")
                        .action(ArgAction::Set)
                        .required(true),
                ),
        )
        .get_matches();
    let rt = Runtime::new().unwrap();
    match matches.subcommand() {
        Some(("deploy", submatches)) => {
            match submatches.get_one::<String>("target").unwrap().as_str() {
                "dev-image" => rt.block_on(gr_infra::deploy_dev_image()),
                "common-infra" => rt.block_on(gr_infra::deploy_common_infra()),
                _ => unreachable!("Exhausted list of deploy subcommands"),
            }
        }
        Some(("dev-instance", submatches)) => {
            match submatches.get_one::<String>("subcmd").unwrap().as_str() {
                "new" => rt.block_on(gr_infra::dev_instance_cf_deploy()),
                "rm" => rt.block_on(gr_infra::dev_instance_rm()),
                "up" => rt.block_on(gr_infra::dev_instance_up()),
                "down" => rt.block_on(gr_infra::dev_instance_down()),
                _ => unreachable!("Exhausted list of dev-instance subcommands"),
            }
        }
        Some(("local", submatches)) => {
            match submatches.get_one::<String>("target").unwrap().as_str() {
                "start-jupyter" => gr_local_dev::start_jupyter_notebook(),
                _ => unreachable!("Exhausted list of local subcommands"),
            }
        }
        _ => unreachable!("Exhaused list of subcommands"),
    }
}
