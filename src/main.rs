enum ElemError {
    OtherError,
    InsertionError,
}

#[derive(Debug)]
struct Element<'a> {
    value: String,
    reference: Option<Vec<&'a Element<'a>>>,
}

impl<'a> Element<'a> {
    fn new(letter: &str) -> Self {
        Element {
            value: letter.to_string(),
            reference: None,
        }
    }
    fn new_ref(letter: &'a str, reference: Vec<&'a Element<'a>>) -> Self {
        Self {
            value: letter.to_string(),
            reference: Some(reference),
        }
    }
}

impl PartialEq for Element<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.reference == other.reference
    }
}

#[allow(unused)]
fn main() {
    let a = Element::new("a");
    let b = Element::new_ref("b", vec![&a]);
    let c = Element::new("c");
    let d = Element::new_ref("d", vec![&b]);
    let e = Element::new_ref("e", vec![&a, &d]);

    let mut v: Vec<&Element> = Vec::new();

    insert_element(&mut v, &a);
    insert_element(&mut v, &a);
    insert_element(&mut v, &b);
    insert_element(&mut v, &c);
    insert_element(&mut v, &d);
    insert_element(&mut v, &e);

    // insert_element(&mut v, &d);

    println!("{:#?}", v);
}

fn insert_element<'a>(
    list: &mut Vec<&'a Element<'a>>,
    element: &'a Element,
) -> Result<(), ElemError> {
    if list.contains(&element) {
        println!("Element already in vector");
        return Ok(());
    }
    if let Some(x) = &element.reference {
        if let true = x.iter().all(|elem| list.contains(elem)) {
            list.push(element);
            return Ok(());
        }
        println!("References a non existent element!");
        return Err(ElemError::OtherError);
    }

    list.push(&element);

    return Ok(());
}
