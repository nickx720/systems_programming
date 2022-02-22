///https://reberhardt.com/cs110l/spring-2020/assignments/week-2-exercises/
///https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut
/// Part 2 Todo rdiff

fn ownership_one() {
    let mut s = String::from("hello");
    let ref1 = &s;
    let ref2 = &ref1;
    let ref3 = &ref2;
    //    s = String::from("goodbye"); Cannot reassign the mutable referrent because it has references
    println!("{}", ref3.to_uppercase());
    let s1 = String::from("hello");
    let mut v = Vec::new();
    v.push(s1);
    //    let s2: String = v[0]; Since string doesnot implement copy type, it cannot copied without
    //    moving
    println!("{:?}", v);
}

fn drip_drop() {
    // fn drop_drop() -> &String { -> doesn't live long enough, the scope of the string ends at the end of the return
    let s = String::from("hello world!");
    //    return &s;
    todo!()
}

pub fn main_lecture2() {
    let x = &mut vec![1, 2, 4];
    for elem in x.iter_mut() {
        *elem += 2;
    }
    println!("{:?}", x);
}
