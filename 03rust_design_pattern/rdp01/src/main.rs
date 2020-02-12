use std::ops::Deref;

trait TraitFoo {
    fn foo(&self);
}

struct A;

impl TraitFoo for A {
    fn foo(&self) {
	println!("Huh, I'm foo!");
    }
}

struct B<T> {
    behavior: T
}

impl<T> B<T> {
    pub fn new(behavior: T) -> B<T> {
	B {
	    behavior
	}
    }
}

impl<T> Deref for B<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
	&self.behavior
    }
}


trait TraitBar {
    fn bar(&self);
}

struct C;

impl TraitBar for C {
    fn bar(&self) {
	println!("Huh, I'm bar!");
    }
}


fn main() {
    let a = A;
    let b = B::new(a);
    b.foo();

    let c = C;
    let b = B::new(c);
    b.bar();
}
