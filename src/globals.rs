use std::env;
use std::sync::LazyLock;

pub(crate) static APP_DIR_NAME: LazyLock<String> = LazyLock::new(|| String::from(".jkl"));
pub(crate) static APP_DIR: LazyLock<String> = LazyLock::new(|| {
    format!(
        "{}/{}",
        env::var("HOME").expect("$HOME is not defined"),
        *APP_DIR_NAME
    )
});
pub(crate) static APP_BIN_DIR: LazyLock<String> = LazyLock::new(|| format!("{}/bin", *APP_DIR));
pub(crate) static APP_BUILD_DIR: LazyLock<String> = LazyLock::new(|| format!("{}/build", *APP_DIR));
pub(crate) static APP_REPO_DIR: LazyLock<String> = LazyLock::new(|| format!("{}/repo", *APP_DIR));
pub(crate) static APP_SCRIPTS_DIR: LazyLock<String> =
    LazyLock::new(|| format!("{}/scripts", *APP_DIR));

pub(crate) static DB_FILE: LazyLock<String> = LazyLock::new(|| String::from("packages.db"));
pub(crate) static DB_URL: LazyLock<String> =
    LazyLock::new(|| format!("sqlite:{}/{}", *APP_DIR, *DB_FILE));
