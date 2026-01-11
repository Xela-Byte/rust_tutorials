use std::collections::HashMap;

pub fn test_hash_map() {
    let mut hash_map_result: HashMap<String, i16> = HashMap::new();
    hash_map_result.insert(String::from("Xela"), 22);
    hash_map_result.insert(String::from("Jeremy"), 24);
    hash_map_result.insert(String::from("Imade"), 78);
    hash_map_result.insert(String::from("Flourish"), 32);

    hash_map_result.remove("Xela");

    println!("Hashmap: {:#?}", hash_map_result)
}
