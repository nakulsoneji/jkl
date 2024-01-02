use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    Init,
    List,
    Install(PackageArgs),
    Update(PackageArgs),
    Delete(PackageArgs),
}

#[derive(Args)]
pub(crate) struct PackageArgs {
    pub(crate) package: String,
}
