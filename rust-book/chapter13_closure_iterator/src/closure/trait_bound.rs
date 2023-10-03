#[test]
fn fn_once_string() {
    let x = String::from("x");
    // closure own the variable x and return x owner to the caller
    let closure = || -> String { x };
    println!("{}", closure());
}

#[test]
fn fn_once_vec() {
    let x = vec![1, 2, 3];
    let closure = || -> Vec<i32> { x };
    println!("{:?}", closure());
}

#[test]
fn fn_mut_i32() {
    let mut x = 0;
    let mut closure = || {
        x += 1;
    };
    closure();
    closure();
    println!("{}", x);
}

#[test]
fn fn_mut_string() {
    let mut x = String::from("x");
    let mut closure = || x.push_str(" append string");
    closure();
    closure();
    println!("modified by closure x={}", x);
}

#[test]
fn fn_string() {
    let x = String::from("x");
    let closure = || println!("in closure x={}", x);
    closure();
    closure();
    println!("out of closure x={}", x);
}

#[test]
fn no_capture() {
    // it can write explicit type here for closure
    let closure: fn() -> () = || println!("in closure without capture");
    closure();
    closure();
}
