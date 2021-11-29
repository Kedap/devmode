use crate::config::editor::Editor;
use anyhow::Result;
use libdmd::utils::config::config::Config;
use libdmd::utils::config::directory::Directory;
use libdmd::utils::config::file::File;
use libdmd::utils::config::format::FileFormat;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, Eq, PartialEq)]
pub struct Settings {
    pub host: String,
    pub owner: String,
    pub editor: Editor,
}

impl Settings {
    pub fn new(host: String, owner: String, editor: Editor) -> Self {
        Settings {
            host,
            owner,
            editor,
        }
    }
    pub fn init(&self) -> Result<()> {
        Config::new()
            .project("devmode")
            .dir(
                Directory::new().name("config").file(
                    File::new()
                        .name("config")
                        .format(FileFormat::TOML)
                        .data(self)?,
                ),
            )
            .dir(Directory::new().name("logs"))
            .dir(Directory::new().name("paths"));
        Ok(())
    }
    pub fn show(&self) {
        println!(
            "Current settings: \n\
        Host: {}\n\
        Owner: {}\n\
        Editor: {}",
            self.host, self.owner, self.editor.app
        )
    }
}
