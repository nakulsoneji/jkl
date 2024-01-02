use anyhow::Result;
use jkl::cli::parse;

// TODO: Clean up debugging stuff everywhere
// TODO: Change visibility (pub->pub(crate) or private)
//
// NOTE: use "-q" flag or redirection for wget, as it logs progress to stderr

#[tokio::main]
async fn main() -> Result<()> {
    parse::parse_commands().await?;
    // actions::db_init().await?;
    // install("bun").await?;
    // actions::print_db().await?;

    Ok(())
}
