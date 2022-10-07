use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error, Read};
use std::fs;
use std::thread;
use std::time::{Duration, Instant};
use std::thread::sleep;
fn main() -> Result<(), Error>{
	let path = "/dev/mymem";

	//let mut output = File::create(path)?;
	//Part 2
	//1 byte
	let timer = Instant::now();
	fs::write(path, "0")?;
	println!("Write one byte: {:?}", timer.elapsed());
	let timer = Instant::now();
	let mut buffer = String::new();
	let mut input = File::open(path)?;  //?
	input.read_to_string(&mut buffer);	
	println!("Read one byte: {:?}", timer.elapsed());
	
	//64 bytes
	let timer = Instant::now();
	let mut arr: [u8; 64] = [0; 64];
	for i in 0..64{
		arr[i] = 51;
	}
	let timer = Instant::now();
	fs::write(path, arr)?;
	let mut buffer = String::new();
	let mut input = File::open(path)?;  //?
	input.read_to_string(&mut buffer);	
	println!("Read and Write 64 bytes: {:?}", timer.elapsed());

	//1 KB
	let timer = Instant::now();
	let mut arr: [u8; 1024] = [0; 1024];
	for i in 0..1024{
		arr[i] = 51;
	}
	let timer = Instant::now();
	fs::write(path, arr)?;
	let mut buffer = String::new();
	let mut input = File::open(path)?;  //?
	input.read_to_string(&mut buffer);	
	println!("Read and Write 1KB: {:?}", timer.elapsed());

	//64 KB
	let timer = Instant::now();
	let mut arr: [u8; 65536] = [0; 65536];
	for i in 0..65536{
		arr[i] = 51;
	}
	let timer = Instant::now();
	fs::write(path, arr)?;
	let mut buffer = String::new();
	let mut input = File::open(path)?;  //?
	input.read_to_string(&mut buffer);	
	println!("Read and Write 64 KB: {:?}", timer.elapsed());

	//512 KB
	let timer = Instant::now();
	let mut arr: [u8; 524288] = [0; 524288];
	for i in 0..524288{
		arr[i] = 51;
	}
	let timer = Instant::now();
	fs::write(path, arr)?;
	let mut buffer = String::new();
	let mut input = File::open(path)?;  //?
	input.read_to_string(&mut buffer);	
	println!("Read and Write 512 KB: {:?}", timer.elapsed());

	
	//Part 3
	fs::write(path, "0")?;
	let handle = thread::spawn(move || {
		for w in 1..30{
			for n in 1..40{
				let mut input = File::open(path);  //?
				let mut buffer = String::new();
				input.expect("unable to open").read_to_string(&mut buffer); //?
				let increment = buffer.parse::<i32>().unwrap() + 1;
				let news: String = increment.to_string();
				fs::write(path, news);
				let mut input2 = File::open(path); //?
				let mut buffer2 = String::new();
				input2.expect("unable to open").read_to_string(&mut buffer2); //?
				println!("{:?}", buffer2);
			}
		}
	});	

	handle.join().unwrap();
	
///	let s: String = String::from_utf8_lossy(&buffer);
///	let mut newbuff = [0;10];
//	let new = input.read(&mut newbuff[..])?;
//	println!("{:?}", newbuff);
	//let buffered = BufReader::new(input);
	//for line in buffered.lines(){
	//	println!("{}", line?);
	//}
	
	Ok(())


}
