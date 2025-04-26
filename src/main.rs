// use std::{thread::sleep, time::Duration};

use clizer::Canvas;
use clizer::lines::{DoubleLine, FatLine, ThinLine};

fn main(){
    let mut area = Canvas::new(9, 0, 5, 30);
    area.add_border(DoubleLine);
    area.add_border(' ');
    // area.add_border(ThinLine);
    // area.add_border(FatLine);
    area.write("Text on the left".to_string(), 2);
    area.write_right("Text on the right".to_string(), 1);
    area.write_middle("Text in the middle".to_string(), 0);
    area.write("THIS STRING IS WAYYYYY TOOO LONG MAN WTF HOW CAN I FIT THIS IN THE BOX".to_string(), 4);
    area.draw();


    let mut area2 = Canvas::new(49, 0, 5, 20);
    area2.add_border(DoubleLine);
    area2.write_middle("box two".to_string(), 0);
    area2.draw();
    // sleep(Duration::from_secs(4));
    // area.clear();
    // area.add_border('0');
    // area.draw();
    // sleep(Duration::from_secs(4));
    // println!("hi\nhello\nwhadup");
    // print!("\x1B[2A");
    // sleep(Duration::from_secs(1));
    // println!("nevermind...");

}
