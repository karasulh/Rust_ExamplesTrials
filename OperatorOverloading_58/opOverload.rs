//1-PartialEq
// '==' and '!=' operator: PartialEq
use std::cmp::PartialEq;

struct Point{
    x:i32,
    y:i32
}

impl PartialEq for Point{
    fn eq(&self, other:&Point) -> bool{
        (self.x == other.x) && (self.y == other.y)
    }
}

fn main(){
    let p1=Point{x:10,y:10};
    let p2=Point{x:10,y:10};
    println!("p1==p2: {}",p1==p2);
}

//2- Overloading '+'(Add)
pub trait Add<RHS=Self>{
   //           ^^^^Default Generic
    type Output; //Associate type
    fn add(self,rhs:RHS) -> Self::Output;
}

struct Point{
    x:i32,
    y:i32
}

impl Add for Point{//RHS is left as default
    type Output = Point; //Associate eype

    fn Add(self, other:Point)->Point{
        Point{
            x:self.x+other.x,
            y:self.y+other.y
        }
    }
}


//3-Overloading Drop(Destructor)
use std::ops::Drop;

struct MyType;

impl Drop for MyType{
    fn drop(&mut self){
        println!("Dropping");
        //Do some cleanup here
    }
}

fn main(){
    println!("Declaring");
    let x = MyType;
    println!("About to exit main");
}


//4-Conversion between MyNumber and i32
#[derive(Debug)]
struct MyNumber{
    value:i32,
}

//from i32 to MyNumber
impl From<i32> for MyNumber{
    fn from(number:i32) -> Self{
        MyNumber{ value: number}
    }
}

//When we implements From, we get the corresponding
//"impl Into<MyNumber> for i32" for free

fn main(){
    //Convert an i32 into MyNumber using From<i32>
    let num1 = MyNumber::from(42i32);

    //Convert an i32 into MyNumber using Into<MyNumber>
    let num2:MyNumber = 42i32.into();

    println!("{:?}",num1);
    println!("{:?}",num2);
}