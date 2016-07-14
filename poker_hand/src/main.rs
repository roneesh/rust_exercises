#![feature(slice_patterns)]

extern crate itertools;
use itertools::Itertools;


fn main() {
    let mut cards: [usize; 5] = [2, 3, 4, 5, 7];

    cards.sort();

	let mut group_lengths: Vec<usize> = cards.iter().group_by(|elt| *elt)
		.map(|(_, group)| group.len()).collect();
	group_lengths.sort();

	let s = match group_lengths[..] {
		[1,1,1,1,1] => {
			if cards[4] - cards[0] == 4 {
				"straight"
			} else {
				"one of a kind"				
			}
		},
		[1,1,1,2] => "a pair",
		[1,2,2] => "two pair",
		[1,1,3] => "three of a kind",
		[2,3] => "full house",
		[1,4] => "four of a kind",
		_ => "",
	};

	println!("{:?}", s);
}