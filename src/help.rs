use notify::{Event, EventKind, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mpsc};
use mlua::{Lua, Error};

pub fn home() -> String {
    let dir = dirs::home_dir();
    return dir
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
}

pub fn kind_of_event(event: &EventKind) -> String {
    if event.is_access() {
        "access".to_string()
    } else if event.is_create() {
        "create".to_string()
    } else if event.is_modify() {
        "modify".to_string()
    } else if event.is_remove() {
        "remove".to_string()
    } else if event.is_other() {
        "other".to_string()
    } else {
        usefulog::warn("unknown fs event");
        "unknown".to_string()
    }
}

pub fn spawn_watcher(path: &str, recursive: bool, functions: mlua::Table) -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;

    let recurse = match recursive {
        true => RecursiveMode::Recursive,
        false => RecursiveMode::NonRecursive,
    };

    println!("spawning watcher for {path}");

    let lua = Lua::new();

    watcher.watch(Path::new(path), recurse)?;
    for res in rx {
        match res {
            Ok(event) => {
                println!("== NEW EVENT ==");
                println!("event: {}", kind_of_event(&event.kind));
                println!("paths: {:#?}", event.paths);

                match kind_of_event(&event.kind).as_str() {
                    "access" => {
                        let func: mlua::Function = functions.get("access").expect(&format!("no 'access' function for {path}"));
                        let _: () = func.call(event.paths).expect("failed to call 'access'");
                    },
                    "create" => {
                        let func: mlua::Function = functions.get("create").expect(&format!("no 'create' function for {path}"));
                        let _: () = func.call(event.paths).expect("failed to call 'create");
                    },
                    "modify" => {
                        let func: mlua::Function = functions.get("modify").expect(&format!("no 'modify' function for {path}"));
                        let _: () = func.call(event.paths).expect("failed to call 'modify'");
                    },
                    "remove" => {
                        let func: mlua::Function = functions.get("remove").expect(&format!("no 'remove' function for {path}"));
                        let _: () = func.call(event.paths).expect("failed to call 'remove'");
                    },
                    "other" => {
                        let func: mlua::Function = functions.get("other").expect(&format!("no 'other' function for {path}"));
                        let _: () = func.call(event.paths).expect("failed to call 'other'");
                    },
                    &_ => panic!("tried to call nonexistent event!"),
                }
            }
            Err(e) => usefulog::err(format!("{:#?}", e)),
        }
    }

    Ok(())
}
