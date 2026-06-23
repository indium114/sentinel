use notify::{Event, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mpsc};

mod help;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(Path::new("/home/indium114/temp"), RecursiveMode::Recursive)?;
    for res in rx {
        match res {
            Ok(event) => {
                println!("== NEW EVENT ==");
                println!("event: {}", help::kind_of_event(&event.kind));
                println!("paths: {:#?}", event.paths);
            }
            Err(e) => println!("error: {:#?}", e),
        }
    }

    Ok(())
}
