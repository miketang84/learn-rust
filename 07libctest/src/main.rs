// use std::thread;

// fn main() {
//     let child = thread::spawn(move || {
//         println!("Hello, I am a new rust thread!");
//     });

//     let res = child.join();
//     println!("{:?}", res);

//     println!("Hello, I am main thread!");
// }


fn main() {
    unsafe {
        let pid = libc::fork();

        if pid > 0 {
            println!("Hello, I am parent thread: {}", libc::getpid());
        }
        else if pid == 0 {
            println!("Hello, I am child thread: {}", libc::getpid());
            println!("My parent thread: {}", libc::getppid());
        }
        else {
            println!("Fork creation failed!");
        }
    }
}
