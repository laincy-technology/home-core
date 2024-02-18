#![feature(lazy_cell, variant_count)]
pub mod plugins;
pub mod accounts;
mod configs;

use color_eyre::eyre::{Ok, Result};
use include_dir::{include_dir, Dir};
use std::{env, sync::LazyLock };
use sqlx::{migrate::MigrateDatabase, Sqlite};

const STATIC_DIR:Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");
pub const DB_URL: &str = "sqlite://data.db";

//Sets a unique key used to encrypt data. Stored in the binary, must be set at compile time.
//Suggested to use the official manager program in order to keep this secure and consistent.
//Does not completely prevent against attacks, but implements an extra step for attackers.
pub const UNIQUE_KEY: LazyLock<String> = LazyLock::new(|| {env::var("LAINCY_UNIQUE_KEY").unwrap()});

#[tokio::main]
async fn main() -> Result<()> {
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
    
    //Detect loaded plugins
    let loaded_plugins = match plugins::loaded().await {
        Ok(plugins) => plugins,
        Err(error) => panic!("Failed to parse loaded plugins: {:?}", error) 
    };
    println!("Loaded Plugins:");
    for p in loaded_plugins {
        println!("{}", p)
    }

    //Detect active plugins
    let active_plugins = match plugins::active().await {
        Ok(plugins) => plugins,
        Err(error) => panic!("Failed to parse active plugins: {:?}", error) 
    };
    println!("Active Plugins: ");
    for p in active_plugins {
        println!("{}", p)
    }

    Ok(())
}

fn init_db() -> Result<()>{
    Ok(())
}