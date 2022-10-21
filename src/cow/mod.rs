use std::borrow::Cow;

struct User<'a> {
    first_name: Cow<'a, str>,
    last_name: Cow<'a, str>,
}

impl<'a> User<'a> {
    pub fn new_owned(first_name: String, last_name: String) -> User<'static> {
        User {
            first_name: Cow::Owned(first_name),
            last_name: Cow::Owned(last_name),
        }
    }

    pub fn new_borrowed(first_name: &'a str, last_name: &'a str) -> Self {
        Self {
            first_name: Cow::Borrowed(first_name),
            last_name: Cow::Borrowed(last_name),
        }
    }

    pub fn first_name(&self) -> &str {
        &&self.first_name
    }
    pub fn last_name(&self) -> &str {
        &&self.last_name
    }
}

fn remove_whitespaces(s: &str) -> Cow<str> {
    if s.contains(' ') {
        Cow::Owned(s.to_string().replace(' ', ""))
    } else {
        Cow::Borrowed(s)
    }
}

pub fn cow_main() {
    let value = remove_whitespaces("Hello World  ");
    println!("{value}");

    let user: User<'static> = User::new_owned("Nick".to_owned(), "Bond".to_owned());
    println!("Name: {} {}", user.first_name, user.last_name);

    let user: User<'static> = User::new_borrowed("Felix", "Leiter");
    println!("Name: {},{}", user.first_name, user.last_name);
}
