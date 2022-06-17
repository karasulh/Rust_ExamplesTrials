// mod animal{
//     pub fn move_around(){}

//     pub mod cat{
//         pub fn meow() {}
//     }

//     pub mod dog{
//         pub fn bark(){}
//     }
// }

/*animal::move_around()
animal::cat::moew() */

mod animal;

fn main(){
    animal::animal::move_around();
    animal::animal::cat::meow();
}