#[test]
fn add() {
    let result = finger::calc("1 + 1".to_owned());
    assert_eq!(result, "2");
}

#[test]
fn sub() {
    let result = finger::calc("234 - 1".to_owned());
    assert_eq!(result, "233");
}

#[test]
fn mul() {
    let result = finger::calc("128 * 2".to_owned());
    assert_eq!(result, "256");
}

#[test]
fn div() {
    let result = finger::calc("1024 / 2".to_owned());
    assert_eq!(result, "512");
}

#[test]
fn add_mul() {
    let result = finger::calc("1 + 1 * 2".to_owned());
    assert_eq!(result, "3");
}

#[test]
fn add_div() {
    let result = finger::calc("2 + 4 / 2".to_owned());
    assert_eq!(result, "4");
}

#[test]
fn random() {
    let a = finger::calc("rand()".into());
    let b = finger::calc("rand()".into());
    assert_ne!(a, b);
}