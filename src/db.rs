use crate::template::Template;
use anyhow::{Context, Result};
use log::debug;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct Store {
    path: PathBuf,
    templates: HashMap<String, Template>,
}

impl Store {
    pub fn load(path: &Path) -> Result<Self> {
        debug!("Loading store from {:?}", path);
        let mut db = Store {
            path: path.to_path_buf(),
            templates: HashMap::new(),
        };

        if path.exists() {
            let len = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
            if len > 0 {
                let bytes = fs::read(path).context("Failed to open store database")?;
                if let Ok(data) = wincode::deserialize::<HashMap<String, Template>>(&bytes) {
                    db.templates = data;
                    debug!("Loaded {} templates", db.templates.len());
                }
            } else {
                debug!("Store file is empty");
            }
        } else {
            debug!("Store file does not exist, creating new");
        }
        Ok(db)
    }

    pub fn save(&self) -> Result<()> {
        debug!("Saving store to {:?}", self.path);
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let encoded =
            wincode::serialize(&self.templates).context("Failed to encode store database")?;
        fs::write(&self.path, encoded).context("Failed to create store database file")?;

        Ok(())
    }

    pub fn add(&mut self, template: Template) -> Result<()> {
        debug!("Adding template: {}", template.manifest.name);
        self.templates
            .insert(template.manifest.name.clone(), template);
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Option<Template> {
        debug!("Removing template: {}", name);
        self.templates.remove(name)
    }

    pub fn clear(&mut self) {
        debug!("Clearing store (removing all templates)");
        self.templates.clear();
    }

    pub fn set_ignored(&mut self, name: &str, state: bool) -> Result<bool> {
        if let Some(tpl) = self.templates.get_mut(name) {
            tpl.manifest.ignored = state;
            debug!("Set ignored status for {} to {}", name, state);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn list(&self) -> Vec<&Template> {
        let mut list: Vec<&Template> = self.templates.values().collect();
        list.sort_by_key(|t| &t.manifest.name);
        list
    }

    pub fn iter(&self) -> std::collections::hash_map::Values<'_, String, Template> {
        self.templates.values()
    }
}
