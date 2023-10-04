#[derive(Debug)]
pub struct Node {
    pub size: Option<i64>,
    pub name: String,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new_from_size(name: String, size: i64) -> Node {
        Node {
            name,
            size: Some(size),
            children: Vec::new(),
        }
    }
    pub fn new_from_children(name: String, children: Vec<Node>) -> Node {
        let mut node = Node {
            name,
            size: None,
            children,
        };
        node.get_or_compute_size();
        node
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

    pub fn refresh_size(&mut self) -> i64 {
        if self.is_leaf() {
            self.size.unwrap()
        } else {
            let mut size = 0;
            for child in &mut self.children {
                size += child.refresh_size();
            }
            self.size = Some(size);
            size
        }
    }
    pub fn get_or_compute_size(&mut self) -> i64 {
        if let Some(size) = self.size {
            size
        } else {
            let mut size = 0;
            for child in &mut self.children {
                size += child.get_or_compute_size();
            }
            self.size = Some(size);
            size
        }
    }
    pub fn get_size(&self) -> i64 {
        self.size.unwrap()
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_size_computation() {
        let mut tree = Node::new_from_children(
            "root".to_string(),
            vec![
                Node::new_from_size("child1".to_string(), 5),
                Node::new_from_size("child2".to_string(), 7),
            ],
        );

        assert_eq!(tree.refresh_size(), 12);
        assert_eq!(tree.get_or_compute_size(), 12);
    }
}
