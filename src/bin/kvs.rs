use clap::{Arg, App, SubCommand};
use std::process::exit;

fn main() {
    const VERSION: &str = "0.1";

    let matches = App::new("kvs inMemory utility") 
                    .version(VERSION)
                    .author("Rafael Mendes, based on PingCAP talent-plan")
                    .about("Stores and queries values on memory")
                    .subcommand(SubCommand::with_name("V")
                        .about("Print version of kvs")
                        .version(VERSION))
                    .subcommand(SubCommand::with_name("get")
                        .about("Read data from memory based on key")
                        .version(VERSION)
                        .arg(Arg::with_name("key")
                            .help("Key to lookup on memstore")
                            .required(true)))
                    .subcommand(SubCommand::with_name("set")
                        .about("Write data into memory store")
                        .version(VERSION)
                        .arg(Arg::with_name("key")
                            .help("Key to persist into memstore")
                            .required(true))
                        .arg(Arg::with_name("value")
                            .help("Value to persist data into memstore")
                            .required(true)))
                    .subcommand(SubCommand::with_name("rm")
                        .about("Remove data from memstore")
                        .version(VERSION)
                        .arg(Arg::with_name("key")
                            .help("Key to remove data from memstore")
                            .required(true))).get_matches();

    match matches.subcommand() {
        ("get", Some(_matches)) => {
            eprintln!("unimplemented!");
            exit(1);
        }
        ("set", Some(_matches)) => {
            eprintln!("unimplemented!");
            exit(1);
        }        
        ("rm", Some(_matches)) => {
            eprintln!("unimplemented!");
            exit(1);
        }
        _ => unreachable!(),
    }
        
}