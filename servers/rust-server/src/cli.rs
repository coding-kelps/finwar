use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

/// Simple CLI for the market server. Defaults to `serve` when no subcommand
/// is provided.
#[derive(Debug, Parser)]
#[command(author, version, about = "Finwar market server CLI")]
pub struct Opts {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Start the HTTP server (default)
    Serve,
    /// Run database migrations using the workspace `migration` member
    Migrate,
}

pub async fn run(db: DatabaseConnection) -> Result<(), crate::error::Error> {
    let opts = Opts::parse();
    match opts.command.unwrap_or(Command::Serve) {
        Command::Serve => crate::run_server(db).await,
        Command::Migrate => {
            dotenv().ok();
            Migrator::up(&db, None).await?;
            Ok(())
        },
    }
}
