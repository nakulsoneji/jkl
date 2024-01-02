use crate::cli::command::{Cli, Commands};
use crate::{
    db::actions::{self},
    packages,
};
use anyhow::Result;
use clap::Parser;

pub async fn parse_commands() -> Result<()> {
    let cli = Cli::parse();

    if let Commands::Init = &cli.command {
        ()
    } else if !actions::db_dirs_exist().await? {
        println!();
    }

    match &cli.command {
        Commands::Init => actions::db_init().await?,
        Commands::List => actions::print_db().await?,
        Commands::Install(args) => packages::install(&args.package).await?,
        Commands::Update(args) => packages::update(&args.package).await?,
        Commands::Delete(args) => packages::delete(&args.package).await?,
    }
    Ok(())
}
