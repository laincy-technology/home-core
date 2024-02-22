#![feature(lazy_cell, variant_count)]
#![allow(clippy::needless_return)]
pub mod accounts;
pub mod configs;
pub mod plugins;

use color_eyre::eyre::{Ok, Result};
use include_dir::{include_dir, Dir};
use sqlx::{migrate::MigrateDatabase, Sqlite};
use std::{env, sync::LazyLock};

const STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");
pub const DB_URL: &str = "sqlite://data.db";

//Sets a unique key used to encrypt data. Stored in the binary, must be set at compile time.
//Suggested to use the official manager program in order to keep this secure and consistent.
//Does not completely prevent against attacks, but implements an extra step for attackers.
pub static UNIQUE_KEY: LazyLock<String> = LazyLock::new(|| env::var("LAINCY_UNIQUE_KEY").unwrap());

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    println!("Initializing Laincy Home Core");

    //Instantiates needed files
    let mut cwd = env::current_exe().unwrap();
    cwd.pop();
    STATIC_DIR.extract(cwd)?;
    println!("Created nessesary files");

    //Create SQLite database
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        init_db()?;
        println!("Created database")
    } else {
        println!("Database already exists");
    }

    let loaded_plugins = plugins::loaded().await?;
    println!("Loaded Plugins: ");
    for p in &loaded_plugins {
        println!("{:?}", p);
    }

    //Detect active plugins
    let active_plugins = plugins::active().await?;
    println!("Loaded Plugins: ");
    for p in &active_plugins {
        println!("{:?}", p);
    }
    Ok(())
}

fn init_db() -> Result<()> {
    Ok(())
}
