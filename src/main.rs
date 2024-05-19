
use name_value_pairs::*;
fn main() {
    let pairs = build_pairs();
    let names = get_names(&pairs);
    let scores = get_scores(&pairs);
    for i in 0..names.len() {
        println!("{} {}", names[i], scores[i]);
    }
}
