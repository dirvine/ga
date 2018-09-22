use std::ops::{Index, IndexMut};
use std::{cmp, fmt, mem, slice};

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash, Deserialize, Serialize)]
/// A node identifier within a particular `Arena`
pub struct NodeId {
  index: usize,
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
/// A node within a particular `Arena`
pub struct Node<T> {
  // Keep these private (with read-only accessors) so that we can keep them consistent.
  // E.g. the parent of a node’s child is that node.

  parent1: Option<NodeId>,
  parent2: Option<NodeId>,
  previous_sibling: Option<NodeId>,
  next_sibling: Option<NodeId>,
  first_child: Option<NodeId>,
  last_child: Option<NodeId>,

  /// The actual data which will be stored within the tree
  pub data: T,
}

impl<T> fmt::Display for Node<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Parent1: {:?}, ", self.parent1)?;
    write!(f, "Parent2: {:?}, ", self.parent2)?;
    write!(f, "Previous sibling: {:?}, ", self.previous_sibling)?;
    write!(f, "Next sibling: {:?}, ", self.next_sibling)?;
    write!(f, "First child: {:?}, ", self.first_child)?;
    write!(f, "Last child: {:?}", self.last_child)
  }
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
/// An `Arena` structure containing certain Nodes
#[derive(Default)]
pub struct Arena<T> {
  nodes: Vec<Node<T>>,
}

impl<T> Arena<T> {
  /// Create a new empty `Arena`
  pub fn new() -> Arena<T> {
    Arena { nodes: Vec::new() }
  }

  /// Create a new node from its associated data.
  pub fn new_node(&mut self, data: T) -> NodeId {
    let next_index = self.nodes.len();
    self.nodes.push(Node {
      parent1: None,
      parent2: None,
      first_child: None,
      last_child: None,
      previous_sibling: None,
      next_sibling: None,
      data: data,
    });
    NodeId { index: next_index }
  }

  // Count nodes in arena.
  pub fn count(&self) -> usize {
    self.nodes.len()
  }

  // Returns true if arena has no nodes, false otherwise
  pub fn is_empty(&self) -> bool {
    self.count() == 0
  }

  /// Get a reference to the node with the given id if in the arena, None
  /// otherwise.
  pub fn get(&self, id: NodeId) -> Option<&Node<T>> {
    self.nodes.get(id.index)
  }

  /// Get a mutable reference to the node with the given id if in the arena,
  /// None otherwise.
  pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node<T>> {
    self.nodes.get_mut(id.index)
  }

  /// Iterate over all nodes in the arena in storage-order.
  pub fn iter(&self) -> slice::Iter<'_, Node<T>> {
    self.nodes.iter()
  }
}

// impl<T: Sync> Arena<T> {
//   /// Return an parallel iterator over the whole arena.
//   // pub fn par_iter(&self) -> rayon::slice::Iter<Node<T>> {
//   //   self.nodes.par_iter()
//   // }
// }

trait GetPairMut<T> {
  /// Get mutable references to two distinct nodes. Panics if the two given IDs
  /// are the same.
  fn get_pair_mut(
    &mut self,
    a: usize,
    b: usize,
    same_index_error_message: &'static str,
  ) -> (&mut T, &mut T);
}

impl<T> GetPairMut<T> for Vec<T> {
  fn get_pair_mut(
    &mut self,
    a: usize,
    b: usize,
    same_index_error_message: &'static str,
  ) -> (&mut T, &mut T) {
    if a == b {
      panic!(same_index_error_message)
    }
    let (xs, ys) = self.split_at_mut(cmp::max(a, b));
    if a < b {
      (&mut xs[a], &mut ys[0])
    } else {
      (&mut ys[0], &mut xs[b])
    }
  }
}

impl<T> Index<NodeId> for Arena<T> {
  type Output = Node<T>;

  fn index(&self, node: NodeId) -> &Node<T> {
    &self.nodes[node.index]
  }
}

impl<T> IndexMut<NodeId> for Arena<T> {
  fn index_mut(&mut self, node: NodeId) -> &mut Node<T> {
    &mut self.nodes[node.index]
  }
}

impl<T> Node<T> {
  /// Return the ID of the parent node, unless this node is the root of the
  /// tree.
  pub fn parent1(&self) -> Option<NodeId> {
    self.parent1
  }
  /// Return the ID of the parent node, unless this node is the root of the
  /// tree.
  pub fn parent2(&self) -> Option<NodeId> {
    self.parent2
  }

  /// Return the ID of the first child of this node, unless it has no child.
  pub fn first_child(&self) -> Option<NodeId> {
    self.first_child
  }

  /// Return the ID of the last child of this node, unless it has no child.
  pub fn last_child(&self) -> Option<NodeId> {
    self.last_child
  }

  /// Return the ID of the previous sibling of this node, unless it is a first
  /// child.
  pub fn previous_sibling(&self) -> Option<NodeId> {
    self.previous_sibling
  }

  /// Return the ID of the previous sibling of this node, unless it is a first
  /// child.
  pub fn next_sibling(&self) -> Option<NodeId> {
    self.next_sibling
  }
}

impl NodeId {
  /// Create a `NodeId` used for attempting to get `Node`s references from an
  /// `Arena`.
  pub fn new(index: usize) -> Self {
    Self { index }
  }

  /// Return an iterator of references to this node and its ancestors.
  ///
  /// Call `.next().unwrap()` once on the iterator to skip the node itself.
  pub fn ancestors<T>(self, arena: &Arena<T>) -> Ancestors<'_, T> {
    Ancestors {
      arena: arena,
      node: Some(self),
    }
  }

  /// Return an iterator of references to this node and the siblings before it.
  ///
  /// Call `.next().unwrap()` once on the iterator to skip the node itself.
  pub fn preceding_siblings<T>(self, arena: &Arena<T>) -> PrecedingSiblings<'_, T> {
    PrecedingSiblings {
      arena: arena,
      node: Some(self),
    }
  }

  /// Return an iterator of references to this node and the siblings after it.
  ///
  /// Call `.next().unwrap()` once on the iterator to skip the node itself.
  pub fn following_siblings<T>(self, arena: &Arena<T>) -> FollowingSiblings<'_, T> {
    FollowingSiblings {
      arena: arena,
      node: Some(self),
    }
  }

  /// Return an iterator of references to this node’s children.
  pub fn children<T>(self, arena: &Arena<T>) -> Children<'_, T> {
    Children {
      arena: arena,
      node: arena[self].first_child,
    }
  }

  /// Return an iterator of references to this node’s children, in reverse
  /// order.
  pub fn reverse_children<T>(self, arena: &Arena<T>) -> ReverseChildren<'_, T> {
    ReverseChildren {
      arena: arena,
      node: arena[self].last_child,
    }
  }

  /// Return an iterator of references to this node and its descendants, in
  /// tree order.
  ///
  /// Parent nodes appear before the descendants.
  /// Call `.next().unwrap()` once on the iterator to skip the node itself.
  pub fn descendants<T>(self, arena: &Arena<T>) -> Descendants<'_, T> {
    Descendants(self.traverse(arena))
  }

  /// Return an iterator of references to this node and its descendants, in
  /// tree order.
  pub fn traverse<T>(self, arena: &Arena<T>) -> Traverse<'_, T> {
    Traverse {
      arena: arena,
      root: self,
      next: Some(NodeEdge::Start(self)),
    }
  }

  /// Return an iterator of references to this node and its descendants, in
  /// tree order.
  pub fn reverse_traverse<T>(self, arena: &Arena<T>) -> ReverseTraverse<'_, T> {
    ReverseTraverse {
      arena: arena,
      root: self,
      next: Some(NodeEdge::End(self)),
    }
  }

  /// Detach a node from its parent and siblings. Children are not affected.
  pub fn detach<T>(self, arena: &mut Arena<T>) {
    let (parent1, parent2, previous_sibling, next_sibling) = {
      let node = &mut arena[self];
      (
        node.parent1.take(),
        node.parent2.take(),
        node.previous_sibling.take(),
        node.next_sibling.take(),
      )
    };

    if let Some(next_sibling) = next_sibling {
      arena[next_sibling].previous_sibling = previous_sibling;
    } 
    if let Some(parent1) = parent1  {
      arena[parent1].last_child = previous_sibling;
    }
    if let Some(parent2) = parent2  {
      arena[parent2].last_child = previous_sibling;
    }

    if let Some(previous_sibling) = previous_sibling {
      arena[previous_sibling].next_sibling = next_sibling;
    } 
    if let Some(parent1) = parent1 {
      arena[parent1].first_child = next_sibling;
    }
    if let Some(parent2) = parent2 {
      arena[parent2].first_child = next_sibling;
    }
  }

//   /// Append a new child to this node, after existing children.
//   pub fn append<T>(self, new_child: NodeId, arena: &mut Arena<T>) {
//     new_child.detach(arena);
//     let last_child_opt;
//     {
//       let (self_borrow, new_child_borrow) = arena.nodes.get_pair_mut(
//         self.index,
//         new_child.index,
//         "Can not append a node to itself",
//       );
//       new_child_borrow.parent = Some(self);
//       last_child_opt = mem::replace(&mut self_borrow.last_child, Some(new_child));
//       if let Some(last_child) = last_child_opt {
//         new_child_borrow.previous_sibling = Some(last_child);
//       } else {
//         debug_assert!(self_borrow.first_child.is_none());
//         self_borrow.first_child = Some(new_child);
//       }
//     }
//     if let Some(last_child) = last_child_opt {
//       debug_assert!(arena[last_child].next_sibling.is_none());
//       arena[last_child].next_sibling = Some(new_child);
//     }
//   }

//   /// Prepend a new child to this node, before existing children.
//   pub fn prepend<T>(self, new_child: NodeId, arena: &mut Arena<T>) {
//     new_child.detach(arena);
//     let first_child_opt;
//     {
//       let (self_borrow, new_child_borrow) = arena.nodes.get_pair_mut(
//         self.index,
//         new_child.index,
//         "Can not prepend a node to itself",
//       );
//       new_child_borrow.parent = Some(self);
//       first_child_opt = mem::replace(&mut self_borrow.first_child, Some(new_child));
//       if let Some(first_child) = first_child_opt {
//         new_child_borrow.next_sibling = Some(first_child);
//       } else {
//         self_borrow.last_child = Some(new_child);
//         debug_assert!(&self_borrow.first_child.is_none());
//       }
//     }
//     if let Some(first_child) = first_child_opt {
//       debug_assert!(arena[first_child].previous_sibling.is_none());
//       arena[first_child].previous_sibling = Some(new_child);
//     }
//   }

//   /// Insert a new sibling after this node.
//   pub fn insert_after<T>(self, new_sibling: NodeId, arena: &mut Arena<T>) {
//     new_sibling.detach(arena);
//     let next_sibling_opt;
//     let parent_opt;
//     {
//       let (self_borrow, new_sibling_borrow) = arena.nodes.get_pair_mut(
//         self.index,
//         new_sibling.index,
//         "Can not insert a node after itself",
//       );
//       parent_opt = self_borrow.parent;
//       new_sibling_borrow.parent = parent_opt;
//       new_sibling_borrow.previous_sibling = Some(self);
//       next_sibling_opt = mem::replace(&mut self_borrow.next_sibling, Some(new_sibling));
//       if let Some(next_sibling) = next_sibling_opt {
//         new_sibling_borrow.next_sibling = Some(next_sibling);
//       }
//     }
//     if let Some(next_sibling) = next_sibling_opt {
//       debug_assert!(arena[next_sibling].previous_sibling.unwrap() == self);
//       arena[next_sibling].previous_sibling = Some(new_sibling);
//     } else if let Some(parent) = parent_opt {
//       debug_assert!(arena[parent].last_child.unwrap() == self);
//       arena[parent].last_child = Some(new_sibling);
//     }
//   }

//   /// Insert a new sibling before this node.
//   pub fn insert_before<T>(self, new_sibling: NodeId, arena: &mut Arena<T>) {
//     new_sibling.detach(arena);
//     let previous_sibling_opt;
//     let parent_opt;
//     {
//       let (self_borrow, new_sibling_borrow) = arena.nodes.get_pair_mut(
//         self.index,
//         new_sibling.index,
//         "Can not insert a node before itself",
//       );
//       parent_opt = self_borrow.parent;
//       new_sibling_borrow.parent = parent_opt;
//       new_sibling_borrow.next_sibling = Some(self);
//       previous_sibling_opt = mem::replace(&mut self_borrow.previous_sibling, Some(new_sibling));
//       if let Some(previous_sibling) = previous_sibling_opt {
//         new_sibling_borrow.previous_sibling = Some(previous_sibling);
//       }
//     }
//     if let Some(previous_sibling) = previous_sibling_opt {
//       debug_assert!(arena[previous_sibling].next_sibling.unwrap() == self);
//       arena[previous_sibling].next_sibling = Some(new_sibling);
//     } else if let Some(parent) = parent_opt {
//       debug_assert!(arena[parent].first_child.unwrap() == self);
//       arena[parent].first_child = Some(new_sibling);
//     }
//   }
}

macro_rules! impl_node_iterator {
  ($name:ident, $next:expr) => {
    impl<'a, T> Iterator for $name<'a, T> {
      type Item = NodeId;

      fn next(&mut self) -> Option<NodeId> {
        match self.node.take() {
          Some(node) => {
            self.node = $next(&self.arena[node]);
            Some(node)
          }
          None => None,
        }
      }
    }
  };
}

/// An iterator of references to the ancestors a given node.
pub struct Ancestors<'a, T: 'a> {
  arena: &'a Arena<T>,
  node: Option<NodeId>,
}
impl_node_iterator!(Ancestors, |node: &Node<T>| node.parent1);

/// An iterator of references to the siblings before a given node.
pub struct PrecedingSiblings<'a, T: 'a> {
  arena: &'a Arena<T>,
  node: Option<NodeId>,
}
impl_node_iterator!(PrecedingSiblings, |node: &Node<T>| node.previous_sibling);

/// An iterator of references to the siblings after a given node.
pub struct FollowingSiblings<'a, T: 'a> {
  arena: &'a Arena<T>,
  node: Option<NodeId>,
}
impl_node_iterator!(FollowingSiblings, |node: &Node<T>| node.next_sibling);

/// An iterator of references to the children of a given node.
pub struct Children<'a, T: 'a> {
  arena: &'a Arena<T>,
  node: Option<NodeId>,
}
impl_node_iterator!(Children, |node: &Node<T>| node.next_sibling);

/// An iterator of references to the children of a given node, in reverse order.
pub struct ReverseChildren<'a, T: 'a> {
  arena: &'a Arena<T>,
  node: Option<NodeId>,
}
impl_node_iterator!(ReverseChildren, |node: &Node<T>| node.previous_sibling);

/// An iterator of references to a given node and its descendants, in tree
/// order.
pub struct Descendants<'a, T: 'a>(Traverse<'a, T>);

impl<'a, T> Iterator for Descendants<'a, T> {
  type Item = NodeId;

  fn next(&mut self) -> Option<NodeId> {
    loop {
      match self.0.next() {
        Some(NodeEdge::Start(node)) => return Some(node),
        Some(NodeEdge::End(_)) => {}
        None => return None,
      }
    }
  }
}

#[derive(Debug, Clone)]
/// Indicator if the node is at a start or endpoint of the tree
pub enum NodeEdge<T> {
  /// Indicates that start of a node that has children. Yielded by
  /// `Traverse::next` before the node’s descendants. In HTML or XML, this
  /// corresponds to an opening tag like `<div>`
  Start(T),

  /// Indicates that end of a node that has children. Yielded by
  /// `Traverse::next` after the node’s descendants. In HTML or XML, this
  /// corresponds to a closing tag like `</div>`
  End(T),
}

/// An iterator of references to a given node and its descendants, in tree
/// order.
pub struct Traverse<'a, T: 'a> {
  arena: &'a Arena<T>,
  root: NodeId,
  next: Option<NodeEdge<NodeId>>,
}

impl<'a, T> Iterator for Traverse<'a, T> {
  type Item = NodeEdge<NodeId>;

  fn next(&mut self) -> Option<NodeEdge<NodeId>> {
    match self.next.take() {
      Some(item) => {
        self.next = match item {
          NodeEdge::Start(node) => match self.arena[node].first_child {
            Some(first_child) => Some(NodeEdge::Start(first_child)),
            None => Some(NodeEdge::End(node)),
          },
          NodeEdge::End(node) => {
            if node == self.root {
              None
            } else {
              match self.arena[node].next_sibling {
                Some(next_sibling) => Some(NodeEdge::Start(next_sibling)),
                None => {
                  match self.arena[node].parent1 {
                    Some(parent) => Some(NodeEdge::End(parent)),

                    // `node.parent()` here can only be `None`
                    // if the tree has been modified during iteration,
                    // but silently stoping iteration
                    // seems a more sensible behavior than panicking.
                    None => None,
                  }
                }
              }
            }
          }
        };
        Some(item)
      }
      None => None,
    }
  }
}

/// An iterator of references to a given node and its descendants, in reverse
/// tree order.
pub struct ReverseTraverse<'a, T: 'a> {
  arena: &'a Arena<T>,
  root: NodeId,
  next: Option<NodeEdge<NodeId>>,
}

// impl<'a, T> Iterator for ReverseTraverse<'a, T> {
//   type Item = NodeEdge<NodeId>;

//   fn next(&mut self) -> Option<NodeEdge<NodeId>> {
//     match self.next.take() {
//       Some(item) => {
//         self.next = match item {
//           NodeEdge::End(node) => match self.arena[node].last_child {
//             Some(last_child) => Some(NodeEdge::End(last_child)),
//             None => Some(NodeEdge::Start(node)),
//           },
//           NodeEdge::Start(node) => {
//             if node == self.root {
//               None
//             } else {
//               match self.arena[node].previous_sibling {
//                 Some(previous_sibling) => Some(NodeEdge::End(previous_sibling)),
//                 None => {
//                   match self.arena[node].parent {
//                     Some(parent) => Some(NodeEdge::Start(parent)),

//                     // `node.parent()` here can only be `None`
//                     // if the tree has been modified during iteration,
//                     // but silently stoping iteration
//                     // seems a more sensible behavior than panicking.
//                     None => None,
//                   }
//                 }
//               }
//             }
//           }
//         };
//         Some(item)
//       }
//       None => None,
//     }
//   }
// }
