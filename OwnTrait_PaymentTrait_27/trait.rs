struct MasterCard {
    number:u8,
    verification:u8
}
struct Visa{
    number:u32
}
struct WesternUnion {
    name:String,
    verification:u8
}
struct BitCredit{
    btcnumber:u32
}

impl CreditCharge for BitCredit{
    fn charge_with_id(&self,id:u32)-> bool{
        id%2 == self.btcnumber%2 
    }
}

impl CreditCharge for Visa{
    fn charge_with_id(&self,id:u32)-> bool{
        id == self.number 
    }
}

trait CreditCharge{
    fn charge_with_id(&self, id:u32) -> bool;
}

// //Generic Functions
// fn print<T:Display>(t:T){
//     println!("{}",t);
// }

// print(12);
// print("Hello World");
// print(ourCustomDisplayThing);

fn transact<Q:CreditCharge>(card:Q){
    //get verif code from user here
    let id=4096;
    if card.charge_with_id(id){
        println!("Success Transact");
    }
    else{
        panic!("Invalid code!");
    }
}

fn main(){
    let card = BitCredit {btcnumber:1024};
    let card2 = Visa {number:1024};
    let card3 = MasterCard {number:15,verification:25};
    let code = 4096;
    let code2 = 1024;
    if card.charge_with_id(code){
        println!("Success!");
    }
    else{
        println!("Failure!");
    }

    if card2.charge_with_id(code2){
        println!("Success2!");
    }
    else{
        println!("Failure2!");
    }

    transact(card);//generic function
}


