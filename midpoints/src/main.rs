use std::collections::HashMap;

// Hashmap and Vector practice

fn main() {

	let mut num_list = vec![1, 1, 1, 2, 3, 5, 6, 67, 78, 8];

	println!("Mean: {:.2}", mean(&mut num_list));
	println!("Median: {}", median(&mut num_list));
	println!("Mode: {}", mode(&mut num_list));

}

// Mean using Vecs
fn mean(x: &mut Vec<i32>) -> f64 {
	let mut total: f64 = 0.0;
	let cap = x.capacity() as f64;
	for num in x {
		let another_num: f64 = *num as f64;
		total += another_num;
	}
	total / cap
}

// Median using Vecs
fn median(x: &mut Vec<i32>) -> i32 {
	x.sort();
	let mid = x.len() / 2;
	x[mid]
}

// Mode using HashMaps
fn mode(x: &mut Vec<i32>) -> i32 {
	let mut num_map: HashMap<_, _> = HashMap::new();

	for num in x {
		let count = num_map.entry(num).or_insert(0);
		*count += 1;
	}

	let mut median: i32 = 1;
	for (key, value) in &num_map {
		if *value > median {
			median = **key;
		}
	}

	match num_map.get(&median) {
		Some(x) => *x,
		None    => -1,
	}
}
