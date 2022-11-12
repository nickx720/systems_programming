// https://www.youtube.com/watch?v=4YAmpHMl1Z0
// AsRef vs Borrow
#[derive(Debug)]
struct MyBox<T>(T);

impl std::borrow::Borrow<str> for MyBox<&str> {
    fn borrow(&self) -> &str {
        &self.0
    }
}

struct FakeHashMap<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K, V> FakeHashMap<K, V> {
    fn new() -> Self {
        Self {
            keys: vec![],
            values: vec![],
        }
    }

    fn insert(&mut self, key: K, value: V) {
        self.keys.push(key);
        self.values.push(value);
    }

    fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: std::cmp::Eq + ?Sized,
    {
        let mut found = None;

        for (index, item) in self.keys.iter().enumerate() {
            if item.borrow() == k {
                found = Some(index);
                break;
            }
        }
        if let Some(index) = found {
            return Some(&self.values[index]);
        }
        None
    }
}

#[derive(Debug)]
struct MyBoxAsRef<T>(T);

impl AsRef<str> for MyBoxAsRef<&str> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

struct FakeHashMapAsRef<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K, V> FakeHashMapAsRef<K, V> {
    fn new() -> Self {
        Self {
            keys: vec![],
            values: vec![],
        }
    }

    fn insert(&mut self, key: K, value: V) {
        self.keys.push(key);
        self.values.push(value);
    }

    fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: AsRef<Q>,
        Q: std::cmp::Eq + ?Sized,
    {
        let mut found = None;

        for (index, item) in self.keys.iter().enumerate() {
            if item.as_ref() == k {
                found = Some(index);
                break;
            }
        }
        if let Some(index) = found {
            return Some(&self.values[index]);
        }
        None
    }
}
pub fn borrow_main() {
    let mut fhm: FakeHashMap<MyBox<&str>, MyBox<&str>> = FakeHashMap::new();
    fhm.insert(MyBox("key"), MyBox("value"));
    println!("{:?}", fhm.get("key"));
}

// What is ?Sized
// Optional for property to be sized aka size to be known at compile time
// Size of std::mem::size_of::<i32>() -> 4
// Size of std::mem::size_of::<i64>() -> 8
// Size of std::mem::size_of::<f32>() -> 4
// Size of std::mem::size_of::<f64>() -> 8
