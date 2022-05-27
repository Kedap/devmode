use anyhow::{anyhow, bail, Context, Result};
use libset::routes::home;

use crate::constants::messages::*;

pub struct CreateAction {
    pub owner: String,
    pub repos: Vec<String>,
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
        }
    }
    pub fn from(owner: &str, repos: Vec<String>) -> Self {
        let owner = owner.to_string();
        let repos = repos.iter().map(|r| r.to_string()).collect();
        CreateAction { owner, repos }
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
            if let Err(e) = git2::Repository::init(path).with_context(|| FAILED_TO_CREATE_REPO) {
                error = e;
            } else if self.repos.len() == ix + 1 {
                return Ok(());
            }
        }
        Err(error)
    }
}
