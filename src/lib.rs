use std::io;
pub fn is_unique_name(storage: &Vec<String>, name: &str) -> bool {
    for item in storage {
        let item: Vec<&str> = item.trim().split(' ').collect();
        if item[0] == name.trim() {
            println!("It's the same name twice. Skipped!");
            return false
        } else {
            continue;
        }
    }
    true
}
pub fn build_pairs() -> Vec<String> {
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
pub fn get_names(pairs: &Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for item in pairs {
        let temp: Vec<&str> = item.split(' ').collect();
        result.push(temp[0].to_string());
    }
    result
}
pub fn get_scores(pairs: &Vec<String>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for item in pairs {
        let temp: Vec<&str> = item.split(' ').collect();
        result.push(temp[1].trim().parse::<i32>().expect("failed"));
    }
    result
}
