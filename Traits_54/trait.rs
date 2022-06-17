//1-Traits in arguments of template functions 
trait NoisyAnimal{
    fn make_noise(&self) -> &'static str;
}

struct Cat {}
impl NoisyAnimal for Cat{
    fn make_noise(&self) -> &'static str{
        "meow"
    }
}

struct Dog{}
impl NoisyAnimal for Dog{
    fn make_noise(&self) -> &'static str{
        "woof"
    }
}

fn make_noises(animals: Vec<Box<dyn NoisyAnimal>>){ //NoisyAnimal is trait object here.
    for animal in animals.iter(){
        println!("{}",animal.make_noise());
    }
}

fn main(){
    let animals: Vec<Box<dyn NoisyAnimal>> = vec![Box::new(Dog{}),Box::new(Cat{})];//use dyn for dynamic dispatch
    make_noises(animals);
}



//2-Tait example
trait Vehicle{
    fn new(name: &'static str) -> Self; //static method //Rust doesnot have constructor, set a new func.
    fn move(&self) ->(); //instance method //has a signature but no need to provide implementation
    fn to_string(&self){
        println!("vehicle {}",self.name)//default implementation
    }
}

struct Airplane{name: &'static str}
struct Bicycle{name: &'static str}
struct Car{name: &'static str}

impl Vehicle for Airplane{
    fn new(name: &'static str) -> Self{
        Airplane{name:name}
    }
    fn move(&self){
        self.fly();
    }
}

impl Vehicle for Bicycle{
    fn new(name: &'static str) -> Self{
        Bicycle{name : name}
    }
    fn move(&self){
        self.pedal();
    }
}

impl Vehicle for Car{
    fn new(name: &'static str) -> Self{
        Car{name:name}
    }
    fn move(&self){
        self.drive();
    }
}

fn from_amsterdam_to_paris< T:Vehicle>(vehicle:T){
    while(location_of(vehicle) != "Paris"){
        vehicle.move();
    }
}



//3-Associated type "type"
use std::fmt;

trait Iterator{
    type Item: fmt::Display; //type Item : associated type //it limits that the type should have Display trait
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter;
impl Iterator for Counter{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>{
        Some(42)//just for demo
    }
}

struct NoDisplayType; //Type without fmt::Display cant be used

// impl Iterator for Counter {
//     type Item = String; //gives error because above it is assigned

//     fn next(&mut self)-> Option<Self::Item>{
//         Some("42".to_string())//just for demo
//     }
// }

fn main(){
    let mut counter = Counter {};
    let n = counter.next();
    println!("{:?}",n);
}



//4-Derived Traits
trait X{}
trait Y{}
trait Z: X+Y {}

struct A;

impl X for A {} //must exist for impl Z for A due to Z:X+Y
impl Y for A {} //must exist for impl Z for A due to Z:X+Y
impl Z for A {}

fn main(){}



//5-Trait Object
pub struct TraitObject{
    pub data: *mut (), //point to concrete data type
    pub vtable: *mut (), //list of function pointers => for dynamic dispatch, it shows the dynamically created function according to its type 
}