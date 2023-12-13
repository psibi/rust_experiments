use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    let value = 5;

    map.entry("poneyland").or_insert_with(|| value).add_entry(8);

    // map.entry("poneyland").or_insert_with(|| value);
    // map.entry("poneyland").or_insert_with(|| "foosball");

    println!("hashmap: {map:?}");
}
