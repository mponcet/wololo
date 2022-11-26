extern crate serde;

use clap::{arg, command, Command};

use crate::repository::DeviceRepository;
use crate::repository::{file::FileRepository, inmemory::InMemoryDeviceRepository};

mod cli;
mod device;
mod repository;
mod wol;

fn main() {
    if let Ok(repo) = build_repository("file") {
        let matches = command!()
            .propagate_version(true)
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                Command::new("cli")
                    .about("cli mode")
                    .subcommand_required(true)
                    .subcommands([
                        Command::new("add")
                            .about("add a device")
                            .arg_required_else_help(true)
                            .args([arg!([NAME]).required(true), arg!([MAC]).required(true)]),
                        Command::new("del")
                            .about("delete a device")
                            .arg_required_else_help(true)
                            .arg(arg!([NAME])),
                        Command::new("wake")
                            .about("wake device")
                            .arg_required_else_help(true)
                            .arg(arg!([MAC])),
                        Command::new("show").about("show all devices"),
                    ]),
            )
            .get_matches();

        match matches.subcommand() {
            Some(("cli", cli_submatches)) => match cli_submatches.subcommand() {
                Some(("add", args)) => cli::add_device(
                    &*repo,
                    args.get_one::<String>("NAME").unwrap(),
                    args.get_one::<String>("MAC").unwrap(),
                ),
                Some(("del", args)) => {
                    cli::delete_device(&*repo, args.get_one::<String>("NAME").unwrap())
                }
                Some(("wake", args)) => cli::wake_device(args.get_one::<String>("MAC").unwrap()),
                Some(("show", _)) => cli::show_devices(&*repo),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

fn build_repository(kind: &str) -> Result<Box<dyn DeviceRepository>, ()> {
    match kind {
        "file" => match FileRepository::try_new("devices.yml") {
            Ok(repo) => Ok(Box::new(repo)),
            Err(_) => {
                eprintln!("failed to create file repository");
                Err(())
            }
        },
        "memory" => Ok(Box::new(InMemoryDeviceRepository::new())),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests;
