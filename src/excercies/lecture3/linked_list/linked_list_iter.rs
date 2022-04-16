use super::linked_list::Node;

pub struct LinkedListIter<'a> {
    current: &'a Option<Box<Node<u32>>>,
}

impl Iterator for LinkedListIter<'_> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                todo!()
            }
            None => {
                todo!()
            }
        }
    }
}
// For linked list, read the docs completely
impl<'a> IntoIterator for &'a LinkedList {
    type Item = u32;
    type IntoIter = LinkedListIter<'a>;
    fn into_iter(self) -> LinkedListIter<'a> {
        LinkedListIter {
            current: &self.head,
        }
    }
}
