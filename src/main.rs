use std::env;

use crate::common::*;

mod common;
mod setup;

fn main() {
    // Startup Checks
    setup::create_directory();

    let args: Vec<String> = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("No command provided");
        std::process::exit(1);
    }

    if args.get(1) == Some(&"add".into()) {
        if args.get(2) == Some(&"category".into()) {
            match args.get(3) {
                Some(name) => {
                    match create_category(name) {
                        Ok(path) => println!("Category created at: {}", path.to_str().unwrap()),
                        Err(e) => eprintln!("Error creating category: {}", e),
                    }
                }
                None => {
                    eprintln!("Category name not provided");
                    std::process::exit(1);
                }
            }
        } else if args.get(2) == Some(&"note".into()) {
            if args.len() < 5 {
                match args.get(3) {
                    Some(name) => {
                        match create_uncategorized_note(&name) {
                            Ok(path) => println!("Note created at: {}", path.to_str().unwrap()),
                            Err(e) => eprintln!("Error creating note: {}", e),
                        }
                    },
                    None => {
                        eprintln!("Category name not provided");
                        std::process::exit(1);
                    }
                }
            } else {
                let category: &String = args.get(3).unwrap();
                let name: &String = args.get(4).unwrap();

                match create_categorized_note(category, name) {
                    Ok(path) => println!("Note created at: {}", path.to_str().unwrap()),
                    Err(e) => eprintln!("Error creating note: {}", e),  
                }
            }
        } else {
            eprintln!("No list type provided");
            std::process::exit(1);
        }
    } else if args.get(1) == Some(&"list".into()) {
        if args.get(2) == Some(&"categories".into()) {
            for entry in get_categories().iter() {
                println!("{}", entry.to_str().unwrap())
            };
        } else if args.get(2) == Some(&"notes".into()) {
            if args.len() < 4 {
                for entry in get_uncategorized_notes().iter() {
                    println!("{}", entry.to_str().unwrap())
                };
            } else {
                let category = args.get(3).unwrap();
                for entry in get_categorized_notes(category).iter() {
                    println!("{}", entry.to_str().unwrap())
                };
            }
        } else {
            eprintln!("No list type provided");
            std::process::exit(1);
        }
    } else if args.get(1) == Some(&"open".into()) {
        if args.len() < 4 {
            if args.get(2) != None {
                match open_uncategorized_note(args.get(2).unwrap()) {
                    Ok(path) => println!("Note opened at: {}", path.to_str().unwrap()),
                    Err(e) => eprintln!("Error opening note: {}", e),  
                };
            } else {
                eprintln!("Note name not provided");
                std::process::exit(1);
            }
        } else {
            let category = args.get(2).unwrap();
            let name = args.get(3).unwrap();

            match open_categorized_note(category, name) {
                Ok(path) => println!("Note opened at: {}", path.to_str().unwrap()),
                Err(e) => eprintln!("Error opening note: {}", e),  
            }
        }
    } else if args.get(1) == Some(&"delete".into()) {
        if args.get(2) != None {
            if args.get(2) == Some(&"category".into()) {
                match delete_category(args.get(3).unwrap()) {
                    Ok(_) => println!("Category deleted"),
                    Err(e) => eprintln!("Error deleting category: {}", e),
                }
            } else if args.get(2) == Some(&"note".into()) {
                if args.len() < 5 {
                    match delete_uncategorized_note(args.get(3).unwrap()) {
                        Ok(_) => println!("Note deleted"),
                        Err(e) => eprintln!("Error deleting note: {}", e),
                    }
                } else {
                    match delete_categorized_note(args.get(3).unwrap(), args.get(4).unwrap()) {
                        Ok(_) => println!("Note deleted"),
                        Err(e) => eprintln!("Error deleting note: {}", e),
                    }
                }
            } else { 
                println!("Please specify a note or category");
                std::process::exit(1);
            }
        } else {
            eprintln!("Name not provided");
            std::process::exit(1);
        }
    } else {
        eprintln!("Unknown command");
        std::process::exit(1);
    }
}