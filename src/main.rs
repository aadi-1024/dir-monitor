use inotify::{Inotify, WatchMask};
use std::env;
use std::path::PathBuf;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("No path provided");
        return
    }

    let path = PathBuf::from(&args[1]);
    if !path.is_dir() {
        eprintln!("Invalid directory; Doesn't exist or is a file");
        return
    }

    let mut fd = Inotify::init().expect("Couldnt generate fd");
    fd.add_watch(&path, WatchMask::CREATE).expect("Couldnt add watch");
    let mut buff = [0; 1024];

    loop {
        let event = fd.read_events_blocking(&mut buff).expect("Couldnt read event");
        for e in event {
            let mut path_temp = path.clone();
            if let Some(t) = e.name {
                path_temp.push(t);
                println!("{}", path_temp.display());
            } else {
                eprintln!("Event returned None for the filename");
            }
        }
    }
}
