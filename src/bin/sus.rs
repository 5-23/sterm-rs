use sterm_rs::cursor::{goto, move_save, save_pos};

fn main() {
    // flash();
    goto(1, 1);
    println!("Hello, world!");
    println!("Hello, world!");
    println!("Hello, world!");
    println!("Hello, world!");
    save_pos();
    goto(3, 1);
    println!("XXXX");
    move_save();
}
