use std::vec;

#[derive(Debug)]
#[allow(unused)]
struct Element<'a>{
    value: String,
    reference: Option<Vec<&'a Element<'a>>>
    // reference: Option<Vec<&'a Element<'a>>>s
}

impl<'a> Element<'a> {
    fn new(letter: &str) -> Self {
        Element { value: letter.to_string(), reference: None }
    }
    fn new_ref (letter: &'a str, reference: Vec<&'a Element<'a>>) -> Self {
        let ref _x = *reference; 
        // &Element { value: letter.to_string(), reference: Some(x) }
        Self { value: letter.to_string(), reference: Some(reference) }
        // &Element { value: letter.to_string(), reference: Some( &vec![&Element]) }
    }
}


#[allow(unused)]
fn main() {
    let a = Element::new("a");
    let b = Element::new_ref("b", vec![&a]);;
    let c = Element::new("c");
    let d = Element::new_ref("d", vec![&b , &c]);

    
    let mut v = Vec::new();

    v.push(&a);
    v.push(&b);
    v.push(&c);
    v.push(&d);

    println!("{:#?}",v);

}
