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

trait HasColour {
	fn is_red(&self) -> bool;
}

impl<K,V> HasColour for Option<TreeNode<K,V>> where K: Ord+Copy, V: Copy {
	fn is_red(&self) -> bool {
		match self {
			&None => false,
			&Some(ref node) => {
				node.is_red
			}
		}
	}
}

impl<K,V> HasColour for TreeNode<K,V> where K: Ord+Copy, V: Copy {
	fn is_red(&self) -> bool {
		self.is_red
	}
}

enum InsertionResultRecursion<K,V> where K: Ord+Copy, V: Copy {
	///Key already present
	Failure,
	///No additional steps necessary
	Standard(Rc<Option<TreeNode<K,V>>>),
	///Sibling recolouring necessary
	SiblingRecolouring(Rc<Option<TreeNode<K,V>>>),
}

impl<K,V> Tree<K,V> where K: Ord+Copy, V: Copy {
	pub fn new() -> Tree<K,V> {
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
		match Self::rec_insert(key, val, &self.root, None) {
			InsertionResultRecursion::Failure => None,
			InsertionResultRecursion::Standard(root) => {
				if root.is_red() {
					//red
					match root.deref() {
						&Some(ref old_node) => {
							Some(Tree {root: Rc::new(Some(TreeNode {
								is_red: false,
								key: old_node.key,
								val: old_node.val,
								left: old_node.left.clone(),
								right: old_node.right.clone()
							}))})
						},
						&None => panic!("assertion failure")
					}
				} else {
					//black
					Some(Tree {root: root})
				}
			},
			_ => panic!("Unexpected recursion result")
		}
	}

	fn rec_insert(key: K, val: V, current: &Rc<Option<TreeNode<K,V>>>, parent: Option<&Rc<Option<TreeNode<K,V>>>>) -> InsertionResultRecursion<K,V>{
		match current.deref() {
			&None => {
				//insert here
				InsertionResultRecursion::Standard(Rc::new(Some(TreeNode{
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
						match Self::rec_insert(key, val, &node.left, Some(current)) {
							InsertionResultRecursion::Failure => InsertionResultRecursion::Failure,
							InsertionResultRecursion::Standard(left_child) => {
								if node.is_red() {
									//red
									assert!(parent.is_some()); //red nodes always have parents
									if left_child.is_red() {
										//conflict
										//need to recolour self, parent and sibling
										InsertionResultRecursion::SiblingRecolouring(Rc::new(Some(TreeNode{
											is_red: false,
											key: node.key,
											val: node.val,
											left: left_child,
											right: node.right.clone()
										})))
									} else {
										unimplemented!()
									}
								} else {
									//black
									InsertionResultRecursion::Standard(Rc::new(Some(TreeNode{
										is_red: node.is_red,
										key: node.key,
										val: node.val,
										left: left_child,
										right: node.right.clone()
									})))
								}
							},
							InsertionResultRecursion::SiblingRecolouring(left_child) => {
								assert!(!left_child.is_red());
								assert!(node.right.is_red());
								assert!(!node.is_red());

								//clone and recolour the right child
								let right_child = {
									match node.right.deref() {
										&Some(ref old_right_child) => Rc::new(Some(TreeNode{
											is_red: false,
											key: old_right_child.key,
											val: old_right_child.val,
											left: old_right_child.left.clone(),
											right: old_right_child.right.clone()
										})),
										&None => panic!("assertion failure")
									}
								};

								//clone and recolour self
								InsertionResultRecursion::Standard(Rc::new(Some(TreeNode{
									is_red: true,
									key: node.key,
									val: node.val,
									left: left_child,
									right: right_child
								})))
							}
						}
					},
					Ordering::Greater => {
						unimplemented!()
					},
					Ordering::Equal => InsertionResultRecursion::Failure
				}
			}
		}
	}
}

fn main() {
    /*let tree = Some(TreeNode {
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

	let tree3 = tree2.insert(0, ()).unwrap();
	println!("{:#?}", tree3);

	let tree4 = Tree::new();
	let tree5 = tree4.insert(0, ()).unwrap();
	println!("{:#?}", tree5);*/

	let tree = Tree::new().insert(13, ()).unwrap().insert(8, ()).unwrap().insert(17, ()).unwrap();
	println!("{:#?}", tree);

	let tree2 = tree.insert(1, ()).unwrap();
	println!("{:#?}", tree2);
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
