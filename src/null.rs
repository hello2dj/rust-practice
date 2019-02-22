use std::collections::HashMap;
use std::sync::Mutex;
use std::borrow::Borrow;

pub fn foo() -> Vec<char> {
    let mut data = vec!['a', 'b', 'c']; // ----data scope

    capitalize(&mut data);

    data.push('d');
    data.push('e');
    data
}

lazy_static! {
    static ref MAP: Mutex<HashMap<&'static str, i32>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}

fn capitalize(data: &mut [char]) {
    for c in data {
        c.make_ascii_uppercase();
    }
}


fn process_or_default(key: &str) {
    MAP.lock().unwrap().insert("dengjie", 123);
    let mut map: HashMap<String, i32> = HashMap::new();
    match map.get_mut(key) {
        Some(value) => println!("{}", value),
        None => {
            map.insert(key.to_string(), 123);
        }
    }

}

#[test]
fn null_test() {
    let data = foo();
    process_or_default("dengjie");
}