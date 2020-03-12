struct Doubler<I> {
    iter: I,
}

impl<I> Iterator for Doubler<I> where
    I: Iterator,
    I::Item: std::ops::Add<Output=I::Item> + Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
	match self.iter.next() {
	    None => None,
	    Some(x) => Some(x + x),
	}
    }
}

fn sum(range: u32) -> u32 {
    (1..=range).fold(0, |sum, i| sum + i)
}

fn sum2<I>(iter: I) -> I::Item where
    I: Iterator,
    I::Item: std::ops::Add<Output=I::Item> + From<u8>,
{
    iter.fold(From::from(0u8), std::ops::Add::add)
}

fn main() {
    for i in (Doubler{iter: 1..11u32}) {
	print!("{} ", i);
    }

    println!();

    println!("Sum is {}", (1..11).fold(0, |sum, i| sum + i));
    println!("Sum is {} using sum helper", sum(10));
    println!("Sum is {} using sum good helper", sum2(0..11));
    println!("ALL DONE");

}
