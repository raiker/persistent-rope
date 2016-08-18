use std::rc::Rc;
use std::cmp::{Ord, Ordering};
use std::ops::Deref;

//Red-Black Tree

pub enum Tree<K,V> where K: Ord {
	Leaf,
	Node {
		is_red: bool,
		key: K,
		val: V,
		left: Rc<Tree<K,V>>,
		right: Rc<Tree<K,V>>
	}
}

impl<K,V> Tree<K,V> where K: Ord {
	pub fn find(&self, search_key: K) -> Option<&V> {
		let mut current = self;
		
		loop {
			match current {
				&Tree::Leaf => return None,
				&Tree::Node{is_red: _, ref key, ref val, ref left, ref right} => {
					match search_key.cmp(&key) {
						Ordering::Less => current = left.deref(),
						Ordering::Greater => current = right.deref(),
						Ordering::Equal => return Some(&val)
					}
				}
			};
		}
	}
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
	use std::rc::Rc;
	use super::*;

	#[test]
	fn test_find(){
		let tree = Tree::Node {
			is_red: false,
			key: 1,
			val: (),
			left: Rc::new(Tree::Leaf),
			right: Rc::new(Tree::Node{
				is_red: true,
				key: 6,
				val: (),
				left: Rc::new(Tree::Leaf),
				right: Rc::new(Tree::Leaf)
			})
		};

		assert_eq!(tree.find(6), Some(&()));
		assert_eq!(tree.find(1), Some(&()));
		assert_eq!(tree.find(12), None);
	}
}
