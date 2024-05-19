
use std::io;

fn build_pairs() -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    loop {
        println!("Enter name value pair or press 'q' to exit: ");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed");

        let name = temp.clone();
        let name:Vec<&str> = name.split(' ').collect();
        if temp.trim() != "q" && is_unique_name(&result, name[0]) {
            result.push(temp.trim().to_string());
        } else if temp.trim() == "q"{
            break result
        } else {
            continue;
        }
    }
}

fn get_names(pairs: &Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in pairs {
        let temp: Vec<&str> = item.split(' ').collect();
        result.push(temp[0].to_string());
    }
    result
}

fn get_scores(pairs: &Vec<String>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for item in pairs {
        let temp: Vec<&str> = item.split(' ').collect();
        result.push(temp[1].trim().parse::<u8>().expect("failed"));
    }
    result
}

fn is_unique_name(storage: &Vec<String>, name: &str) -> bool {
    for item in storage {
        let item: Vec<&str> = item.trim().split(' ').collect();
        if item[0] == name.trim() {
            println!("It's the same name twice. It's dropped");
            return false
        } else {
            continue;
        }
    }
    true
}

fn main() {
    let pairs = build_pairs();
    let names = get_names(&pairs);
    let scores = get_scores(&pairs);
    for i in 0..names.len() {
        println!("{} {}", names[i], scores[i]);
    }
}
