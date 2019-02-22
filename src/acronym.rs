use std::error::Error;
use std::iter::FromIterator;
use std::collections::BTreeMap;
use std::collections::LinkedList;


pub fn acronym(input: &str) -> Result<String, std::option::NoneError> {
    let mut words = String::new();
//    for word in input.split(" ") {
//        let a = word.chars().nth(0)?;
//        words.push(a.to_ascii_uppercase());
//    }
//
    let c = input.split(" ");

    let a = input.split(" ")
        .map(|word| -> Result<String, std::option::NoneError> {
            Ok(word.chars().nth(0)?.to_ascii_uppercase().to_string())
        })
        .collect::<Result<String, std::option::NoneError>>();
    let mut map = BTreeMap::new();
    map.entry("dj").or_insert(100);

    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_front(23);

    let mut a = Some(String::new()).take();
    match a {
        Some(ref b) => println!("{:?}", b),
        None => println!(""),
    }

    dbg!(a);
    Ok(words)
}


