use name_value_pairs::*;
#[test]
fn t_is_unique_name() {
    let x:Vec<String> = vec!["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()];
    assert!(is_unique_name(&x, "Alice"));
}