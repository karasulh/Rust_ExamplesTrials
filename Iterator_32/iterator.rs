fn main(){
    let v= [1,2,3,4,5,6,7];
    let mut iterator = v.into_iter().skip(2);
    println!("{:?}",iterator.next());

    let mut taken = iterator.take(2);
    println!("{:?}",taken.next());
    println!("{:?}",taken.next());
    println!("{:?}",taken.next());
    //println!("{:?}",iterator.next()); //gives error due to iterator moves to taken

    let mut iterator3 = vec!['a','b','c'].into_iter();
    let mut enumerated = iterator3.enumerate();
    println!("{:?}",enumerated.next());
    println!("{:?}",enumerated.next());

    let mut taken = (1..10).into_iter().skip(2).take(4);
    let v: Vec<i32> = taken.collect(); //3,4,5,6
    println!("{:?}",v);

    let items = (1..10).into_iter();
    let other_items:Vec<i32> = items.map(|x|{ x+1 }).collect();
    println!("{:?}",other_items);

    // let items2 = (1..10).into_iter();
    // let other_items2: Vec<i32> = items2
    //     .map(|x| {function1(x,12)})
    //     .map(|x| {function2(x,"Hello")})
    //     .map(|x| {function3(x) })
    //     .collect();
        
    let items3 = (1..10).into_iter();
    let other_items3: Vec<i32> = items3
        .filter(|x| {x%2 ==0})
        .collect(); //[2,4,6,8]


    let items4 = (1..10).into_iter();
    let result= items4.fold(0,|sum,item| { sum+item }); //starting point, closure
    println!("{:?}",result);//45


    let items5 = (1..100).into_iter();
    let result2 = items5
        .filter(|x| {x%2==1} )
        .map(|x| {x*x} )
        .filter(|x| {x%5!=0} )
        .fold(0,|sum,x| {sum+x} );
        
    println!("{:?}",result2);//133400
       
}