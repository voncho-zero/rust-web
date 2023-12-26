use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub mod base;


#[derive(serde::Serialize)]
pub struct Message {
    pub message: String,
}
#[derive(serde::Serialize)]
pub struct TreeNode<T> {
    pub id: Option<usize>,
    pub children: Vec<Rc<RefCell<TreeNode<T>>>>,
    // pub parent: Option<Rc<RefCell<TreeNode<T>>>>,
    pub is_leaf: bool,
    pub raw: T,
}

impl<T> TreeNode<T> where T: Tree {
    pub fn new(raw: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            TreeNode {
                id: raw.get_id(),
                children: vec![],
                // parent: None,
                is_leaf: false,
                raw
            }
        ))

    }
    pub fn list_to_tree (menus: Vec<T>) -> Vec<Rc<RefCell<TreeNode<T>>>> {
        let mut result = vec![];
        let mut mapping: HashMap<usize, Rc<RefCell<TreeNode<T>>>> = HashMap::new();
        let mut group: HashMap<usize, Vec<Rc<RefCell<TreeNode<T>>>>> = HashMap::new();
        for menu in menus {
            let id = menu.get_id().unwrap();
            let parent_id = menu.get_parent_id().unwrap();
            let tn = TreeNode::new(menu);
            mapping.insert(id, tn.clone());
            if group.contains_key(&parent_id) {
                group.get_mut(&parent_id).unwrap().push(tn.clone());
            } else {
                group.insert(parent_id, vec![]);
            }
        }
        group.iter().for_each(|(k, v)| {
            if mapping.contains_key(k) {
                let mut parent = mapping.get(k).unwrap().borrow_mut();
                let children = v.to_vec();
                parent.children = children;
            } else {
                result.append(&mut v.to_vec())
            }
        });
        result
    }
}
pub trait Tree {
    fn get_id(&self) -> Option<usize>;
    fn get_parent_id(&self) -> Option<usize>;
}



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

