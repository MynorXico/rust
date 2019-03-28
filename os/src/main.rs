extern crate rand;

use std::sync::mpsc::sync_channel;
use std::thread;
use rand::Rng;
use std::time::Duration;
use vulcano::sync::Semaphore;

fn main(){


	let max_baking_time = 3;
	let max_eating_time = 10;
	let cap = 3;
	
	let (tx, rx) = sync_channel(cap);
	let occ = 0;
	
	
	
	let handle = thread::spawn(move||{
		let mut counter = 0;
		while true {
			let n = rand::thread_rng().gen_range(0,max_baking_time);
			tx.try_send("cake").unwrap();
			println!("sent cake #{}", counter);
			counter = counter + 1;
			thread::sleep(Duration::from_secs(n));
		}
	});
	
	let mut order = 0;
	thread::sleep(Duration::from_secs(5));
	while true {
		let received = rx.try_recv().unwrap();
		println!("Got {} {}", received, order);
		let eating_time = rand::thread_rng().gen_range(1,max_baking_time);
		thread::sleep(Duration::from_secs(eating_time));
		order = order +1;
	}
}
	
