use std::collections::HashMap;

fn process_or_default(key: char, map: &mut HashMap<char, String>) {
    match map.get_mut(&key) {
        Some(value) => value.push_str(", world!"),
        None => {
            map.insert(key, Default::default());
        }
    }
}

fn main() {
    let mut map = HashMap::new();
    map.insert('h', "Hello".to_string());
    process_or_default('h', &mut map);
    println!("{:?}", map);
}