extern crate rand;
extern crate std_semaphore;

use std::sync::mpsc::sync_channel; // multiple producer, single consumer
use std::thread;
use rand::Rng;
use std::time::Duration;
use std_semaphore::Semaphore;
use std::sync::Arc;

fn main(){
	

	//let tx1 = mpsc::Sender::clone(&tx);
	
	// Semáforos (empty, data)

	let data_c = Arc::new(Semaphore::new(0));
	
	// Propiedades del consumidor
	let max_eating_time = 3;
	
	// Propiedades del productor
	let max_baking_time = 10;
	let cap = 3;
	// Canal con tamaño definido (buffer)
	let (tx, rx) = sync_channel(cap); // tx -> transmisor, rx -> receptor
	let cap_u = cap as isize;
		let empty_c = Arc::new(Semaphore::new(cap_u));
	
	// Mapeo de semáforos sobre heap
	let empty_p = empty_c.clone();
	let data_p = data_c.clone();
	
	
	// Proceso de restaurantes
	println!("All you can eat ¡TACOS!");
	
	// Nuevo thread
	let handle = thread::spawn(move||{ // Asignar variable permite hacer "join"
		let mut counter = 0;	
		loop {
			let t = rand::thread_rng().gen_range(0,max_baking_time);
			//empty_p.acquire();	// revisa que hayan espacios vacíos en buffer
			tx.try_send("taco").unwrap();	// envía un taco
			//data_p.release();	// aumenta contador de espacio ocupados
			println!("Se preparó taco #{}", counter);
			counter = counter + 1;
			thread::sleep(Duration::from_secs(t)); // tiempo que tarda en producir
		}
	});
	
	let mut order = 0;
	thread::sleep(Duration::from_secs(5));	// Se da tiempo inicial al productor

	loop {
		//data_c.acquire();	// verifica que hayan datos en el buffer
		let received = rx.try_recv().unwrap();
		//empty_c.release();	// aumenta contador de espacios vacíos
		println!("Se consumió {} #{}", received, order);
		// tiempo que tarda en comer
		let eating_time = rand::thread_rng().gen_range(1,max_eating_time);
		thread::sleep(Duration::from_secs(eating_time));
		order = order +1;
	}
	
	handle.join().unwrap();
}
	
