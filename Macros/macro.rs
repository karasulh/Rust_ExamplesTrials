//1-Creating Macro
macro_rules! hello{
    () => {//matches no argument
           //all call to hello!() will be expanded to this
           println!("hello"); 
    };
}

fn main(){
    hello!();
}


//2-Macro_use
//Loads all macros from the "my_module_with_macros" module 
#[macro_use]
mod my_module_with_macros;

//Loads only my_macro! from my_crate_with_macros
#[macro_use(my_macro)]
extern crate my_crate_with_macros;


//3-Macro Example
extern crate hello_macro; //define the HelloMacro trait
#[macro_use]
extern crate hello_macro_derive; //define the derive


use hello_macro::HelloMacro;
struct Cat;

/*The #[derive(HelloMacro)] will implement this:
impl HelloMacro for Cat{
    fn hello_macro(){
        println!("Hello, Macro! I'm a Cat!");
    }
} */

fn main(){
    Cat::hello_macro();
}



//4-cfg!
fn main(){
    if cfg!(target_os = "macos"){
        println!("Run the MacOs-spesific code");
    }
    else if cfg!(target_os = "linux"){
        println!("Run the Linux-spesific code");
        LinuxCode {};
    }
    else{
        println!("We dont support this architecture");
    }
}

#[cfg(target_os = "linux")]
struct LinuxCode;


//5-unimplemented!
trait Animal{
    fn walk(&self);
    fn make_noise(&self);
}

struct Cat;

impl Animal for Cat{
    fn walk(&self){
        unimplemented!();
    }
    fn make_noise(&self){
        unimplemented!();
    }
}

fn main(){
    let cat = Cat {};
    cat.walk(); //thread 'main' panicked at 'not yet implemented'
}


//6-Assert!
fn main(){
    assert!(1==1);

    let x=true;
    assert!(x,"x isnot true"); //writes the reason of fail
    assert_eq!(1,1);

    let y=10;
    let z=20;
    assert_eq!(y,z);

    //Only run in un-optimized builds
    debug_assert!(1 == 1);
    debug_assert_eq!(1,2);
}