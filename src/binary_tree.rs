//TODO : replace Box by a generic which could be either Ac Nac or Box
pub struct Node<T: PartialEq + PartialOrd>
{
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    value: Option<Box<T>>,
}

impl<T: PartialEq + PartialOrd> Node<T>
{
	//???? is it better to pass by reference (&) or by pointer (Ac/box)
	pub fn new_node(new_value: Box<T>) -> Node<T>
	{
		Node{
			left: None,
			right: None,
			value: Some(box *new_value),
		}
	}

	fn insert_value(emplacement: &mut Option<Box<Node<T>>>, new_value: Box<T>)
	{
		match *emplacement {
			Some(ref mut x) => {
				x.put(new_value);
			},None => {
				*emplacement = Some(box Node::new_node(new_value));
			}
		};
	}

	pub fn put(&mut self, new_value: Box<T>) -> bool
	{
		if self.value == None {
			self.value = Some(box *new_value);
		}else{
			match self.value {
				Some(ref mut x) => {
					if *new_value > **x {
						Node::insert_value(&mut self.right, new_value);
					}else{
						Node::insert_value(&mut self.left, new_value);
					}
				},None => {
					*self = Node::new_node(new_value);
				}
			};
		}
		true
	}
}

#[test]
fn node_new()
{
	println!("hello");
}

