use std::ptr;

pub struct List<T> {
	head: Link<T>,
	tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
	elem: T,
	next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
	next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
	next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
	pub fn new() -> Self {
		List { head: None, tail: ptr::null_mut() }
	}

	pub fn push(&mut self, elem: T) {
		let mut new_tail = Box::new(Node {
			elem: elem,
			next: None,
		});
		let raw_tail: *mut _ = &mut *new_tail;

		if self.tail.is_null() {
			self.head = Some(new_tail);
		} else {
			unsafe {
				(*(self.tail)).next = Some(new_tail);
			}
		}
		self.tail = raw_tail;
	}

	pub fn pop(&mut self) -> Option<T> {
		self.head.take().map(|head| {
			let head = *head;
			self.head = head.next;

			if self.head.is_none() {
				self.tail = ptr::null_mut();
			}
		head.elem
		})
	}

	pub fn peek(&self) -> Option<&T> {
		self.head.as_ref().map(|node| {
			&node.elem
		})
	}

	pub fn peek_mut(&mut self) -> Option<&mut T> {
		self.head.as_mut().map(|node| {
			&mut node.elem
		})
	}

	pub fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}

	pub fn iter(&self) -> Iter<'_, T> {
		Iter { next: self.head.as_ref().map(|node| &**node) }
	}

	pub fn iter_mut(&mut self) -> IterMut<'_, T> {
		IterMut { next: self.head.as_mut().map(|node| &mut **node) }
	}
}

impl<T> Drop for List<T> {
	fn drop(&mut self) {
		let mut cur_link = self.head.take();
		while let Some(mut boxed_node) = cur_link {
			cur_link = boxed_node.next.take();
		}
	}
}

impl<T> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		self.0.pop()
	}
}

impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.next.map(|node| {
			self.next = node.next.as_ref().map(|node| &**node);
			&node.elem
		})
	}
}

impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;
	fn next(&mut self) -> Option<Self::Item> {
		self.next.take().map(|node| {
			self.next = node.next.as_mut().map(|node| &mut **node);
			&mut node.elem
		})
	}
}

#[cfg(test)]
mod test {
	use super::List;
	
	#[test]
	fn fifth_basics() {
		let mut list = List::new();
		assert_eq!(list.pop(), None);

		list.push(1);
		list.push(10);
		list.push(25);
		assert_eq!(list.pop(), Some(1));
		assert_eq!(list.pop(), Some(10));

		list.push(2);
		list.push(20);
		list.push(50);
		assert_eq!(list.pop(), Some(25));
		assert_eq!(list.pop(), Some(2));
		assert_eq!(list.pop(), Some(20));
		assert_eq!(list.pop(), Some(50));

		assert_eq!(list.pop(), None);
	}
}