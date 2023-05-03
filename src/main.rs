enum ElemError {
    InsertionError,
    OtherError,
}

#[derive(Debug)]
#[allow(unused)]
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
    insert_element(&mut v, &e);

    // v.push(&a);
    // v.push(&b);
    // v.push(&c);
    // v.push(&d);

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
        for i in x.iter() {
            if !list.contains(i) {
                println!("References a non existent element!");
                return Err(ElemError::InsertionError);
            } else {
                list.push(element);
                return Ok(());
            }
        }
        // return Err(ElemError::InsertionError);
    }

    list.push(&element);

    return Ok(());
}
