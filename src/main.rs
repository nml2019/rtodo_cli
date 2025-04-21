use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { item: String },
    Complete { item: String },

    List,
}
#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    map: HashMap<String, TodoEntry>,
    next_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoEntry {
    id: u64,
    completed: bool,
}

impl Todo {
    fn insert(&mut self, key: String) {
        let current_id = self.next_id;
        self.next_id += 1;
        self.map.insert(
            key,
            TodoEntry {
                id: current_id,
                completed: false,
            },
        );
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;

        serde_json::to_writer_pretty(f, &self)?;
        Ok(())

        // let mut content = String::new();
        // for (k, v) in self.map {
        //     let record = format!("{}\t{}\n", k, v);
        //     content.push_str(&record);
        // }
        // std::fs::write("db.txt", content)
    }

    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;

        let todo: Todo = match serde_json::from_reader(f) {
            Ok(todo) => todo,
            Err(e) if e.is_eof() => Todo {
                map: HashMap::new(),
                next_id: 0,
            },
            Err(e) => panic!("An error occurred: {}", e),
        };
        // let mut content = String::new();
        // f.read_to_string(&mut content)?;
        // let map: HashMap<String, bool> = content
        //     .lines()
        //     .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
        //     .map(|v| (v[0], v[1]))
        //     .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
        //     .collect();
        // Ok(Todo { map })
        Ok(todo)
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(entry) => Some(entry.completed = false),
            None => None,
        }
    }
}

fn main() {
    let args = Cli::parse();
    // let action = std::env::args().nth(1).expect("Please specify an action");
    // let item = std::env::args().nth(2).expect("Please specify an item");

    // println!("{:?}, {:?}", args.command, args.item);

    let mut todo = Todo::new().expect("Initializing db error");

    let mut needs_save = false;

    match args.command {
        Commands::Add { item } => {
            todo.insert(item.clone());
            println!("'{}' added", item);
            needs_save = true;
        }
        Commands::Complete { item } => match todo.complete(&item) {
            None => println!("'{}' is not found in the list", item),
            Some(_) => {
                println!("'{}' completed", item);
                needs_save = true;
            }
        },
        Commands::List => {
            println!("{:?}", todo.map);
        }
    }

    if needs_save {
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    }

    // if args.command == "add" {
    //     todo.insert(args.item);
    //     match todo.save() {
    //         Ok(_) => println!("todo saved"),
    //         Err(why) => println!("An error occurred: {}", why),
    //     }
    // } else if args.command == "complete" {
    //     match todo.complete(&args.item) {
    //         None => println!("{} is not found in the list", args.item),
    //         Some(_) => match todo.save() {
    //             Ok(_) => println!("todo saved"),
    //             Err(why) => println!("An error occurred: {}", why),
    //         },
    //     }
    // } else if args.command == "list" {
    //     println!("{:?}", todo.map);
    // }
}
