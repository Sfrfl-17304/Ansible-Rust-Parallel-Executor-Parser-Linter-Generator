use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "ansc", about = "Ansible Automation Control Center")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Gen(GenArgs),
    Lint(LintArgs),
    Run(RunArgs),
    Watch(WatchArgs),
    Bench(BenchArgs),
}

#[derive(Args, Debug)]
pub struct GenArgs {
    #[arg(short, long)]
    pub input: PathBuf,

    #[arg(short, long)]
    pub output: PathBuf,
}

#[derive(Args, Debug)]
pub struct LintArgs {
    #[arg(short, long)]
    pub playbook: PathBuf,
}

#[derive(Args, Debug)]
pub struct RunArgs {
    #[arg(short, long)]
    pub playbook: PathBuf,

    #[arg(short, long)]
    pub inventory: PathBuf,

    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Args, Debug)]
pub struct WatchArgs {
    #[arg(short, long)]
    pub playbook: PathBuf,

    #[arg(short, long)]
    pub inventory: PathBuf,

    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Args, Debug)]
pub struct BenchArgs {
    #[arg(short, long)]
    pub playbook: PathBuf,

    #[arg(short, long, default_value = "4")]
    pub concurrency: u32,
}
