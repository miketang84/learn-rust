use std::process::Command;

fn main() {
    let mut finished = false;
    let mut i = 0;
    while !finished {
	i += 1;
	println!(">>>>> try {} time..", i);
	let status = Command::new("./scpr.sh")
	    .status()
	    .expect("Failed to execute command");

	//println!("Hello, world! {:?}", status.success());

	finished = status.success();
	std::thread::sleep(std::time::Duration::new(2, 0));
    }

    println!("Success and exit.")
}
