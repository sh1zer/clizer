// use std::{thread::sleep, time::Duration};

use clizer::DrawingArea;
use clizer::lines::{DoubleLine, FatLine, ThinLine};

fn main(){
    let mut area = DrawingArea::new(9, 0, 5, 30);
    area.add_border(DoubleLine);
    area.add_border(ThinLine);
    area.add_border(FatLine);
    area.write("Hi :D".to_string(), 2);
    area.write_middle("hello".to_string(), 3);
    area.write("THIS STRING IS WAYYYYY TOOO LONG MAN WTF HOW CAN I FIT THIS IN THE BOX".to_string(), 4);
    area.write_middle("THIS STRING IS WAYYYYY TOOO LONG MAN WTF HOW CAN I FIT THIS IN THE BOX".to_string(), 1);
    area.draw();
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
