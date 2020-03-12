trait AAA {
    type Item;

    fn test();
}

struct Foo;

impl AAA for Foo {
    type Item = String;

    fn test() {
	    println!("a test.");
    }

}

fn main() {
    //let f: Foo::Item= String::from("test foo");
    let f: <Foo as AAA>::Item= String::from("test foo");

    println!("{}", f);
}
