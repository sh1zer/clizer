use std::{thread::sleep, time::Duration};

use clizer::DrawingArea;
mod cursor;

fn main(){

    let mut area = DrawingArea::new(9, 0, 5, 30);
    area.add_border('█');
    area.draw();
    sleep(Duration::from_secs(4));
    area.clear();
    // area.add_border('0');
    // area.draw();
    // sleep(Duration::from_secs(4));
    // println!("hi\nhello\nwhadup");
    // print!("\x1B[2A");
    // sleep(Duration::from_secs(1));
    // println!("nevermind...");

}
