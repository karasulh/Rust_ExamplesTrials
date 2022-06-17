fn foo() -> i32{
    let t:i32 = 15;
    t
}

fn do_something(_b: i32){
    //_b = 5;
    println!("b: {}", _b);
}

fn do_something_else(_b: &i32){
   // _b = 20;
   println!("b: {}", _b);
}

fn main(){
    // //a owns the value
    // let a = foo();
    // {
    //     //b owns the value
    //     let b : &i32 = &a;
    //     println!("b: {}", b);

    //     //ownerhsip passed to do_something
    //     do_something(a);

    //     println!("b: {}", b);

    //     //This gives error.
    //     do_something_else(b);
    // }
    // //ownerhsip passed to do_something
    // do_something(a);

    let mut a =10;
    let b = &mut a;
    *b= *b + 1;

    let c= &a;
    *b=*b+1;


    let mut values = vec![1,2,3,4,5];

    let h = &values;
    let p = &values;

    //it gives error, because we tried to change variable while there are some immutable references.  
    values[2] = 3432;
}






// // This function takes ownership of a box and destroys it
// fn eat_box_i32(boxed_i32: Box<i32>) {
//     println!("Destroying box that contains {}", boxed_i32);
// }

// fn eat_i32(_i32: i32) {
//     println!("Destroying box that contains {}", _i32);
// }

// // This function borrows an i32
// fn borrow_i32(borrowed_i32: &i32) {
//     println!("This int is: {}", borrowed_i32);
// }

// fn main() {
//     // Create a boxed i32, and a stacked i32
//     let boxed_i32 = Box::new(5_i32);
//     let stacked_i32 = 6_i32;

//     // Borrow the contents of the box. Ownership is not taken,
//     // so the contents can be borrowed again.
//     borrow_i32(&boxed_i32);
//     borrow_i32(&stacked_i32);

//     {
//         // Take a reference to the data contained inside the box
//         let _ref_to_i32: &i32 = &boxed_i32;
//         //let _ref_stacked_i32: &i32 = &stacked_i32;

//         // Error!
//         // Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
//         eat_box_i32(boxed_i32);
//        // eat_i32(stacked_i32);
//         // FIXME ^ Comment out this line

//         // Attempt to borrow `_ref_to_i32` after inner value is destroyed
//         //borrow_i32(_ref_stacked_i32);
//         borrow_i32(_ref_to_i32);
//         // `_ref_to_i32` goes out of scope and is no longer borrowed.
//     }

//     // `boxed_i32` can now give up ownership to `eat_box` and be destroyed
//     eat_box_i32(boxed_i32);
//     //eat_i32(stacked_i32);
// }





    // //**It is true for a dynamic variable ( like newed Box ), not normal variable( like i32 )**
    // //a owns the value
    // let a = foo(); 

    // //b owns the value
    // let b = a;

    // //ownership passed to do_something
    // do_something(b);

    // //This gives error.
    // do_something_else(b); //X


    // //a owns the value
    // let a = foo(); 

    // //a still owns the value, only immutable borrow
    // let b = &a;

    // //functions can still use it if they only need a borrow:
    // do_something(&a);
    // //this gives error:
    // do_something(a); //X


    // let mut a=10;
    // lete b= &mut a;
    // *b=*b+1;
    // //It gives error: Can't have mutable and immutable borrow at the same time.
    // let c= &a;
    // *b=*b+1; //when we try to reach b, it will give error due to above reason.





    

    // function munge_file(){
    //     let mut file = File::Create("file.txt")?;
    //     file.write_all(b"Hello, world");
    // }//file closed automatically



    // let a = vec![103,7,234,774];
    // let sla = &a[1..2]; //&[7,234]

    // let b= String::from("Hello");
    // let slb = &b[0..2]; //"Hel"


