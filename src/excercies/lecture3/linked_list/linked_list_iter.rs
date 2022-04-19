use super::linked_list::{LinkedList, Node};

pub struct LinkedListIter<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}

impl<T> Iterator for LinkedListIter<'_, T>
where
    T: Copy,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                if let Some(next) = node.next.as_ref() {
                    Some(next.value)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
// For linked list, read the docs completely
impl<'a, T> IntoIterator for &'a LinkedList<T>
where
    T: Copy,
{
    type Item = T;
    type IntoIter = LinkedListIter<'a, T>;
    fn into_iter(self) -> LinkedListIter<'a, T> {
        LinkedListIter {
            current: &self.head,
        }
    }
}
