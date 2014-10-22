struct Node<T: PartialEq>
{
    left: Box<Option<Node<T>>>,
    right: Box<Option<Node<T>>>,
    value: Option<T>,
}

impl<T: PartialEq> Node<T>
{
	fn put(&self, value: T) -> bool
	{
		let newNode = Node{
			left: box None,
			right: box None,
			value: Some(value),
		};
		true
	}
}

#[test]
fn nodeNew()
{
	println!("hello");
}

