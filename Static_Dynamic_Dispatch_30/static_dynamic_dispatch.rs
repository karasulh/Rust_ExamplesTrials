//STATIC DISPATCH
trait Foo { fn method(&self) -> String; }
impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }
impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }
fn do_something<T: Foo>(x: T) {
    x.method();
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    do_something(x);
    do_something(y);
}

//At compile-time, the above code transforms to the below code ==>
trait Foo { fn method(&self) -> String; }
impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }
impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }
fn do_something_u8(x: u8) {
    x.method();
}

fn do_something_string(x: String) {
    x.method();
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    do_something_u8(x);
    do_something_string(y);
}


//DYNAMIC DISPATCH
//With below codes, we can write codes in dynamic dispatch, so functions are not transformed as static dispatch.
//First Way:
trait Foo { fn method(&self) -> String; }
impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }
impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }
fn do_something(x: &Foo) { // &Foo => Dynamic Dispatch
    x.method();
}

fn main() {
    let x = 5u8;
    do_something(&x as &Foo);
}
//Second Way:
trait Foo { fn method(&self) -> String; }
impl Foo for u8 { fn method(&self) -> String { format!("u8: {}", *self) } }
impl Foo for String { fn method(&self) -> String { format!("string: {}", *self) } }
fn do_something(x: &Foo) { //&Foo => Dynamic Dispatch
    x.method();
}

fn main() {
    let x = "Hello".to_string();
    do_something(&x);
}



// //Closure
// let val=10;
// let ourClosure= |x| {x+val}
// println!("{}",ourClosure(2)); //12