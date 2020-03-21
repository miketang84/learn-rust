fn main() {
    loop {

	for i in 1..11 {
	    println!("num: {}", i)
	}

	nix::unistd::sleep(1);

    }
}
