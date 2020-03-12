trait AAA {
    type Item0;

    fn test();
}

impl<T> AAA for T {
    type Item0 = String;

    fn test() {
	println!("test it");
    }
}

trait BBB {
    type Item1;

    fn doit(&self);
}



struct Foo<T> {
    foo: T
}

impl<T> BBB for Foo<T>
where
    T: AAA
{
    type Item1 = T::Item0;

    fn doit(&self) {
	println!("just do it.");
    }
}

fn main() {
    let f = Foo::<u8>{
	foo: 100
    };

    f.doit();
}
