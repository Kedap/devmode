use anyhow::{Context, Result};
use requestty::Answer;

use dmdlib::utils::clone::Clone;

use crate::cmd::*;
use dmdlib::utils::config::AppOptions;
use dmdlib::utils::editor::{Editor, EditorApp};
use dmdlib::utils::host::Host;

pub fn clone_setup() -> Result<Cmd> {
    let mut clone = Clone::new(None, None, None);
    let question = requestty::Question::select("host")
        .message("Choose your Git host:")
        .choices(vec!["GitHub", "GitLab"])
        .build();
    if let Answer::ListItem(host) = requestty::prompt_one(question)? {
        clone.host = Host::from(host.text);
    }
    let question = requestty::Question::input("owner")
        .message("Git username:")
        .validate(|owner, _previous| {
            if owner.is_empty() {
                Err("Please enter a Git username.".to_owned())
            } else {
                Ok(())
            }
        })
        .build();
    if let Answer::String(owner) = requestty::prompt_one(question)? {
        clone.owner = Option::from(owner);
    }
    let question = requestty::Question::input("repo")
        .message("Git repo name:")
        .validate(|owner, _previous| {
            if owner.is_empty() {
                Err("Please enter a Git repo name.".to_owned())
            } else {
                Ok(())
            }
        })
        .build();
    if let Answer::String(repo) = requestty::prompt_one(question)? {
        clone.repo = Option::from(repo);
    }
    Ok(Cmd::Clone(clone))
}

pub fn config_all() -> Result<Cmd> {
    let editor = config_editor()?;
    let editor = if let Cmd::Config(options) = editor {
        options.editor
    } else {
        Editor::default()
    };
    let owner = config_owner()?;
    let owner = if let Cmd::Config(options) = owner {
        options.owner
    } else {
        String::new()
    };
    let host = config_host()?;
    let host = if let Cmd::Config(options) = host {
        options.host
    } else {
        String::new()
    };
    Ok(Cmd::Config(AppOptions::new(host, owner, editor)))
}

pub fn config_owner() -> Result<Cmd> {
    let question = requestty::Question::input("owner")
        .message("What's your Git username:")
        .validate(|owner, _previous| {
            if owner.is_empty() {
                Err("Please enter a Git username.".to_owned())
            } else {
                Ok(())
            }
        })
        .build();
    let mut options = AppOptions::current().unwrap_or_default();
    if let Answer::String(owner) = requestty::prompt_one(question)? {
        options.owner = owner;
    }
    Ok(Cmd::Config(options))
}

pub fn config_host() -> Result<Cmd> {
    let question = requestty::Question::select("host")
        .message("Choose your Git host:")
        .choices(vec!["GitHub", "GitLab"])
        .build();
    let mut options = AppOptions::current().unwrap_or_default();
    if let Answer::ListItem(host) = requestty::prompt_one(question)? {
        options.host = Host::from(host.text)
            .with_context(|| "Couldn't get a host.")?
            .to_string();
    }
    Ok(Cmd::Config(options))
}

pub fn config_editor() -> Result<Cmd> {
    let question = requestty::Question::select("editor")
        .message("Choose your favorite editor:")
        .choices(vec!["Vim", "VSCode", "Custom"])
        .build();
    let mut options = AppOptions::current().unwrap_or_default();
    if let Answer::ListItem(i) = requestty::prompt_one(question)? {
        if i.text.to_lowercase() == "custom" {
            let mut command: Option<String> = None;
            let question = requestty::Question::input("command")
                .message("Editor command:")
                .validate(|owner, _previous| {
                    if owner.is_empty() {
                        Err("Please enter a editor command".to_owned())
                    } else {
                        Ok(())
                    }
                })
                .build();
            if let Answer::String(cmd) = requestty::prompt_one(question).unwrap() {
                command = Option::from(cmd);
            }
            options.editor = Editor::custom(command.unwrap());
        } else {
            options.editor = Editor::new(EditorApp::from(&*i.text));
        }
    }
    Ok(Cmd::Config(options))
}
