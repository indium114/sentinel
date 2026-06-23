use notify::Result;
use std::{path::Path, sync::mpsc};

mod help;

fn main() -> Result<()> {
    let path = "/home/indium114/temp";

    help::spawn_watcher(path, true)
}
