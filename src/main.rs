extern crate notify;

use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

struct Rule {
    name: String,
    condition: String,
    category: String,
    value: String
}

struct Action {
    name: String,
    date_created: String,
    rule: Rule,
    category: String,
    to: String,
    directory: String
}

fn build_some_test_rules() -> Action {
    let rule = Rule{
        name: "Test".to_string(),
        condition: "GTE".to_string(),
        category: "SIZE".to_string(), 
        value: "1KB".to_string()
    };

    let action = Action{
        name: "Copy large files".to_string(),
        date_created: "now".to_string(),
        rule: rule,
        category: "COPY".to_string(),
        to: "/home/sal/Documents".to_string(),
        directory: "/home/sal/Development/folder_maid/folder_maid".to_string() 
    };

    return action;
}

fn main() {
    let (tx, rx) = channel();
    
    let action = build_some_test_rules();
    
    println!("{}", action.name);

    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    watcher.watch("/home/sal/Development", RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
           Ok(event) => println!("{:?}", event),
           Err(e) => println!("watch error: {:?}", e),
        }
    }
}
