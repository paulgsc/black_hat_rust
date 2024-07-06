use std::rc::Rc;

fn main() {
	let pointer = Rc::new(1);

	{
		let second_pointer = Rc::clone(&pointer);
		println!("{}", *second_pointer);
	}

	println!("{}", *pointer);
}
