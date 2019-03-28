extern crate rand;
extern crate std_semaphore;
use std::sync::mpsc::sync_channel;
use std::thread;
use rand::Rng;
use std::time::Duration;
use std_semaphore::Semaphore;
use std::sync::Arc;
fn main(){


	let max_baking_time = 10;
	let max_eating_time = 3;
	let cap = 3;
	
	let (tx, rx) = sync_channel(cap);
	let occ = 0;
	let empty_c = Arc::new(Semaphore::new(3));
	let data_c = Arc::new(Semaphore::new(0));
	let empty_p = empty_c.clone();
	let data_p = data_c.clone();
	
	let sem = Semaphore::new(5);
	
	

	
	let handle = thread::spawn(move||{

		let mut counter = 0;
		while true {

			let n = rand::thread_rng().gen_range(0,max_baking_time);
			empty_p.acquire();// wait on empty
			tx.try_send("cake").unwrap();
			data_p.release();// send signal to data
			println!("sent cake #{}", counter);
			counter = counter + 1;
			thread::sleep(Duration::from_secs(n));
		}
	});
	
	let mut order = 0;
	thread::sleep(Duration::from_secs(5));
	while true {
		data_c.acquire();// wait on data
		let received = rx.try_recv().unwrap();
		empty_c.release();// send signal 
		println!("Got {} {}", received, order);
		let eating_time = rand::thread_rng().gen_range(1,max_baking_time);
		thread::sleep(Duration::from_secs(eating_time));
		order = order +1;
	}
}
	
