use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
	for i in 0..args.len() {
		if args[i].parse::<u128>().is_ok() {
			println!("{:?}", hailstone(args[i].parse::<u128>().unwrap()));
		}
	}
}

fn hailstone(mut num: u128) -> Vec<u128> {
	let mut list: Vec<u128> = vec!();
	list.push(num);
	while num != 1 {
		if num % 2 == 0 {
			num /= 2;
			list.push(num);
		} else {
			num = (num * 3) + 1;
			list.push(num);
			num /= 2;
			list.push(num);
		}
	}
	list
}