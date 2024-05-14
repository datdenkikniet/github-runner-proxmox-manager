use std::{borrow::Cow, collections::BTreeMap};

use crate::{Path, PathElement};

use super::{Info, TreeNode};

#[derive(Clone)]
pub struct Value<'a> {
    pub leaf: Option<u32>,
    pub path: Path,
    text: Cow<'a, str>,
    pub info: BTreeMap<Cow<'a, str>, Info<'a>>,
}

impl core::fmt::Debug for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Value")
            .field("leaf", &self.leaf)
            .field("path", &format!("{}", &self.path))
            .field("text", &self.text)
            .field("info", &self.info)
            .finish()
    }
}

impl<'a> TryFrom<super::Value<'a>> for Value<'a> {
    type Error = ();

    fn try_from(value: super::Value<'a>) -> Result<Self, Self::Error> {
        let path = Path::try_from(value.path.as_ref())?;

        Ok(Self {
            leaf: value.leaf,
            path,
            text: value.text,
            info: value.info,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Collection<'a> {
    values: Vec<(PathElement, Node<'a>)>,
}

impl<'a> Collection<'a> {
    pub fn from_nodes<'b>(nodes: &'b [TreeNode<'a>]) -> Self {
        let mut values = Vec::new();
        for node in nodes {
            let element = Node::nth_element(&node.value.path, 0);

            let node = Node::from_tree_node(node.clone());
            values.push((element, node));
        }

        Self { values }
    }

    pub fn iter(&self) -> impl Iterator<Item = &'_ Node> {
        self.values.iter().map(|(_, node)| node)
    }
}

#[derive(Debug, Clone)]
pub struct Node<'a> {
    pub value: Value<'a>,
    pub children: Vec<(PathElement, Node<'a>)>,
}

impl<'a> Node<'a> {
    fn nth_element(path: &str, n: usize) -> PathElement {
        Path::try_from(path).unwrap().iter().nth(n).unwrap().clone()
    }

    pub fn from_tree_node(node: TreeNode<'a>) -> Self {
        Self::from_tree_node_depth(node, 1)
    }

    fn from_tree_node_depth(node: TreeNode<'a>, depth: usize) -> Self {
        let value = node.value;

        let mut children = Vec::new();
        for node in node.children {
            let next_element = Self::nth_element(&node.value.path, depth);

            children.push((next_element, Self::from_tree_node_depth(node, depth + 1)));
        }

        Self {
            value: value.try_into().unwrap(),
            children,
        }
    }
}
