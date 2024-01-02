#![feature(lazy_cell)]
#![feature(path_file_prefix)]

mod backend;
pub mod cli;
pub mod db;
mod globals;

pub use backend::packages;
