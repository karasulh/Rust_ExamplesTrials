// #[derive(Debug)] 
// struct MyStruct;

// /* 
// //with Error
// #[allow(unused_variables)]
// pub fn main(){
//     let x = MyStruct{};
//     let y = x; //non-primitive variable types are moved with this.
//     //println!("{:?}",x);//gives error: x has moved into y, x isnot usable anymore
// }
// */


// //1.way to use copy for MyStruct
// //pub trait Copy: Clone{} //Clone is supertrait of Copy
// //Copy is trivial bitwise copy, can't be overloaded
// //Notice it's empty, this is what we called marker trait
// impl Copy for MyStruct{} //marker trait => tell compiler that we need copy behaviour

// //Clone is explicit memory copy, usually expensive
// impl Clone for MyStruct{
//     fn clone(&self)->MyStruct{
//         *self
//     }
// }
// #[allow(unused_variables)]
// pub fn main(){
//     let x=MyStruct{};
//     let y=x; //y is a copy of x, x is not moved
//     println!("{:?}",x);
// }


// //2.way to use copy for MyStruct
// #[derive(Debug,Copy,Clone)] //Automatically implemented by compiler
// struct MyStruct;

// #[allow(unused_variables)]
// pub fn main(){
//     let x=MyStruct{};
//     let y=x; //y is a copy of x, x is not moved
//     println!("{:?}",x);
// }


// //2-Sized Trait
// #![no_main]

// struct HasSizedT<T>(T); //T:Sized by default
// struct NotSizedT<T: ?Sized>(T); //?Sized =>shows "not known Sized"

// //[i32] is not Sized
// struct NoOK(HasSizedT<[i32]>);
// //                ^^^^^^^^ [i32] doesnot have a constant size known at compile time

// struct OK(NotSizedT<[i32]>);


// //3-Default
// #[derive(Default,Debug)]
// struct MyData{
//     int_field: i32,
//     float_field: f32,
// }

// fn main(){
//     //Use default values for all the fields
//     let data1: MyData = Default::default();
//     println!("{:?}",data1);

//     //Specify part of fields and use the defaults for the rest
//     let data2 = MyData {
//        int_field:42, 
//        ..Default::default(); //Use default for the rest
//     }
//     println!("{:?}",data2);
// }


//4-Default for Enum
#[allow(dead_code)]

#[derive(Debug)]
enum Pet{
    Cat,
    Dog,
    Bird,
}
impl Default for Pet{
    fn default() -> Pet{
        Pet::Cat
    }
}

fn main(){
    let default_pet:Pet = Default::default();
    println!("{:?}",default_pet);
}