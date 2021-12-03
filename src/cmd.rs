use crate::config::clone::CloneAction;
use crate::config::fork::Fork;
use crate::config::host::{is_host, Host};
use crate::config::project::Project;
use crate::config::settings::Settings;
use crate::constants::patterns::GIT_URL;
use anyhow::Result;
use anyhow::{bail, Context};
use clap::ArgMatches;
use libdmd::utils::config::Config;
use libdmd::utils::config::format::FileFormat::TOML;
use regex::bytes::Regex;

use crate::cli::{clone_setup, config_all, config_editor, config_host, config_owner, fork_setup};
use crate::constants::messages::APP_OPTIONS_NOT_FOUND;

pub enum Cmd {
    Clone(CloneAction),
    Fork(Fork),
    Open(Project),
    Config(Settings),
    ShowConfig,
    MapPaths,
    None,
}

impl<'a> Cmd {
    pub fn new(matches: &'a ArgMatches<'a>) -> Result<Cmd> {
        if let Some(matches) = matches.subcommand_matches("clone") {
            let args = matches
                .values_of("args")
                .unwrap_or_default()
                .collect::<Vec<_>>();
            let first = args.get(0).copied().unwrap_or_default();
            let rx = Regex::new(GIT_URL).unwrap();
            if args.is_empty() {
                clone_setup()
            } else if rx.is_match(first.as_ref()) {
                let clone = CloneAction::parse_url(first, rx)?;
                Ok(Cmd::Clone(clone))
            } else if is_host(&args) {
                let host = Host::from(first.into());
                let owner = args.get(1).map(|a| a.to_string());
                let repo = args.get(2).map(|a| a.to_string());
                Ok(Cmd::Clone(CloneAction::from(
                    host,
                    owner.unwrap(),
                    vec![repo.unwrap()],
                )))
            } else {
                let options = Config::get::<Settings>("devmode/config/config.toml", TOML)
                    .with_context(|| APP_OPTIONS_NOT_FOUND)?;
                let host = Host::from(options.host);
                let repos = args.iter().map(|a| a.to_string()).collect::<Vec<String>>();
                Ok(Cmd::Clone(CloneAction::from(host, options.owner, repos)))
            }
        } else if let Some(matches) = matches.subcommand_matches("fork") {
            let clone_arg = matches
                .values_of("args")
                .unwrap_or_default()
                .collect::<Vec<_>>();
            let upstream_arg = matches
                .values_of("upstream")
                .unwrap_or_default()
                .collect::<Vec<_>>();
            let upstream_url = upstream_arg[0]; //upstream_url exist for is deleted on future
            let first = clone_arg.get(0).copied().unwrap_or_default();
            let rx = Regex::new(GIT_URL).unwrap();
            if clone_arg.is_empty() {
                fork_setup()
            } else if rx.is_match(first.as_ref()) {
                let fork = Fork::parse_url(first, rx, upstream_url.to_string())?;
                Ok(Cmd::Fork(fork))
            } else if clone_arg.len() == 1 {
                let options = Config::get::<Settings>("devmode/config/config.toml", TOML)
                    .with_context(|| APP_OPTIONS_NOT_FOUND)?;
                let host = Host::from(options.host);
                let repo = clone_arg.get(0).map(|a| a.to_string());
                Ok(Cmd::Fork(Fork::from(
                    host,
                    upstream_url.to_string(),
                    options.owner,
                    repo.unwrap(),
                )))
            } else {
                let host = Host::from(first.into());
                let owner = clone_arg.get(1).map(|a| a.to_string());
                let repo = clone_arg.get(2).map(|a| a.to_string());
                Ok(Cmd::Fork(Fork::from(
                    host,
                    upstream_url.to_string(),
                    owner.unwrap(),
                    repo.unwrap(),
                )))
            }
        } else if let Some(open) = matches.subcommand_matches("open") {
            Ok(Cmd::Open(Project {
                name: open.value_of("project").map(|a| a.to_string()),
            }))
        } else if let Some(config) = matches.subcommand_matches("config") {
            if config.is_present("all") {
                match config_all() {
                    None => bail!("Failed to configure."),
                    Some(cmd) => Ok(cmd),
                }
            } else if config.is_present("map") {
                Ok(Cmd::MapPaths)
            } else if Config::get::<Settings>("devmode/config/config.toml", TOML).is_some() {
                if config.is_present("editor") {
                    match config_editor() {
                        None => bail!("Failed to set editor."),
                        Some(cmd) => Ok(cmd),
                    }
                } else if config.is_present("owner") {
                    match config_owner() {
                        None => bail!("Failed to set owner."),
                        Some(cmd) => Ok(cmd),
                    }
                } else if config.is_present("host") {
                    match config_host() {
                        None => bail!("Failed to set host."),
                        Some(cmd) => Ok(cmd),
                    }
                } else if config.is_present("show") {
                    Ok(Cmd::ShowConfig)
                } else {
                    Ok(Cmd::Config(
                        Config::get::<Settings>("devmode/config/config.toml", TOML)
                            .with_context(|| APP_OPTIONS_NOT_FOUND)?,
                    ))
                }
            } else {
                let cmd = config_all().unwrap();
                if let Cmd::Config(settings) = &cmd {
                    settings.init()?;
                }
                Ok(cmd)
            }
        } else {
            Ok(Cmd::None)
        }
    }
    pub fn check(&self) -> Result<()> {
        match self {
            Cmd::Clone(clone) => clone.run(),
            Cmd::Open(project) => project.run(),
            Cmd::Config(settings) => settings.run(),
            Cmd::ShowConfig => {
                Config::get::<Settings>("devmode/config/config.toml", TOML)
                    .with_context(|| APP_OPTIONS_NOT_FOUND)?
                    .show();
                Ok(())
            }
            Cmd::Fork(fork) => fork.run(),
            Cmd::None => bail!("No argument found."),
            Cmd::MapPaths => Project::make_dev_paths(),
        }
    }
}
