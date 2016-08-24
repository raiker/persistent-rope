use std::rc::{Rc};
use std::cmp::{Ord, Ordering};

fn ptr_eq<T>(a: *const T, b: *const T) -> bool { a == b }

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

impl<K,V> TreeNode<K,V> where K: Ord+Copy, V: Copy {
	fn recolour(&self, is_red: bool, left: Rc<Option<TreeNode<K,V>>>, right: Rc<Option<TreeNode<K,V>>>) -> Rc<Option<TreeNode<K,V>>> {
		Rc::new(Some(TreeNode{
			is_red: is_red,
			key: self.key,
			val: self.val,
			left: left,
			right: right
		}))
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
	///Child is red, new grandchild on left also red
	DoubleRedLeft(Rc<Option<TreeNode<K,V>>>),
	///Child is red, new grandchild on right also red
	DoubleRedRight(Rc<Option<TreeNode<K,V>>>),
}

impl<K,V> Tree<K,V> where K: Ord+Copy, V: Copy {
	pub fn new() -> Tree<K,V> {
		Tree {root: Rc::new(None)}
	}

	pub fn find(&self, search_key: K) -> Option<&V> {
		let mut current = self.root.as_ref();
		
		loop {
			match current {
				&None => return None,
				&Some(ref node) => {
					match search_key.cmp(&node.key) {
						Ordering::Less => current = node.left.as_ref(),
						Ordering::Greater => current = node.right.as_ref(),
						Ordering::Equal => return Some(&node.val)
					}
				}
			};
			/*match current {
				&None => return None,
				&Some(ref node) => {
					match search_key.cmp(&node.key) {
						Ordering::Less => current = node.left.as_ref(),
						Ordering::Greater => current = node.right.as_ref(),
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
					let old_node = root.as_ref().as_ref().unwrap();
					Some(Tree {root: old_node.recolour(false, old_node.left.clone(), old_node.right.clone())})
				} else {
					//black
					Some(Tree {root: root})
				}
			},
			_ => panic!("Unexpected recursion result")
		}
	}

	fn sibling<'a>(current: &'a Rc<Option<TreeNode<K,V>>>, parent: &'a Rc<Option<TreeNode<K,V>>>) -> &'a Rc<Option<TreeNode<K,V>>> {
		match parent.as_ref() {
			&None => panic!("assertion failure"),
			&Some(ref p) => {
				if ptr_eq(p.left.as_ref(), current.as_ref()) {
					&p.right
				} else if ptr_eq(p.right.as_ref(), current.as_ref()) {
					&p.left
				} else {
					panic!("assertion failure")
				}
			}
		}
	} 

	fn rec_insert(key: K, val: V, current: &Rc<Option<TreeNode<K,V>>>, parent: Option<&Rc<Option<TreeNode<K,V>>>>) -> InsertionResultRecursion<K,V>{
		match current.as_ref() {
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
									InsertionResultRecursion::DoubleRedLeft(left_child)
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
							InsertionResultRecursion::DoubleRedLeft(left_grandchild) => {
								unimplemented!()
							},
							InsertionResultRecursion::DoubleRedRight(right_grandchild) => {
								unimplemented!()
							}
						}
					},
					Ordering::Greater => {
						match Self::rec_insert(key, val, &node.right, Some(current)) {
							InsertionResultRecursion::Failure => InsertionResultRecursion::Failure,
							InsertionResultRecursion::Standard(right_child) => {
								if node.is_red() {
									//red
									InsertionResultRecursion::DoubleRedRight(right_child)
								} else {
									//black
									InsertionResultRecursion::Standard(node.recolour(node.is_red, node.left.clone(), right_child))
								}
							},
							InsertionResultRecursion::DoubleRedLeft(left_grandchild) => {
								assert!(!node.is_red());

								if node.left.is_red() {
									//   B (self)
									//  / \
									// R   R
									//    /
									//   R
									
									// recolour self and both children
									let old_left = node.left.as_ref().as_ref().unwrap();
									let old_right = node.right.as_ref().as_ref().unwrap();

									let new_left = old_left.recolour(false, old_left.left.clone(), old_left.right.clone());
									let new_right = old_right.recolour(false, left_grandchild, old_right.right.clone());

									InsertionResultRecursion::Standard(node.recolour(true, new_left, new_right))
								} else {
									//   B (self)
									//  / \
									// B   R
									//    /
									//   R
									
									// reorder and recolour
									let old_right = node.right.as_ref().as_ref().unwrap();
									let old_grandchild = left_grandchild.as_ref().as_ref().unwrap();
									let new_left = node.recolour(true, node.left.clone(), old_grandchild.left.clone());
									let new_right = old_right.recolour(true, old_grandchild.right.clone(), old_right.right.clone());
									InsertionResultRecursion::Standard(old_grandchild.recolour(false, new_left, new_right))
								}
							},
							InsertionResultRecursion::DoubleRedRight(right_grandchild) => {
								unimplemented!()
							}
						}
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
	use super::HasColour;

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
	fn test_insert_case1(){
		let start = Tree::new();
		let test = start.insert(5, ()).unwrap();
		//  5
		assert_eq!(test.root.is_some(), true);

		let test_root = test.root.as_ref().as_ref().unwrap();
		assert_eq!(test_root.is_red(), false);
		assert_eq!(test_root.key, 5);
		assert_eq!(test_root.left.is_some(), false);
		assert_eq!(test_root.right.is_some(), false);
	}

	#[test]
	fn test_insert_case2_left() {
		let start = Tree::new().insert(5, ()).unwrap();
		let test = start.insert(4, ()).unwrap();
		//    5
		//   /
		//  4
		assert_eq!(test.root.is_some(), true);

		let ref test_root = test.root.as_ref().as_ref().unwrap();
		assert_eq!(test_root.is_red(), false);
		assert_eq!(test_root.key, 5);
		assert_eq!(test_root.left.is_some(), true);
		assert_eq!(test_root.right.is_some(), false);

		let ref test_left = test_root.left.as_ref().as_ref().unwrap();
		assert_eq!(test_left.is_red(), true);
		assert_eq!(test_left.key, 4);
		assert_eq!(test_left.left.is_some(), false);
		assert_eq!(test_left.right.is_some(), false);
	}

	#[test]
	fn test_insert_case2_right() {
		let start = Tree::new().insert(5, ()).unwrap();
		let test = start.insert(6, ()).unwrap();
		//  5
		//   \
		//    6
		assert_eq!(test.root.is_some(), true);

		let ref test_root = test.root.as_ref().as_ref().unwrap();
		assert_eq!(test_root.is_red(), false);
		assert_eq!(test_root.key, 5);
		assert_eq!(test_root.left.is_some(), false);
		assert_eq!(test_root.right.is_some(), true);

		let ref test_right = test_root.right.as_ref().as_ref().unwrap();
		assert_eq!(test_right.is_red(), true);
		assert_eq!(test_right.key, 6);
		assert_eq!(test_right.left.is_some(), false);
		assert_eq!(test_right.right.is_some(), false);
	}

	#[test]
	fn test_insert_case5_left() {
		//      B
		//     /
		//    R
		//   /
		//  R
		let start = Tree::new().insert(6, ()).unwrap().insert(5, ()).unwrap();
		let test = start.insert(4, ()).unwrap();
		//    5
		//   / \
		//  4   6
		assert_eq!(test.root.is_some(), true);

		let ref test_root = test.root.as_ref().as_ref().unwrap();
		assert_eq!(test_root.is_red(), false);
		assert_eq!(test_root.key, 5);
		assert_eq!(test_root.left.is_some(), true);
		assert_eq!(test_root.right.is_some(), true);

		let ref test_left = test_root.left.as_ref().as_ref().unwrap();
		assert_eq!(test_left.is_red(), true);
		assert_eq!(test_left.key, 4);
		assert_eq!(test_left.left.is_some(), false);
		assert_eq!(test_left.right.is_some(), false);

		let ref test_right = test_root.right.as_ref().as_ref().unwrap();
		assert_eq!(test_right.is_red(), true);
		assert_eq!(test_right.key, 6);
		assert_eq!(test_right.left.is_some(), false);
		assert_eq!(test_right.right.is_some(), false);
	}

	#[test]
	fn test_insert_case5_right() {
		//  B
		//   \
		//    R
		//     \
		//      R
		let start = Tree::new().insert(4, ()).unwrap().insert(5, ()).unwrap();
		let test = start.insert(6, ()).unwrap();
		//    5
		//   / \
		//  4   6
		assert_eq!(test.root.is_some(), true);

		let ref test_root = test.root.as_ref().as_ref().unwrap();
		assert_eq!(test_root.is_red(), false);
		assert_eq!(test_root.key, 5);
		assert_eq!(test_root.left.is_some(), true);
		assert_eq!(test_root.right.is_some(), true);

		let ref test_left = test_root.left.as_ref().as_ref().unwrap();
		assert_eq!(test_left.is_red(), true);
		assert_eq!(test_left.key, 4);
		assert_eq!(test_left.left.is_some(), false);
		assert_eq!(test_left.right.is_some(), false);

		let ref test_right = test_root.right.as_ref().as_ref().unwrap();
		assert_eq!(test_right.is_red(), true);
		assert_eq!(test_right.key, 6);
		assert_eq!(test_right.left.is_some(), false);
		assert_eq!(test_right.right.is_some(), false);
	}

	#[test]
	fn test_insert_case4_left() {
		//    B
		//   /
		//  R
		//   \
		//    R
		let start = Tree::new().insert(6, ()).unwrap().insert(4, ()).unwrap();
		let test = start.insert(5, ()).unwrap();
		//    5
		//   / \
		//  4   6
		assert_eq!(test.root.is_some(), true);

		let ref test_root = test.root.as_ref().as_ref().unwrap();
		assert_eq!(test_root.is_red(), false);
		assert_eq!(test_root.key, 5);
		assert_eq!(test_root.left.is_some(), true);
		assert_eq!(test_root.right.is_some(), true);

		let ref test_left = test_root.left.as_ref().as_ref().unwrap();
		assert_eq!(test_left.is_red(), true);
		assert_eq!(test_left.key, 4);
		assert_eq!(test_left.left.is_some(), false);
		assert_eq!(test_left.right.is_some(), false);

		let ref test_right = test_root.right.as_ref().as_ref().unwrap();
		assert_eq!(test_right.is_red(), true);
		assert_eq!(test_right.key, 6);
		assert_eq!(test_right.left.is_some(), false);
		assert_eq!(test_right.right.is_some(), false);
	}

	#[test]
	fn test_insert_case4_right() {
		//  B
		//   \
		//    R
		//   /
		//  R
		let start = Tree::new().insert(4, ()).unwrap().insert(6, ()).unwrap();
		let test = start.insert(5, ()).unwrap();
		//    5
		//   / \
		//  4   6
		assert_eq!(test.root.is_some(), true);

		let ref test_root = test.root.as_ref().as_ref().unwrap();
		assert_eq!(test_root.is_red(), false);
		assert_eq!(test_root.key, 5);
		assert_eq!(test_root.left.is_some(), true);
		assert_eq!(test_root.right.is_some(), true);

		let ref test_left = test_root.left.as_ref().as_ref().unwrap();
		assert_eq!(test_left.is_red(), true);
		assert_eq!(test_left.key, 4);
		assert_eq!(test_left.left.is_some(), false);
		assert_eq!(test_left.right.is_some(), false);

		let ref test_right = test_root.right.as_ref().as_ref().unwrap();
		assert_eq!(test_right.is_red(), true);
		assert_eq!(test_right.key, 6);
		assert_eq!(test_right.left.is_some(), false);
		assert_eq!(test_right.right.is_some(), false);
	}

	#[test]
	fn test_insert_case3_left_a() {
		//      B
		//     / \
		//    R   R
		//   /
		//  R
		let start = Tree::new().insert(5, ()).unwrap().insert(4, ()).unwrap().insert(6, ()).unwrap();
		let test = start.insert(3, ()).unwrap();
		//      5
		//     / \
		//    4   6
		//   /
		//  3
		assert_eq!(test.root.is_some(), true);

		let ref test_root = test.root.as_ref().as_ref().unwrap();
		assert_eq!(test_root.is_red(), false);
		assert_eq!(test_root.key, 5);
		assert_eq!(test_root.left.is_some(), true);
		assert_eq!(test_root.right.is_some(), true);

		let ref test_left = test_root.left.as_ref().as_ref().unwrap();
		assert_eq!(test_left.is_red(), false);
		assert_eq!(test_left.key, 4);
		assert_eq!(test_left.left.is_some(), true);
		assert_eq!(test_left.right.is_some(), false);

		let ref test_right = test_root.right.as_ref().as_ref().unwrap();
		assert_eq!(test_right.is_red(), false);
		assert_eq!(test_right.key, 6);
		assert_eq!(test_right.left.is_some(), false);
		assert_eq!(test_right.right.is_some(), false);

		let ref test_left_left = test_left.left.as_ref().as_ref().unwrap();
		assert_eq!(test_left_left.is_red(), true);
		assert_eq!(test_left_left.key, 3);
		assert_eq!(test_left_left.left.is_some(), false);
		assert_eq!(test_left_left.right.is_some(), false);
	}

	#[test]
	fn test_insert_case3_left_b() {
		//    B
		//   / \
		//  R   R
		//   \
		//    R
		let start = Tree::new().insert(5, ()).unwrap().insert(3, ()).unwrap().insert(6, ()).unwrap();
		let test = start.insert(4, ()).unwrap();
		//    5
		//   / \
		//  3   6
		//   \
		//    4
		assert_eq!(test.root.is_some(), true);

		let ref test_root = test.root.as_ref().as_ref().unwrap();
		assert_eq!(test_root.is_red(), false);
		assert_eq!(test_root.key, 5);
		assert_eq!(test_root.left.is_some(), true);
		assert_eq!(test_root.right.is_some(), true);

		let ref test_left = test_root.left.as_ref().as_ref().unwrap();
		assert_eq!(test_left.is_red(), false);
		assert_eq!(test_left.key, 3);
		assert_eq!(test_left.left.is_some(), false);
		assert_eq!(test_left.right.is_some(), true);

		let ref test_right = test_root.right.as_ref().as_ref().unwrap();
		assert_eq!(test_right.is_red(), false);
		assert_eq!(test_right.key, 6);
		assert_eq!(test_right.left.is_some(), false);
		assert_eq!(test_right.right.is_some(), false);

		let ref test_left_right = test_left.right.as_ref().as_ref().unwrap();
		assert_eq!(test_left_right.is_red(), true);
		assert_eq!(test_left_right.key, 4);
		assert_eq!(test_left_right.left.is_some(), false);
		assert_eq!(test_left_right.right.is_some(), false);
	}

	#[test]
	fn test_insert_case3_right_a() {
		//    B
		//   / \
		//  R   R
		//     /
		//    R
		let start = Tree::new().insert(5, ()).unwrap().insert(4, ()).unwrap().insert(7, ()).unwrap();
		let test = start.insert(6, ()).unwrap();
		//      5
		//     / \
		//    4   7
		//       /
		//      6
		assert_eq!(test.root.is_some(), true);

		let ref test_root = test.root.as_ref().as_ref().unwrap();
		assert_eq!(test_root.is_red(), false);
		assert_eq!(test_root.key, 5);
		assert_eq!(test_root.left.is_some(), true);
		assert_eq!(test_root.right.is_some(), true);

		let ref test_left = test_root.left.as_ref().as_ref().unwrap();
		assert_eq!(test_left.is_red(), false);
		assert_eq!(test_left.key, 4);
		assert_eq!(test_left.left.is_some(), false);
		assert_eq!(test_left.right.is_some(), false);

		let ref test_right = test_root.right.as_ref().as_ref().unwrap();
		assert_eq!(test_right.is_red(), false);
		assert_eq!(test_right.key, 7);
		assert_eq!(test_right.left.is_some(), true);
		assert_eq!(test_right.right.is_some(), false);

		let ref test_right_left = test_right.left.as_ref().as_ref().unwrap();
		assert_eq!(test_right_left.is_red(), true);
		assert_eq!(test_right_left.key, 6);
		assert_eq!(test_right_left.left.is_some(), false);
		assert_eq!(test_right_left.right.is_some(), false);
	}

	#[test]
	fn test_insert_case3_right_b() {
		//    B
		//   / \
		//  R   R
		//       \
		//        R
		let start = Tree::new().insert(5, ()).unwrap().insert(4, ()).unwrap().insert(6, ()).unwrap();
		let test = start.insert(7, ()).unwrap();
		//      5
		//     / \
		//    4   6
		//         \
		//          7
		assert_eq!(test.root.is_some(), true);

		let ref test_root = test.root.as_ref().as_ref().unwrap();
		assert_eq!(test_root.is_red(), false);
		assert_eq!(test_root.key, 5);
		assert_eq!(test_root.left.is_some(), true);
		assert_eq!(test_root.right.is_some(), true);

		let ref test_left = test_root.left.as_ref().as_ref().unwrap();
		assert_eq!(test_left.is_red(), false);
		assert_eq!(test_left.key, 4);
		assert_eq!(test_left.left.is_some(), false);
		assert_eq!(test_left.right.is_some(), false);

		let ref test_right = test_root.right.as_ref().as_ref().unwrap();
		assert_eq!(test_right.is_red(), false);
		assert_eq!(test_right.key, 6);
		assert_eq!(test_right.left.is_some(), false);
		assert_eq!(test_right.right.is_some(), true);

		let ref test_right_right = test_right.right.as_ref().as_ref().unwrap();
		assert_eq!(test_right_right.is_red(), true);
		assert_eq!(test_right_right.key, 7);
		assert_eq!(test_right_right.left.is_some(), false);
		assert_eq!(test_right_right.right.is_some(), false);
	}
}
