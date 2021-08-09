use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    
    for elem in scores.into_iter() {
        println!("key: {}, value: {}", elem.0, elem.1)
    }
}
