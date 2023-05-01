#[derive(Debug)]
#[allow(unused)]
struct Element<'a> {
    value: String,
    reference: Option<&'a Vec<&'a Element<'a>>>
    // reference: Option<Vec<&'a Element<'a>>>s
}

impl Element<'_> {
    fn new(letter: &str) -> Self {
        Element { value: letter.to_string(), reference: None }
    }
    fn new_ref<'a> (letter: &'a str, reference: &'a Vec<&'a Element<'a>>) -> &'a Self {
        &Self { value: letter.to_string(), reference: Some(reference) }
    }
}


#[allow(unused)]
fn main() {
    let a = Element { value: "a".to_string(), reference: None };
    let b = Element { value: "b".to_string(), reference: Some(&vec![&a]) };
    let c = Element::new("c");

    
    let mut v = Vec::new();

    v.push(&a);
    v.push(&b);

    println!("{:#?}",v);

}
