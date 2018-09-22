use dialoguer::{Checkboxes, Confirmation, Input, Select};

fn main() {
    let checkboxes = &["Info", "Debug", "Error", "Critical"];
    let selections = Checkboxes::new().items(&checkboxes[..]).interact().unwrap();
    let selections2 = Select::new().items(&checkboxes[..]).interact();

    if selections2.is_err() {
        println!("You did not select anything :(");
    } else {
        println!("You selected {}:", selections2.unwrap());
    }
    if selections.is_empty() {
        println!("You did not select anything :(");
    } else {
        println!("You selected these things:");
        for selection in selections {
            println!("  {}", checkboxes[selection]);
        }
    }
    if Confirmation::new("Do you want to continue?")
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }

    let input = Input::new("Your name").interact().unwrap();
    println!("Hello {}!", input);

    if Confirmation::new("Do you want to continue?")
        .interact()
        .unwrap()
    {
        println!("Looks like you want to continue");
    } else {
        println!("nevermind then :(");
    }

    let file = Input::new("Enter file name : ").interact().unwrap();
    println!("Using  {}!", file);
}
