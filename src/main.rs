use std::io::stdin;

fn handle_input(running: &mut bool, input: &String, list: &mut Vec<(String, bool)>) {
    println!("input: {}", input);
    match input.trim() {
        "q" | "quit" | "exit" => {
            println!("exiting program");
            *running = false;
        }
        "add" | "Add" => {
            println!("add item");
            add_item(list);
        }
        "finish" => {
            println!("item done");
            complete_item(list)
        }
        "clear" => {
            println!("clearing finished items");
            remove_finished(list)
        }
        "list" | "List" => {
            println!("list items")
        }
        _ => println!("didn't recognize command"),
    }
}

fn add_item(list: &mut Vec<(String, bool)>) {
    let mut item_buf = String::new();
    println!("input todo item:");
    stdin()
        .read_line(&mut item_buf)
        .unwrap_or_default()
        .to_string();
    list.push((item_buf, false));
}

fn complete_item(list: &mut Vec<(String, bool)>) {
    println!("input item # you completed:");
    let mut item_buf = String::new();
    stdin().read_line(&mut item_buf).unwrap_or_default();
    let trimmed = item_buf.trim();
    match trimmed.parse::<usize>() {
        Ok(i) => {
            if i < list.len() {
                list[i].1 = true
            } else {
                println!("no item at index {}", i)
            }
        }
        Err(..) => println!("the input was not a valid index"),
    }
}

fn remove_finished(list: &mut Vec<(String, bool)>) {
    let mut i = 0;
    while i < list.len() {
        if list[i].1 {
            list.remove(i);
        } else {
            i += 1
        }
    }
}

fn main() {
    // commands:
    // - Add
    // - Complete
    // - List
    let mut list: Vec<(String, bool)> = vec![];
    let mut running = true;
    while running {
        let mut buffer = String::new();
        println!("use 'help' for list of commands\nEnter a command:");
        stdin().read_line(&mut buffer).unwrap_or_default();
        // handle input switch based on what the user input.
        handle_input(&mut running, &buffer, &mut list);
        println!("Hello, world! {}", buffer);
        println!("{:?}", list);
    }
}
