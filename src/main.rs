use clap::Parser;
use std::collections::HashMap;

#[derive(Parser)]
struct Cli {
    command: String,
    item: String,
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;

        serde_json::to_writer_pretty(f, &self.map)?;
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

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
        // let mut content = String::new();
        // f.read_to_string(&mut content)?;
        // let map: HashMap<String, bool> = content
        //     .lines()
        //     .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
        //     .map(|v| (v[0], v[1]))
        //     .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
        //     .collect();
        // Ok(Todo { map })
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

fn main() {
    let args = Cli::parse();
    // let action = std::env::args().nth(1).expect("Please specify an action");
    // let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?}, {:?}", args.command, args.item);

    let mut todo = Todo::new().expect("Initializing db error");

    if args.command == "add" {
        todo.insert(args.item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if args.command == "complete" {
        match todo.complete(&args.item) {
            None => println!("{} is not found in the list", args.item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    }
}
