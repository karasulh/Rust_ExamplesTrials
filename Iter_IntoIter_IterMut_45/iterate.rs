fn main(){
    
    //1-We must convert some lists to iterator to use in loop 
    for element in 1..10{
        println!("{}",element); //OK
    }

    let myArray=[1,2,3,4];//vector is transformed to iterator automatically by compiler
    for element in myArray.into_iter(){
        println!("{}",element); //OK
    }
    for element in myArray{ //Not an iterator //but array can not be transformed to iterator automatically by compiler
        //println!("{}",element); // Not Okay
    }



    //2-Differences of into_iter, iter, iter_mut
    let mut mylist = vec!(1,2,3,4);
    for element in mylist.into_iter(){
        //element is i32
        println!("{}",element)
    }
    //println!("{}",mylist[0]); //Error, value used here, after move

    mylist = vec!(1,2,3,4);
    for element in mylist.iter(){
        //element is &i32
        println!("{}",*element);
    }
    println!("{}",mylist[0]); //OK

    mylist = vec!(1,2,3,4);
    for element in mylist.iter_mut(){
        //element is &mut i32
        *element = *element +1;
        println!("{}",*element);
    }
    println!("{}",mylist[0]); //OK





    //3-Iterator Adaptors,Functional Programming Loop(iterator loop)
    let mut numbers= vec![1,2,3,4];
    let plus_one = numbers.iter().map(|x| x+1 );
    plus_one.for_each(|x| println!("{}",x));//Learn this usage type

    numbers = vec![1,2,3,4];
    let max: &i32 = numbers.iter().max().unwrap(); //max returns Option,so to take real value of it, use unwrap! 
    println!("{}",max);

    
    //lazy evaulation of iterator loop
    let numbers: Vec<i32> = (1..) //1 to infinity
        .map(|x| x + 1) //to infinity => lazy evaluation, so not need to calculate all
        .map(|x| x * x) //to infinity => lazy evaluation, so not need to calculate all
        .take(5)//take only first five
        .collect(); //"now need to calculate", but only 5 of them    
    println!("{:?}",numbers);

    let numbers2 = vec![1,2,3,4];
    let plus_one_iter = numbers.iter().map(|x|x+1);
    println!("{:?}",plus_one_iter); //return Map{ iter: Iter([1,2,3,4]) => lazy evaluation, so not calculated yet

    let plus_one:Vec<i32> = plus_one_iter.collect(); //"now need to calculate" it calculates to creat vector due to "collect"
    println!("{:?}",plus_one);//[2,3,4,5]
}