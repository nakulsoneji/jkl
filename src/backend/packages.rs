use crate::db::actions;
use crate::db::types::PData;
use crate::globals;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::{self};
use std::process::{Command, Stdio};

// format: install for first install
//         update to check build folder name and replace old install
//         sync to update database before update
// TODO: Add checking db in install command and add update + sync commands
//

pub async fn install(package: &str) -> Result<()> {
    println!("Attempting to install package: {}", package);
    if let Some(repo_pkg_data) = get_repo_data(package)? {
        match actions::get_entry(&repo_pkg_data.name).await? {
            Some(db_pkg_data)
                if version_to_int(&db_pkg_data.ver)? < version_to_int(&repo_pkg_data.ver)? =>
            {
                println!(
                    "Package version in ~/.jkl/repos ({}) is greater than version in database ({})",
                    repo_pkg_data.ver, db_pkg_data.ver
                );
                println!("Run \"jkl sync && jkl update\" to update the package");
            }
            Some(db_pkg_data)
                if version_to_int(&db_pkg_data.ver)? == version_to_int(&repo_pkg_data.ver)? =>
            {
                println!(
                    "Package is already installed and up to date ({})",
                    repo_pkg_data.ver
                );
            }
            // only other possible Some(p) case is if version in repo is less than that in db, not using
            // match guard because rust-analyzer will complain otherwise
            Some(db_pkg_data) => {
                println!(
                    "Package version in ~/.jkl/repos ({}) is less than version in database ({})",
                    repo_pkg_data.ver, db_pkg_data.ver
                );
                println!("Consider renaming your build.sh to the correct version as jkl does not support downgrades");
            }
            None => {
                run_build(repo_pkg_data).await?;
            }
        }
        return Ok(());
    } else {
        println!();
        println!("Could not find package in ~/.jkl/repo");
        println!("Make sure you create ~/.jkl/repo/<BIN_NAME>-<BIN_VERSION>/build.sh");
        return Ok(());
    }
}

pub async fn run_build(package_data: PData) -> Result<()> {
    let path = format!(
        "{}/{}",
        globals::APP_REPO_DIR.to_string(),
        package_data.folder_name()
    );

    let mut envs = HashMap::new();
    envs.insert("BUILD_DIR", globals::APP_BUILD_DIR.to_string());
    envs.insert("BIN_DIR", globals::APP_BIN_DIR.to_string());
    envs.insert("V", package_data.ver.clone());
    envs.insert("P", package_data.name.clone());
    envs.insert("PV", package_data.folder_name());

    println!("[1] Running build.sh...");
    let out = Command::new("sh")
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .current_dir(globals::APP_BUILD_DIR.to_string())
        .envs(&envs)
        .arg(&format!("{}/build.sh", path))
        .output()?;

    let err = out.stderr;
    if err.len() > 0 {
        println!("\t[X] Error occurred while running build.sh");
        return Err(anyhow!(String::from_utf8(err)?));
    }
    println!("\t[✓] build.sh successfully executed");

    println!("[2] Updating database...");
    actions::add_or_update_entry(package_data.clone()).await?;
    println!("\t[✓] Database successfully updated");
    println!("Package installed at ~/.jkl/bin/{}", package_data.name);

    Ok(())
}

pub async fn update(package: &str) -> Result<()> {
    println!("Attempting to update package: {}", package);
    if let Some(db_pkg_data) = actions::get_entry(package).await? {
        match get_repo_data(package)? {
            Some(repo_pkg_data)
                if version_to_int(&repo_pkg_data.ver)? > version_to_int(&db_pkg_data.ver)? =>
            {
                println!("Updating package...");
                fs::remove_file(format!(
                    "{}/{}",
                    globals::APP_BIN_DIR.to_string(),
                    repo_pkg_data.name
                ))?;
                run_build(repo_pkg_data).await?;
            }
            Some(repo_pkg_data)
                if version_to_int(&db_pkg_data.ver)? == version_to_int(&repo_pkg_data.ver)? =>
            {
                println!(
                    "Package is already installed and up to date ({})",
                    repo_pkg_data.ver
                );
            }
            // only other possible Some(p) case is if version in repo is less than that in db, not using
            // match guard because rust-analyzer will complain otherwise
            Some(repo_pkg_data) => {
                println!(
                    "Package version in ~/.jkl/repo ({}) is less than version in database ({})",
                    repo_pkg_data.ver, db_pkg_data.ver
                );
                println!("Consider renaming your build.sh to the correct version as jkl does not support downgrades");
            }
            None => {
                println!("Package is in database but not repos");
                println!("Make sure you create ~/.jkl/repo/<BIN_NAME>-<BIN_VERSION>/build.sh and run \"jkl install <BIN_NAME\"");
            }
        }
    } else {
        println!("Package is not in the database");
        println!("Make sure you create ~/.jkl/repo/<BIN_NAME>-<BIN_VERSION>/build.sh and run \"jkl install <BIN_NAME>\"");
    }
    Ok(())
}

// this only removes the package from the database and its binary
pub async fn delete(package: &str) -> Result<()> {
    println!("Attempting to delete package: {}", package);
    if let Some(_) = actions::get_entry(package).await? {
        println!("[1] Deleting entry from database...");
        actions::delete_entry(package).await?;
        println!("\t[✓] Entry removed successfully");
        println!("[2] Deleting binary...");
        fs::remove_file(format!("{}/{}", globals::APP_BIN_DIR.to_string(), package))?;
        println!("\t[✓] Binary deleted successfully");
        println!();
        println!("Package deletion complete");
    } else {
        println!("Package is not installed!");
        println!("Make sure you create ~/.jkl/repo/<BIN_NAME>-<BIN_VERSION>/build.sh and run \"jkl install <BIN_NAME>\"");
    }
    Ok(())
}

pub async fn list() -> Result<()> {
    actions::print_db().await?;
    Ok(())
}

pub fn version_to_int(ver: &str) -> Result<i32> {
    ver.split(".")
        .collect::<String>()
        .parse::<i32>()
        .map_err(|_| anyhow!("Error converting version from string to int"))
}

pub fn get_repo_data(package: &str) -> Result<Option<PData>> {
    for dir in fs::read_dir(globals::APP_REPO_DIR.to_string())? {
        let entry = dir?;

        let pkg_folder = entry.file_name();

        let pkg_folder_str = pkg_folder
            .to_str()
            .ok_or(anyhow!("error converting file to string"))?
            .to_owned();

        // [0] is name, [1] is version
        let pkg_items = pkg_folder_str
            .split("-")
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();

        if pkg_items[0] == package {
            return Ok(Some(PData::new(&pkg_items[0], &pkg_items[1])));
        }
    }

    Ok(None)
}
