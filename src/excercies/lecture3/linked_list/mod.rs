use linked_list::LinkedList;
pub mod linked_list;

pub fn linkedmain() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    let mut list2: LinkedList<u32> = LinkedList::new();
    for i in 1..12 {
        list2.push_front(i);
    }
    println!("{}", list);
    if list == list2 {
        println!("Equal");
    }
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}
}
