type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
	head: Link<T>,
}

pub struct Iter<'a,T> {
	next: Option<&'a Node<T>>,
}

pub struct IterMut<'a,T> {
	next: Option<&'a mut Node<T>>,
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
	pub fn new() -> Self {
		List { head: None }
	}

	pub fn push(&mut self, elem: T) {
		let new_node = Box::new(Node {
			elem: elem,
			next: self.head.take(),
		});

		self.head = Some(new_node);
	}

	pub fn pop(&mut self) -> Option<T> {
		self.head.take().map(|node| { // Option.map(|x| ... ) is syntactic sugar for matching an Option.
			self.head = node.next;		  // None case is abbreviated in the map method, so we only need to
			node.elem 	 								// specify behavior for the Some case
		})
	}

	pub fn peek(&self) -> Option<&T> {
		self.head.as_ref().map(|node| {
			&node.elem
		})
	}

	pub fn mutable_peek(&mut self) -> Option<&mut T> {
		self.head.as_mut().map(|node|{
			&mut node.elem
		})
	}

	pub fn iter(&self) -> Iter<T> {
		Iter { next: self.head.as_ref().map(|node| &**node) }
	}

	pub fn iter_mut(&self) -> IterMut<'_, T> {
		IterMut { next: self.head.as_mut().map(|node| &mut **node) }
	}

	pub fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
}

impl<T> Drop for List<T> {
	fn drop(&mut self) {
		let mut current_link = self.head.take();
		while let Some(mut boxed_node) = current_link {
			current_link = boxed_node.next.take();
		}
	}
}

impl<'a,T> Iterator for Iter<'a,T> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		self.next.map(|node| {
			self.next = node.next.as_ref().map(|node| &**node);
			&node.elem
		})
	}
}

impl<'a,T> Iterator for IterMut<'a,T> {
	type Item = &'a mut T;

	fn next(&mut self) -> Option<Self::Item> {
		self.next.map(|node| {
			self.next = node.next.as_mut().map(|node| &mut **node);
			&mut node.elem
		})
	}
}

impl<T> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		self.0.pop()
	}
}

struct Node<T> {
	elem: T,
	next: Link<T>,
}

#[cfg(test)]
mod test {
	use super::List;

	#[test]
	fn second_new_empty_linked_list() {
		let mut list: List<i32> = List::new();
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

	#[test]
	fn second_peek() {
		let mut list = List::new();
		list.push(10);
		let top = list.peek();
		assert_eq!(top, Some(&10));
	}

	#[test]
	fn second_mutable_peek() {
		let mut list = List::new();
		list.push(10);
		let top = list.mutable_peek().unwrap();
		*top += 10;
		assert_eq!(list.peek(), Some(&20));
	}

	#[test]
	fn second_into_iter() {
		let mut list = List::new();
		list.push(1); list.push(10); list.push(25);

		let mut iter = list.into_iter();

		assert_eq!(iter.next(), Some(25));
		assert_eq!(iter.next(), Some(10));
		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), None);
	}

	#[test]
	fn second_iter() {
		let mut list = List::new();
		list.push(1); list.push(10); list.push(25);

		let mut iter = list.iter();

		assert_eq!(iter.next(), Some(&25));
		assert_eq!(iter.next(), Some(&10));
		assert_eq!(iter.next(), Some(&1));
	}
}