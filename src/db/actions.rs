use super::types::PData;
use crate::globals;
use anyhow::{anyhow, Result};
use sqlx::migrate::MigrateDatabase;
use sqlx::{Connection, Sqlite, SqliteConnection};
use std::path::PathBuf;

pub async fn db_dirs_exist() -> Result<bool> {
    let app_dir_e = PathBuf::from(globals::APP_DIR.to_string()).exists();
    let app_bin_dir_e = PathBuf::from(globals::APP_BIN_DIR.to_string()).exists();
    let app_build_dir_e = PathBuf::from(globals::APP_BUILD_DIR.to_string()).exists();
    let app_repo_dir_e = PathBuf::from(globals::APP_REPO_DIR.to_string()).exists();
    let app_scripts_dir_e = PathBuf::from(globals::APP_SCRIPTS_DIR.to_string()).exists();
    let db_e = Sqlite::database_exists(&globals::DB_URL).await?;

    if !app_dir_e
        || !app_bin_dir_e
        || !app_build_dir_e
        || !app_repo_dir_e
        || !app_scripts_dir_e
        || !db_e
    {
        println!("Necessary folders/files do not exist, generating them now");
        db_init().await?;
        return Ok(false);
    }
    Ok(true)
}

pub async fn db_init() -> Result<()> {
    let app_dir_p = PathBuf::from(globals::APP_DIR.to_string());
    let app_bin_dir_p = PathBuf::from(globals::APP_BIN_DIR.to_string());
    let app_build_dir_p = PathBuf::from(globals::APP_BUILD_DIR.to_string());
    let app_repo_dir_p = PathBuf::from(globals::APP_REPO_DIR.to_string());
    let app_scripts_dir_p = PathBuf::from(globals::APP_SCRIPTS_DIR.to_string());

    println!("[1] Initializing directories...");
    if !app_dir_p.exists() {
        std::fs::create_dir_all(globals::APP_DIR.to_string())?;
        println!("\t[✓] Created ~/.jkl/");
    } else {
        println!("\t[!] ~/.jkl/ already exists");
    }
    if !app_bin_dir_p.exists() {
        std::fs::create_dir_all(globals::APP_BIN_DIR.to_string())?;
        println!("\t[✓] Created ~/.jkl/bin");
    } else {
        println!("\t[!] ~/.jkl/bin already exists");
    }
    if !app_build_dir_p.exists() {
        std::fs::create_dir_all(globals::APP_BUILD_DIR.to_string())?;
        println!("\t[✓] Created ~/.jkl/build");
    } else {
        println!("\t[!] ~/.jkl/build already exists");
    }
    if !app_repo_dir_p.exists() {
        std::fs::create_dir_all(globals::APP_REPO_DIR.to_string())?;
        println!("\t[✓] Created ~/.jkl/repo");
    } else {
        println!("\t[!] ~/.jkl/repo already exists");
    }
    if !app_scripts_dir_p.exists() {
        std::fs::create_dir_all(globals::APP_SCRIPTS_DIR.to_string())?;
        println!("\t[✓] Created ~/.jkl/scripts");
    } else {
        println!("\t[!] ~/.jkl/scripts already exists");
    }

    println!("[2] Initializing database...");
    let db_exists = Sqlite::database_exists(&globals::DB_URL).await?;
    if !db_exists {
        Sqlite::create_database(&globals::DB_URL).await?;
        let mut conn = SqliteConnection::connect(&globals::DB_URL).await?;
        sqlx::query(
            "
        CREATE TABLE IF NOT EXISTS installed
        (
            name TEXT PRIMARY KEY NOT NULL,
            ver  TEXT NOT NULL
        );
        ",
        )
        .execute(&mut conn)
        .await?;
        println!("\t[✓] Created ~/.jkl/packages.db");
    } else {
        println!("\t[!] ~/.jkl/packages.db already exists");
    }
    println!("Initialization complete");

    Ok(())
}

pub async fn get_entry(name: &str) -> Result<Option<PData>> {
    let mut conn = SqliteConnection::connect(&globals::DB_URL).await?;

    sqlx::query_as::<_, PData>("SELECT * FROM installed WHERE name = ?")
        .bind(name)
        .fetch_optional(&mut conn)
        .await
        .map_err(|_| anyhow!("Error when attempting to fetch package from database"))
}

pub async fn delete_entry(name: &str) -> Result<()> {
    let mut conn = SqliteConnection::connect(&globals::DB_URL).await?;

    sqlx::query("DELETE FROM installed WHERE name = ?")
        .bind(name)
        .execute(&mut conn)
        .await?;

    Ok(())
}

pub async fn add_entry(entry_to_add: PData) -> Result<()> {
    let mut conn = SqliteConnection::connect(&globals::DB_URL).await?;

    sqlx::query(
        "
    INSERT INTO installed 
    VALUES (?, ?)
    ",
    )
    .bind(entry_to_add.name)
    .bind(entry_to_add.ver)
    .execute(&mut conn)
    .await?;

    Ok(())
}

pub async fn add_or_update_entry(entry_to_add: PData) -> Result<()> {
    match get_entry(&entry_to_add.name).await? {
        Some(_) => {
            let mut conn = SqliteConnection::connect(&globals::DB_URL).await?;

            sqlx::query("UPDATE installed SET ver = ? WHERE name = ?")
                .bind(entry_to_add.ver)
                .bind(entry_to_add.name)
                .execute(&mut conn)
                .await?;
        }
        None => {
            add_entry(entry_to_add).await?;
        }
    }
    Ok(())
}

pub async fn print_db() -> Result<()> {
    let mut conn = SqliteConnection::connect(&globals::DB_URL).await?;

    let res = sqlx::query_as::<_, PData>("SELECT * FROM installed")
        .fetch_all(&mut conn)
        .await?;

    println!("Package count: {}", res.len());
    if res.len() > 0 {
        println!("Installed:");
        for r in res {
            println!("\t{} [{}]", r.name, r.ver);
        }
    } else {
        println!("No packages to list!");
    }

    Ok(())
}
