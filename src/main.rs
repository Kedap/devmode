use crate::cmd::Cmd;
use anyhow::Result;
use clap::{load_yaml, App};
use libdmd::utils::config::config::Config;
use libdmd::utils::config::directory::Directory;
use libdmd::utils::config::file::File;
use libdmd::utils::config::format::FileFormat;

mod cli;
mod cmd;
mod config;
mod constants;

fn main() -> Result<()> {
    let yaml = load_yaml!("app.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut config = Config::new()
        .project("devmode")
        .dir(
            Directory::new()
                .name("config")
                .file(File::new().name("config").format(FileFormat::TOML)),
        )
        .dir(
            Directory::new().name("logs")
        )
        .dir(Directory::new().name("paths").file(
            File::new().name("devpaths").format(FileFormat::TOML) //TODO: Implement FileFormat::NONE
        ));
    if config.current().is_none() {
        config.build()?;
    }
    let cmd = Cmd::new(&matches)?;
    cmd.check()
}
