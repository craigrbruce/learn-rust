fn main() {
    let mut tree = Tree::new(1);
    tree.insert(2);
    tree.insert(3);
    tree.insert(4);

    tree.print();
}

type Node = Option<Box<Tree>>;
type Item = Option<u64>;

#[derive(Default)]
pub struct Tree {
    item: Item,
    left: Node,
    right: Node,
}

impl Tree {
    pub fn new(item: u64) -> Self {
        println!("Create the tree with {}", item);
        Tree {
            item: Some(item),
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, i: u64) {
        if self.item.is_none() {
            println!("Inserting {} to self", i);
            self.item = Some(i);
        } else {
            let node = if i < self.item.unwrap() {
                &mut self.left
            } else {
                &mut self.right
            };
            match node {
                Some(n) => n.as_mut().insert(i),
                None => {
                    let tree = Tree::new(i);
                    *node = Some(Box::new(tree));
                }
            }
        }
    }

    pub fn search(&self, i: u64) -> bool {
        let item = self.item.unwrap();
        if item == i {
            return true;
        }
        let node = if i < item { &self.left } else { &self.right };
        node.as_ref().map(|n| n.search(i)).unwrap_or(false)
    }

    pub fn print(&mut self) {
        if let Some(n) = self.item {
            println!("Value: {}", n);
        }
        if let Some(n) = &mut self.left {
            n.as_mut().print();
        }
        if let Some(n) = &mut self.right {
            n.as_mut().print();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search() {
        let mut tree = Tree::new(1);
        tree.insert(2);
        tree.insert(3);
        tree.insert(4);
    }
}

// pub fn power(a: u64, n: u64) {
//     match n {
//         0 => 1,
//         _ => {
//             let x: u64 = power(a, n / 2);
//             if (n.is_even()) {
//                 return x.mu
//             }
//             return a * (x * x);
//         }
//     }
// }
