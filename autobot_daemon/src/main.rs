use daemon::run_daemon;
use shortcuts::read_shortcuts;
use std::thread;
mod daemon;
mod shortcuts;
use std::sync::mpsc;

fn main() {
    let handle = thread::spawn(|| run_daemon());
    // let (tx, rx) = mpsc::channel();

    // let mut shortcut_map = read_shortcuts();
    // //TODO : handle error

    // shortcut_map
    //     .save_shortcut(&mut vec!["a", "b"], "this is the shotcut file ab")
    //     .unwrap();

    handle.join();
}
