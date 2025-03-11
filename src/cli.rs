use clap::{Parser, Arg};
use std::env;

pub const ENVIRONMENT_VARIABLE: &str = "TEXPENSE_DATABASE_URL";

/// Struct representing CLI arguments using derive functionality.
#[derive(Parser)]
#[command(
    name = "Texpense",
    version = "0.1.0",
    author = "jisef",
    about = "A simple TUI tool to track your expenses"
)]
struct Cli {
    /// The database URL
    #[arg(short, long, value_name = "URL", help = "Returns or sets the database-url")]
    database: Option<String>,
}

/// Function checks if a database-url is configured
pub fn start_cli() -> Result<(), String> {
    let cli: Cli = Cli::parse();

    match cli.database {
        Some(url) => {
            env::set_var(ENVIRONMENT_VARIABLE, &url);
            println!("Database URL set to: {}", url);
            println!("Environment variable Name is: {}", ENVIRONMENT_VARIABLE);
            Ok(())
        }
        None => {
            let var = env::var(ENVIRONMENT_VARIABLE);
            if var.is_ok() {
                println!("Database URL set to: {}", var.unwrap());
                Ok(())
            } else {
                Err("Environment variable is not set! \ntexpense --database <url>".to_string())
            }
        }
    }
}