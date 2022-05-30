use anyhow::{anyhow, bail, Context, Result};
use libset::routes::home;
use std::{env, fs, process::Command};

use crate::constants::messages::*;

pub enum ProjectLang {
    Javascript,
    None,
}

pub struct CreateAction {
    pub owner: String,
    pub repos: Vec<String>,
    pub lang: ProjectLang,
}

impl Default for CreateAction {
    fn default() -> Self {
        Self::new()
    }
}

impl CreateAction {
    pub fn new() -> Self {
        CreateAction {
            owner: String::new(),
            repos: Vec::new(),
            lang: ProjectLang::None,
        }
    }
    pub fn from(owner: &str, repos: Vec<String>, lang: String) -> Self {
        let owner = owner.to_string();
        let repos = repos.iter().map(|r| r.to_string()).collect();
        let lang = match lang.to_lowercase().as_str() {
            "javascript" => ProjectLang::Javascript,
            _ => ProjectLang::None,
        };
        CreateAction { owner, repos, lang }
    }
    pub fn run(&self) -> Result<()> {
        if self.owner.is_empty() {
            bail!("Missing arguments: <owner> <repo>")
        } else if self.repos.is_empty() {
            bail!("Missing arguments: <repo>")
        } else {
            self.create_repo()
        }
    }
    pub fn create_repo(&self) -> Result<()> {
        let mut error = anyhow!("");
        for (ix, repo) in self.repos.iter().enumerate() {
            let path = format!(
                "{}/Developer/{}/{}/{}",
                home().display(),
                "local",
                self.owner,
                repo
            );
            println!("Creating {}/{}...", self.owner, repo);
            if let ProjectLang::Javascript = self.lang {
                let curret_path = env::current_dir()?;
                fs::create_dir_all(path.clone())?;
                env::set_current_dir(path.clone())?;
                let mut child = Command::new("yarn")
                    .arg("init")
                    .arg("--yes")
                    .spawn()
                    .expect(FAILED_TO_CREATE_REPO);
                let _result = child.wait();
                env::set_current_dir(curret_path)?;
            } else if let Err(e) =
                git2::Repository::init(path).with_context(|| FAILED_TO_CREATE_REPO)
            {
                error = e;
            }

            if self.repos.len() == ix + 1 {
                return Ok(());
            }
        }
        Err(error)
    }
}
