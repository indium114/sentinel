use notify::{Event, EventKind, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mpsc};

pub fn home() -> String {
    let dir = dirs::home_dir();
    dir
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default()
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

    #[cfg(debug_assertions)]
    println!("spawning watcher for {path}");

    watcher.watch(Path::new(path), recurse)?;
    for res in rx {
        match res {
            Ok(event) => {
                #[cfg(debug_assertions)]
                println!("== NEW EVENT ==");
                #[cfg(debug_assertions)]
                println!("event: {}", kind_of_event(&event.kind));
                #[cfg(debug_assertions)]
                println!("paths: {:#?}", event.paths);

                match kind_of_event(&event.kind).as_str() {
                    "access" => {
                        let func: mlua::Function = functions.get("access").unwrap_or_else(|_| panic!("no 'access' function for {path}"));
                        let _: () = func.call(event.paths).expect("failed to call 'access'");
                        usefulog::log(format!("access event in {path}"))
                    },
                    "create" => {
                        let func: mlua::Function = functions.get("create").unwrap_or_else(|_| panic!("no 'create' function for {path}"));
                        let _: () = func.call(event.paths).expect("failed to call 'create");
                        usefulog::log(format!("create event in {path}"))
                    },
                    "modify" => {
                        let func: mlua::Function = functions.get("modify").unwrap_or_else(|_| panic!("no 'modify' function for {path}"));
                        let _: () = func.call(event.paths).expect("failed to call 'modify'");
                        usefulog::log(format!("modify event in {path}"))
                    },
                    "remove" => {
                        let func: mlua::Function = functions.get("remove").unwrap_or_else(|_| panic!("no 'remove' function for {path}"));
                        let _: () = func.call(event.paths).expect("failed to call 'remove'");
                        usefulog::log(format!("remove event in {path}"))
                    },
                    "other" => {
                        let func: mlua::Function = functions.get("other").unwrap_or_else(|_| panic!("no 'other' function for {path}"));
                        let _: () = func.call(event.paths).expect("failed to call 'other'");
                        usefulog::log(format!("other event in {path}"))
                    },
                    &_ => panic!("tried to call nonexistent event!"),
                }
            }
            Err(e) => usefulog::err(format!("{:#?}", e)),
        }
    }

    Ok(())
}
