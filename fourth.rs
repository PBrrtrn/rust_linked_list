use std::rc::Rc;
use std::cell::{Ref, RefCell};

pub struct List<T> {
	head: Link<T>,
	tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
	elem: T,
	next: Link<T>,
	prev: Link<T>,
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
	pub fn new() -> Self {
		List { 
			head: None,
			tail: None,
		}
	}

	pub fn push_front(&mut self, elem: T) {
		let new_head = Node::new(elem);
		match self.head.take() {
			Some(old_head) => {
				old_head.borrow_mut().prev = Some(new_head.clone());
				new_head.borrow_mut().next = Some(old_head);
				self.head = Some(new_head);
			}
			None => {
				self.head = Some(new_head.clone());
				self.tail = Some(new_head);
			}
		}
	}

	pub fn push_back(&mut self, elem: T) {
		let new_tail = Node::new(elem);
		match self.tail.take() {
			Some(old_tail) => {
				old_tail.borrow_mut().next = Some(new_tail.clone());
				new_tail.borrow_mut().prev = Some(old_tail);
				self.tail = Some(new_tail);
			}
			None => {
				self.head = Some(new_tail.clone());
				self.tail = Some(new_tail);
			}
		}
	}

	pub fn pop_front(&mut self) -> Option<T> {
		self.head.take().map(|old_head| {
			match old_head.borrow_mut().next.take() {
				Some(new_head) => {
					new_head.borrow_mut().prev.take();
					self.head = Some(new_head);
				}
				None => {
					self.tail.take();
				}
			}
			Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
		})
	}

	pub fn pop_back(&mut self) -> Option<T> {
		self.tail.take().map(|old_tail| {
			match old_tail.borrow_mut().prev.take() {
				Some(new_tail) => {
					new_tail.borrow_mut().next.take();
					self.tail = Some(new_tail);
				}
				None => {
					self.head.take();
				}
			}
			Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
		})
	}

	pub fn peek_front(&self) -> Option<Ref<T>> {
		self.head.as_ref().map(|node| {
			Ref::map(node.borrow(), |node| &node.elem)
		})
	}

	pub fn peek_back(&self) -> Option<Ref<T>> {
		self.tail.as_ref().map(|node| {
			Ref::map(node.borrow(), |node| &node.elem)
		})
	}

	pub fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
}

impl<T> Node<T> {
	fn new(elem: T) -> Rc<RefCell<Self>> {
		Rc::new(RefCell::new(Node {
			elem: elem,
			next: None,
			prev: None,
		}))
	}
}

impl<T> Drop for List<T> {
	fn drop(&mut self) {
		while self.pop_front().is_some() {}
	}
}

impl<T> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<T> {
		self.0.pop_front()
	}
}

impl<T> DoubleEndedIterator for IntoIter<T> {
	fn next_back(&mut self) -> Option<T> {
		self.0.pop_back()
	}
}

#[cfg(test)]
mod test {
	use super::List;

	#[test]
	fn fourth_push_and_pop_front() {
		let mut list = List::new();
		list.push_front(1);
		list.push_front(10);
		list.push_front(25);

		assert_eq!(list.pop_front(), Some(25));
		assert_eq!(list.pop_front(), Some(10));
		assert_eq!(list.pop_front(), Some(1));
	}

	#[test]
	fn fourth_push_and_pop_back() {
		let mut list = List::new();
		list.push_front(1);
		list.push_front(10);
		list.push_front(25);

		assert_eq!(list.pop_back(), Some(1));
		assert_eq!(list.pop_back(), Some(10));
		assert_eq!(list.pop_back(), Some(25));
	}

	#[test]
	fn fourth_peek_front() {
		let mut list = List::new();

		assert!(list.peek_front().is_none());

		list.push_front(1);
		list.push_front(10);
		list.push_front(25);

		assert_eq!(&*list.peek_front().unwrap(), &25);
	}

	#[test]
	fn fourth_peek_back() {
		let mut list = List::new();

		assert!(list.peek_front().is_none());

		list.push_back(1);
		list.push_back(10);
		list.push_back(25);

		assert_eq!(&*list.peek_back().unwrap(), &25);
	}

	#[test]
	fn fourth_into_iter() {
		let mut list = List::new();

		list.push_front(1);
		list.push_front(10);
		list.push_front(25);
		list.push_front(50);

		let mut into_iter = list.into_iter();
		assert_eq!(into_iter.next(), Some(50));
		assert_eq!(into_iter.next_back(), Some(1));
		assert_eq!(into_iter.next(), Some(25));
		assert_eq!(into_iter.next_back(), Some(10));
		assert_eq!(into_iter.next(), None);
		assert_eq!(into_iter.next_back(), None);
	}
}