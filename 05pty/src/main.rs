use nix::pty;
use nix::pty::OpenptyResult;
use nix::unistd::ForkResult;
use std::os::unix::io::RawFd;
use nix::sys::select::FdSet;

fn main() {

    let ws = pty::Winsize {
	ws_row: 80,
	ws_col: 24,
	ws_xpixel: 0,
	ws_ypixel: 0,
    };

    let opr: OpenptyResult = pty::openpty(Some(&ws), None).unwrap();
    // let (master, slave0): (RawFd, RawFd) = (opr.master.clone(), opr.slave.clone());
    println!("{:?}", opr);

    match nix::unistd::fork() {
	Ok(ForkResult::Parent { child, .. }) => {
	    println!("Continuing execution in parent process, new child has pid: {}", child);
	    //let _ = nix::unistd::close(opr.slave);
	    println!("leave fork parent branch.");
	}
	Ok(ForkResult::Child) => {
	    println!("I'm a new child process");
	    let _ = nix::unistd::sleep(1);
	    println!("ready to replace image.");

	    // let a: RawFd = slave0.clone();
	    let _ = nix::unistd::setsid();
	    let _ = nix::unistd::dup2(opr.slave, 0);
	    let _ = nix::unistd::dup2(opr.slave, 1);
	    let _ = nix::unistd::dup2(opr.slave, 2);
	    unsafe {
		libc::ioctl(opr.slave, libc::TIOCSCTTY);
	    }
	    let _ = nix::unistd::close(opr.slave);
	    let _ = nix::unistd::close(opr.master);

	    let cstr = std::ffi::CString::new("./p10").unwrap();
	    let _ = nix::unistd::execvp(&cstr, &[]);

	    // loop {
	    //	let mut i: usize = 0;
	    //	// println!("from chind process output: {}", i);
	    //	i += 1;
	    //	nix::unistd::write(opr.slave, b"test.");
	    //	nix::unistd::sleep(1);
	    // }


	}
	Err(_) => println!("Fork failed"),
    }

    // println!("Parent start to wait 5 seconds!");
    // let _ = nix::unistd::sleep(5);

    let mut set = FdSet::new();


    loop {
	set.clear();
	set.insert(opr.master);

	let mut tv = nix::sys::time::TimeVal::from(libc::timeval {
	    tv_sec: 0,
	    tv_usec: 5
	});
	let r = nix::sys::select::select(
	    Some(opr.master + 1),
	    Some(&mut set),
	    None,
	    None,
	    None,  //Some(&mut tv)
	);
	println!("select result: {:?}", r);

	println!("in set? {}", set.contains(opr.master));
	let mut eof = false;
	while !eof {
	    let mut buf = [0u8;1024];
	    //let mut buf: Vec<u8> = Vec::with_capacity(64);
	    let nread = nix::unistd::read(opr.master, &mut buf).unwrap();
	    if nread == 0 {
		eof = true;
	    }

	    println!("received:{}: {}", nread, String::from_utf8_lossy(&buf));
	    let _ = nix::unistd::sleep(1);
	}

    }


    println!("parent process exit.");

}
