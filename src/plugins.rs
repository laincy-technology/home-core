use color_eyre::eyre::Result;
use laincy_home_pdk::Plugin;
use std::{env, fs::read_dir};

pub type Plugins = Vec<Plugin>;

pub async fn loaded() -> Result<Plugins> {
    let mut plugins = Plugins::new();
    let mut cwd = env::current_exe().unwrap();
    cwd.push("../plugins");
    let plugin_files = read_dir(cwd)?;

    for file in plugin_files {
        let plugin_path = file?.path().join("plugin.json");
        let plugin_info = std::fs::read_to_string(plugin_path);
        let plugin: Plugin = serde_json::from_str(&plugin_info?)?;
        plugins.push(plugin);
    }
    Ok(plugins)
}

pub async fn active() -> Result<Plugins> {
    let plugins = Plugins::new();

    Ok(plugins)
}
