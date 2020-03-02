use std::mem;

type Link = Option<Box<Node>>;

pub struct List {
	head: Link,
}

impl List {
	pub fn new() -> Self {
		List { head: None }
	}

	pub fn push(&mut self, elem: i32) {
		let new_node = Box::new(Node {
			elem: elem,
			next: self.head.take(),
		});

		self.head = Some(new_node);
	}

	pub fn pop(&mut self) -> Option<i32> {
		self.head.take().map(|node| { // Option.map(|x| ... ) is syntactic sugar for matching an Option
			self.head = node.next;		  // None case is abbreviated in the map method, so we only need to
			node.elem 	 								// specify behavior for the Some case
		})
	}
}

impl Drop for List {
	fn drop(&mut self) {
		let mut current_link = self.head.take();
		while let Some(mut boxed_node) = current_link {
			current_link = boxed_node.next.take();
		}
	}
}


struct Node {
	elem: i32,
	next: Link,
}

#[cfg(test)]
mod test {
	use super::List;

	#[test]
	fn second_new_empty_linked_list() {
		let mut list = List::new();
		assert_eq!(list.pop(), None);
	}

	#[test]
	fn second_push_to_empty_list() {
		let mut list = List::new();
		list.push(10);
		assert_eq!(list.pop(), Some(10));
	}

	#[test]
	fn second_push_several_elements() {
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