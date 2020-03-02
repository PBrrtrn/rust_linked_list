use std::mem;

pub struct List {
	head: Link,
}

impl List {
	pub fn new() -> Self {
		List { head: Link::Empty }
	}

	pub fn push(&mut self, elem: i32) {
		let new_node = Box::new(Node {
			elem: elem,
			next: mem::replace(&mut self.head, Link::Empty),
		});

		self.head = Link::More(new_node);
	}

	pub fn pop(&mut self) -> Option<i32> {
		match mem::replace(&mut self.head, Link::Empty) {
			Link::Empty => None,
			Link::More(node) => {
				self.head = node.next;
				Some(node.elem)
			} 
		}
	}
}

impl Drop for List {
	fn drop(&mut self) {
		let mut current_link = mem::replace(&mut self.head, Link::Empty);
		while let Link::More(mut boxed_node) = current_link {
			current_link = mem::replace(&mut boxed_node.next, Link::Empty);
		}
	}
}

enum Link {
	Empty,
	More(Box<Node>),
}

struct Node {
	elem: i32,
	next: Link,
}

#[cfg(test)]
mod test {
	use super::List;

	#[test]
	fn new_empty_linked_list() {
		let mut list = List::new();
		assert_eq!(list.pop(), None);
	}

	#[test]
	fn push_to_empty_list() {
		let mut list = List::new();
		list.push(10);
		assert_eq!(list.pop(), Some(10));
	}

	#[test]
	fn push_several_elements() {
		let mut list = List::new();
		list.push(10);
		list.push(15);
		list.push(2);
		list.push(64);
		list.push(12);
		list.push(11);
		list.push(54);
		list.push(2);
		assert_eq!(list.pop(), Some(2));
	}
}