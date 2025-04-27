#![allow(unused_imports, unused_braces)]
use std::{thread::sleep, time::Duration};
use clizer::{TextBox};
use clizer::lines::{DoubleLine, FatLine, ThinLine};
use clizer::cursor;
fn main(){
    for _ in 0..10{
        println!();
    }
    cursor::cursor_jump_up(10);
    let mut area = TextBox::new(9, 1, 20, 5);
    area.add_border(DoubleLine);
    // area.add_border(ThinLine);
    // area.add_border(FatLine);
    area.write("Text on the left".to_string(), 2);
    area.write_right("Text on the right".to_string(), 1);
    area.write_middle("Text in the middle".to_string(), 0);
    area.write("THIS STRING IS WAYYYYY TOOO LONG MAN WTF HOW CAN I FIT THIS IN THE BOX".to_string(), 4);
    area.draw();
    sleep(Duration::from_secs(2));
    

    
    let mut area2 = TextBox::new(49, 1, 20, 5);
    sleep(Duration::from_secs(1));
    area2.add_border(DoubleLine);
    area2.write_middle("box two".to_string(), 0);
    area2.draw();
    sleep(Duration::from_secs(2));

    area2.write_middle("oh yeah!".to_string(), 2);

    area.write_middle("wooohooo".to_string(), 2);
    area.draw();

    area2.draw();

    area.draw();

    cursor::cursor_jump_down(1000);
    println!();
    // sleep(Duration::from_secs(4));
    // area.clear();
    // area.add_border('0');
    // area.draw();
    // sleep(Duration::from_secs(4));
    // println!("hi\nhello\nwhadup");
    // print!("\x1B[2A");
    // println!("nevermind...");

}
