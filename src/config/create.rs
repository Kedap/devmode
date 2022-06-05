use anyhow::{anyhow, bail, Context, Result};
use libset::routes::home;
use std::{env, fs, process::Command};

use crate::constants::messages::*;

#[derive(Debug, Clone)]
pub enum LanguagesSupport {
    Javascript,
    Ruby,
    None,
}

pub struct CreateAction {
    pub owner: String,
    pub repos: Vec<String>,
    pub manager: LangManager,
}

#[derive(Debug, Clone)]
pub struct LangManager {
    pub name: String,
    pub args_create: Vec<String>,
    pub lang: LanguagesSupport,
}

impl Default for CreateAction {
    fn default() -> Self {
        Self::new()
    }
}

impl LangManager {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            args_create: Vec::new(),
            lang: LanguagesSupport::None,
        }
    }

    pub fn from(name: String, args_create: Vec<String>, lang: LanguagesSupport) -> Self {
        Self {
            name,
            args_create,
            lang,
        }
    }

    pub fn from_lang(lang: String) -> Self {
        match lang.to_lowercase().as_str() {
            "javascript" => Self::from(
                String::from("yarn"),
                vec![String::from("init"), String::from("--yes")],
                LanguagesSupport::Javascript,
            ),
            "ruby" => Self::from(
                String::from("bundle"),
                vec![String::from("init")],
                LanguagesSupport::Ruby,
            ),
            _ => Self::new(),
        }
    }

    pub fn run(&self) {
        let mut child = Command::new(self.name.clone())
            .args(self.args_create.clone())
            .spawn()
            .expect(FAILED_TO_CREATE_REPO);
        let _result = child.wait();
    }
}

impl CreateAction {
    pub fn new() -> Self {
        CreateAction {
            owner: String::new(),
            repos: Vec::new(),
            manager: LangManager::new(),
        }
    }
    pub fn from(owner: &str, repos: Vec<String>, lang: String) -> Self {
        let owner = owner.to_string();
        let repos = repos.iter().map(|r| r.to_string()).collect();
        CreateAction {
            owner,
            repos,
            manager: LangManager::from_lang(lang),
        }
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
            if !self.manager.name.is_empty() {
                let curret_path = env::current_dir()?;
                fs::create_dir_all(path.clone())?;
                env::set_current_dir(path.clone())?;
                self.manager.run();
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
