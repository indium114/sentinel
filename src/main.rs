use mlua::{Lua, Result, Table};
use std::{fs, process, thread};

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

    // MARK: watch directories
    let mut processes = Vec::new();

    for pair in config.pairs::<String, Table>() {
        let (k, v) = pair?;
        usefulog::hint(format!("watching {k}"));
        println!("value: {:#?}", v);
        let process = thread::spawn(move || {
            help::spawn_watcher(
                &k,
                v.get("recurse")
                    .expect(format!("{k}.recurse is not a boolean").as_str()),
            )
        });
        processes.push(process);
    }

    for process in processes {
        if let Err(e) = process.join() {
            usefulog::err(format!("a watcher thread panicked! {:#?}", e));
            panic!()
        }
    }

    Ok(())
}
