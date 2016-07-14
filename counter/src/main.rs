use std::collections::HashMap;

fn main() {
    let list = "aaabbcddddzzkkk";
	let v: Vec<&str> = list.split("").collect();
    let mut letter_counts = HashMap::new();

    for letter in v {
    	if !letter_counts.contains_key(letter) {
    		letter_counts.insert(letter, 1);
    	} else if letter_counts.contains_key(letter) {
    		let current_count = letter_counts[letter];
    		letter_counts.insert(letter, current_count + 1);
    	}
    }

    letter_counts.remove(&"")
    letter_counts.remove(&" ")

    println!("{:?}", letter_counts);
}