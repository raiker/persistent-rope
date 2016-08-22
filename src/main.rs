use std::rc::{Rc};
use std::cmp::{Ord, Ordering};
use std::ops::Deref;

//Red-Black Tree

#[derive(Debug)]
pub struct Tree<K,V> where K: Ord+Copy, V: Copy {
	root: Rc<Option<TreeNode<K,V>>>
}

#[derive(Debug)]
pub struct TreeNode<K,V> where K: Ord+Copy, V: Copy {
	is_red: bool,
	key: K,
	val: V,
	left: Rc<Option<TreeNode<K,V>>>,
	right: Rc<Option<TreeNode<K,V>>>
}

impl<K,V> Tree<K,V> where K: Ord+Copy, V: Copy {
	pub fn new() {
		Tree {root: Rc::new(None)}
	}

	pub fn find(&self, search_key: K) -> Option<&V> {
		let mut current = self.root.deref();
		
		loop {
			match current {
				&None => return None,
				&Some(ref node) => {
					match search_key.cmp(&node.key) {
						Ordering::Less => current = node.left.deref(),
						Ordering::Greater => current = node.right.deref(),
						Ordering::Equal => return Some(&node.val)
					}
				}
			};
			/*match current {
				&None => return None,
				&Some(ref node) => {
					match search_key.cmp(&node.key) {
						Ordering::Less => current = node.left.deref(),
						Ordering::Greater => current = node.right.deref(),
						Ordering::Equal => return Some(&node.val)
					}
				}
			};*/
		}
	}

	/*fn is_red(&self) -> bool {
		match self {
			&None => false,
			&Some(ref node) => {
				node.is_red
			}
		}
	}*/

	pub fn insert(&self, key: K, val: V) -> Option<Tree<K,V>> {
		Self::rec_insert(key, val, &self.root, None).map(|r| Tree {root: r})
	}

	fn rec_insert(key: K, val: V, current: &Rc<Option<TreeNode<K,V>>>, parent: Option<&Rc<Option<TreeNode<K,V>>>>) -> Option<Rc<Option<TreeNode<K,V>>>>{
		match current.deref() {
			&None => {
				//insert here
				Some(Rc::new(Some(TreeNode{
					is_red: true,
					key: key,
					val: val,
					left: Rc::new(None),
					right: Rc::new(None)
				})))
			},
			&Some(ref node) => {
				match key.cmp(&node.key) {
					Ordering::Less => {
						let new_left_child = Self::rec_insert(key, val, &node.left, Some(current));
						new_left_child.map(|c| {
							Rc::new(Some(TreeNode{
								is_red: node.is_red,
								key: node.key,
								val: node.val,
								left: c,
								right: node.right.clone()
							}))
						})
					},
					Ordering::Greater => {
						let new_right_child = Self::rec_insert(key, val, &node.right, Some(current));
						new_right_child.map(|c| {
							Rc::new(Some(TreeNode{
								is_red: node.is_red,
								key: node.key,
								val: node.val,
								left: node.left.clone(),
								right: c
							}))
						})
					},
					Ordering::Equal => None
				}
			}
		}
	}
}

fn main() {
    let tree = Some(TreeNode {
		is_red: false,
		key: 13,
		val: (),
		left: Rc::new(Some(TreeNode {
			is_red: true,
			key: 8,
			val: (),
			left: Rc::new(Some(TreeNode {
				is_red: false,
				key: 1,
				val: (),
				left: Rc::new(None),
				right: Rc::new(Some(TreeNode{
					is_red: true,
					key: 6,
					val: (),
					left: Rc::new(None),
					right: Rc::new(None)
				}))
			})),
			right: Rc::new(Some(TreeNode {
				is_red: false,
				key: 11,
				val: (),
				left: Rc::new(None),
				right: Rc::new(None)
			}))
		})),
		right: Rc::new(Some(TreeNode {
			is_red: true,
			key: 17,
			val: (),
			left: Rc::new(Some(TreeNode {
				is_red: false,
				key: 15,
				val: (),
				left: Rc::new(None),
				right: Rc::new(None)
			})),
			right: Rc::new(Some(TreeNode {
				is_red: false,
				key: 25,
				val: (),
				left: Rc::new(Some(TreeNode {
					is_red: true,
					key: 22,
					val: (),
					left: Rc::new(None),
					right: Rc::new(None)
				})),
				right: Rc::new(Some(TreeNode {
					is_red: true,
					key: 27,
					val: (),
					left: Rc::new(None),
					right: Rc::new(None)
				}))
			}))
		}))
	});

	let tree2 = Tree { root: Rc::new(Some(TreeNode {
		is_red: false,
		key: 1,
		val: (),
		left: Rc::new(None),
		right: Rc::new(Some(TreeNode{
			is_red: true,
			key: 6,
			val: (),
			left: Rc::new(None),
			right: Rc::new(None)
		}))
	}))};

	let tree3 = tree2.insert(0, ());
	println!("{:#?}", tree3);
}

#[cfg(test)]
mod tests {
	use std::rc::Rc;
	use super::*;

	#[test]
	fn test_find(){
		let tree = Tree{root: Rc::new(Some(TreeNode {
			is_red: false,
			key: 1,
			val: (),
			left: Rc::new(None),
			right: Rc::new(Some(TreeNode{
				is_red: true,
				key: 6,
				val: (),
				left: Rc::new(None),
				right: Rc::new(None)
			}))
		}))};

		assert_eq!(tree.find(6), Some(&()));
		assert_eq!(tree.find(1), Some(&()));
		assert_eq!(tree.find(12), None);
	}

	#[test]
	fn test_insert(){

	}
}
