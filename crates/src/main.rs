// nested modules
use garden::{
    fruits::{Banana, Strawberry},
    vegetables::{Asparagus, Potato},
};

mod garden;

fn main() {
    let asparagus = Asparagus { health: 8 };
    let potato = Potato { health: 8 };
    let banana = Banana { health: 8 };
    let strawberry = Strawberry { health: 8 };

    println!("This is an asparagus! {:#?}", asparagus);
    println!("This is a potato! {:#?}", potato);
    println!("This is a banana! {:#?}", banana);
    println!("This is a strawberry! {:#?}", strawberry);
}
