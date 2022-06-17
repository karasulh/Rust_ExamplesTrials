/* 
enum Option<T>{
    Some(T),
    None
}

enum Result<T,E>{
    Ok(T),
    Err(E),
}
*/

/*impl<T> Vec<T>{
    pub const fn new() -> Vec<T>
} */

fn first_element(list:Vec<i32>) -> Option<i32>{ //Option is generally used for return value for generic functions
    if list.len() == 0{
        None
    } else{
        Some(list[0])
    }
}

fn get_middle(list:Vec<i32>) -> Result<i32,&'static str>{
    match list.len(){
        0 => Err("empty list"),
        x if x%2 == 0 =>Err("list has even number of elements"),
        _ => Ok(list[list.len()/2]),
    }
}

fn main(){
    let some_integer = Option::Some(42);
    let some_float = Option::Some(3.14);
    let none: Option<i32> = Option::None; //we must define the type T

    let list= vec!(1,2,3,4);

    match first_element(list){
        None => println!("empty list!"),
        Some(x) => println!("The first element is {}",x),
    }
    match first_element(Vec::new()){
        None => println!("empty list"),
        Some(x) => println!("The first element is {}",x),
    }

    println!("{:?}",get_middle(vec![1,2,3]));
    println!("{:?}",get_middle(Vec::new()));


    let mut list1: Vec<i32> = Vec::new();
    let mut list2 = Vec::new();//Type inference
    let mut list3 = Vec::<i32>::new(); //turbofish::<>

} 