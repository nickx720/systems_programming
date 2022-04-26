use linked_list::LinkedList;
pub mod linked_list;
pub mod linked_list_iter;

pub fn linkedmain() {
    let mut list: LinkedList<u32> = LinkedList::new();
    let three = 2u32;
    dbg!(three);
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    // If you implement iterator trait:
    for val in &list {
        println!("{}", val);
    }
    println!("Completed");
}
