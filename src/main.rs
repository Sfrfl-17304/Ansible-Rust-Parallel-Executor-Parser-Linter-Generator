use clap::Parser;

mod ast;
mod cli;
mod generator;

use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Gen(args) => {
            let playbook = generator::generate_from_file(args.input.to_str().unwrap()).unwrap();
            generator::output_file_yaml(playbook, args.output.to_str().unwrap()).unwrap();
            println!("Playbook generated successfully");
        }
        Commands::Lint(args) => println!("[lint] playbook={:?}", args.playbook),
        Commands::Run(args) => println!(
            "[run] playbook={:?} inventory={:?} dry_run={}",
            args.playbook, args.inventory, args.dry_run
        ),
        Commands::Watch(args) => println!(
            "[watch] playbook={:?} inventory={:?} dry_run={}",
            args.playbook, args.inventory, args.dry_run
        ),
        Commands::Bench(args) => println!(
            "[bench] playbook={:?} concurrency={}",
            args.playbook, args.concurrency
        ),
    }
}
