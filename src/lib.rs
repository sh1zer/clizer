#![allow(dead_code)]

use std::iter;

mod cursor;

pub struct DrawingArea{
    start_line: u32,
    height: u32,
    start_column: u32,
    width: u32,
    cursor: Cursor,
    content: Vec<String>,
}

impl DrawingArea{
    pub fn new(start_line: u32, height: u32, width: u32) -> Self{
        let mut content: Vec<String> = Vec::new(); 
        for _ in 0..height{
            let line: String = " ".repeat(width as usize);
            content.push(line);
        }
        DrawingArea{
            start_line,
            height,
            cursor: Cursor::new(0),
            start_column: 0,
            width,
            content,
        }
    }

    pub fn draw_outline(&mut self, border: char){
        let top: String = iter::repeat(border).take(self.width as usize + 2).collect();
        
        self.cursor.jump_up(self.cursor.curr_line);
        println!("{top}");
        for line in &self.content{
            println!("{border}{line}{border}");
        }
        println!("{top}");
        
    }

}

struct Cursor{
    curr_line:i32,
}

impl Cursor{
    fn new(curr_line: i32) -> Self{
        Cursor{curr_line}
    }

    fn jump_up(&mut self, offset:i32){
        if self.curr_line - offset < 0{
            return;
        }
        self.curr_line -= offset;
        print!("\x1B[{offset}A");
    }

    fn jump_down(&mut self, offset:i32){
        self.curr_line += offset;
        print!("\x1B[{offset}B");
    }

}

#[cfg(test)]
mod tests {
    use crate::cursor::{jump, Cursor};

    use super::*;

    #[test]
    fn piss_off(){
        let area = DrawingArea::new(2, 2);
        assert_eq!(area.num_of_lines, 2);
        assert_eq!(area.start_line, 2);
    }
    #[test]
    fn cursor_jumpin(){
        println!("hello\nhi\nwhaddup");
        println!("nevermind");
    }
}
