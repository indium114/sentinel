use notify::{Event, EventKind, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mpsc};

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
        "unknown".to_string()
    }
}

pub fn spawn_watcher(path: &str, recursive: bool) -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;

    let recurse = match recursive {
        true => RecursiveMode::Recursive,
        false => RecursiveMode::NonRecursive,
    };

    watcher.watch(Path::new(path), recurse)?;
    for res in rx {
        match res {
            Ok(event) => {
                println!("== NEW EVENT ==");
                println!("event: {}", kind_of_event(&event.kind));
                println!("paths: {:#?}", event.paths);
            }
            Err(e) => println!("error: {:#?}", e),
        }
    }

    Ok(())
}
