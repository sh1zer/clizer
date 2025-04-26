use std::{thread::sleep, time::Duration};

use clizer::DrawingArea;
mod cursor;

fn main(){

    println!("hi\nhello\nwhadup");
    print!("\x1B[3A");
    sleep(Duration::from_secs(1));
    println!("nevermind...");

    let mut area = DrawingArea::new(4, 5, 20);
    area.draw_outline('#');
}
