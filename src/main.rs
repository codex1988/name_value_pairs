use std::io::{self, Write};
use name_value_pairs::*;

fn main() {
    let pairs = build_pairs();
    let names = get_names(&pairs);
    let scores = get_scores(&pairs);
    for i in 0..names.len() {
        println!("{} {}", names[i], scores[i]);
    }

    let request = {
        print!("Search name:>>> ");
        io::stdout().flush().unwrap();

        let mut result = String::new();
        io::stdin()
            .read_line(&mut result)
            .expect("failed");

        result.trim().to_string()
    };

    for i in 0..names.len() {
        if names[i] == request {
            println!("name {}, value {}", request, scores[i]);
            break;
        } else if i == names.len() - 1 {
            println!("Name not found");
            break;
        } else {
            continue;
        }
    }
    
    let request: i32 = {
        print ! ("Search value:>>> ");
        io::stdout().flush().unwrap();

        let mut result = String::new();
        io::stdin()
            .read_line( & mut result)
            .expect("failed");

        result.trim().parse().unwrap()
    };

    for i in 0..scores.len() {
        if scores[i] == request {
            println!("value {}, name {}", scores[i], names[i]);
        } else if i == names.len() - 1 {
            println!("Score not found");
            break;
        } else {
            continue;
        }
    }
    
}
