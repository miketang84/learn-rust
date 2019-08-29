
//  error[E0106]: missing lifetime specifier
//  --> src/main.rs:3:17
//   |
// 3 | fn teststr() -> &str {
//   |                 ^ help: consider giving it a 'static lifetime: `&'static`
//   |
//   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
//fn teststr() -> &str {
//    let a = "foobar";
//    println!("In function: {}", a);
//    
//    a
//}

fn fn1() -> &'static str {
    let a = "foobar";
    println!("In fn1: {}", a);
    
    a
}

//  error[E0106]: missing lifetime specifier
//   --> src/main.rs:24:13
//    |
// 24 | fn fn2() -> &str {
//    |             ^ help: consider giving it a 'static lifetime: `&'static`
//    |
//    = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// fn fn2() -> &str {
//    let a = "foobar".to_string();
//    println!("In fn2: {}", a);
   
//    &a
// }

// error[E0515]: cannot return reference to local variable `a`
//   --> src/main.rs:41:5
//    |
// 41 |     &a
//    |     ^^ returns a reference to data owned by the current function
// fn fn2<'a>() -> &'a str {
//     let a = "foobar".to_string();
//     println!("In fn2: {}", a);
    
//     &a
// }

fn fn2<'a>() -> String {
    let a = "foobar".to_string();
    println!("In fn2: {}", a);
    
    a
}


fn fn3(a: &str) {
    println!("In fn3: {}", a);
}


fn fn4(a: &str) {
    println!("In fn4: {}", a);
}

fn fn5(a: String){
    println!("In fn5: {}", a);
}

fn fn6(a: String) -> String {
    println!("In fn6: {}", a);

    a
}

fn main() {
    println!("In main 1: {}", fn1());
    println!("In main 2: {}", fn2());

    let a = "foobar";
    fn3(a);
    println!("In main 3: {}", a);

    let a = "foobar".to_string();
    fn4(&a);
    println!("In main 4: {}", a);

    fn5(a);
    
    // error[E0382]: borrow of moved value: `a`
    //   --> src/main.rs:84:31
    //    |
    // 79 |     let a = "foobar".to_string();
    //    |         - move occurs because `a` has type `std::string::String`, which does not implement the `Copy` trait
    // ...
    // 83 |     fn5(a);
    //    |         - value moved here
    // 84 |     println!("In main 5: {}", a);
    //    |                               ^ value borrowed here after move
    //    println!("In main 5: {}", a);

    let a = "foobar".to_string();
    let b = fn6(a);
    println!("In main 6: {}", b);
    
    // error[E0382]: borrow of moved value: `a`
    //    --> src/main.rs:106:31
    //     |
    // 102 |     let a = "foobar".to_string();
    //     |         - move occurs because `a` has type `std::string::String`, which does not implement the `Copy` trait
    // 103 |     let b = fn6(a);
    //     |                 - value moved here
    // ...
    // 106 |     println!("In main 6: {}", a);
    //     |                               ^ value borrowed here after move
    // println!("In main 6: {}", a);



}
