use mlua::{Lua, Result, Table};
use std::{fs, process};

mod help;

fn main() -> Result<()> {
    // MARK: load config
    let config_dir = help::home() + "/.config/sentinel/init.lua";
    // check if config dir exists
    match fs::exists(&config_dir) {
        Ok(true) => {
            usefulog::log(format!("loading config from {config_dir}"));
        }
        Ok(false) => {
            usefulog::err(format!("{} does not exist, please create it.", config_dir));
            process::exit(1);
        }
        Err(e) => {
            usefulog::err(format!("failed to read config due to {e}"));
            panic!("{}", e);
        }
    }

    let config_string = fs::read_to_string(config_dir)?;

    // MARK: instantiate lua and evaluate config
    let lua = Lua::new();
    let config: Table = lua.load(&config_string).eval()?;

    println!("{:#?}", config);

    Ok(())

    // help::spawn_watcher(path, true)
}
