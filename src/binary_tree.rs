extern crate core;
use core::fmt::Show;
use core::fmt::Formatter;
use std::fmt;
use core::cmp::PartialEq;

//TODO : replace Box by a generic which could be either Ac Nac or Box
pub struct Node<T: PartialEq + PartialOrd + Show>
{
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    value: Option<Box<T>>,
}

impl<T: PartialEq + PartialOrd + Show> Node<T>
{
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

impl<T: PartialEq + PartialOrd + Show> Show for Node<T>
{
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::FormatError>
	{
		match self.left {
			Some(ref x) => {x.fmt(f);},
			None => {}
		};
		match self.value {
			Some(ref x) => {
				write!(f, "{}", x.to_string().as_slice());
			},
			None => {}
		};
		match self.right {
			Some(ref x) => {x.fmt(f);},
			None => {}
		};
		Ok(())
	}
}

#[test]
fn node_new()
{
	let mut tree = Node::new_node(box 10i);
	tree.put(box 5i);
	tree.put(box 15i);
	tree.put(box 12i);
	tree.put(box 8i);
	tree.put(box 1i);
	tree.put(box 20i);
	let ret_str = tree.to_string();
	println!("{}", tree);
	assert_eq!(ret_str.as_slice(), "15810121520");
}
