use clap::Parser;

mod ast;
mod cli;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gen(args) => println!("[gen] input={:?} output={:?}", args.input, args.output),
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
