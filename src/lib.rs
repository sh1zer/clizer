#![allow(dead_code)]
mod lines;
use std::iter;

pub struct DrawingArea{
    start_line: u32,
    height: u32,
    start_column: u32,
    width: u32,
    cursor: Cursor,
    content: Vec<String>,
    border: String,
}

impl DrawingArea{
    pub fn new(start_column: u32, start_line: u32, height: u32, width: u32) -> Self{
        let mut content: Vec<String> = Vec::new(); 
        for _ in 0..height{
            let line: String = " ".repeat(width as usize);
            content.push(line);
        }
        DrawingArea{
            start_line,
            height,
            cursor: Cursor::new(0),
            start_column,
            width,
            content,
            border: "".to_string(),
        }
    }

    pub fn clear(mut self){
        for line in &mut self.content{
            *line = " ".repeat(self.width as usize);
        }
        self.border = " ".repeat(self.border.len());
        self.draw();
    }

    pub fn clear_content(&mut self){
        for line in &mut self.content{
            *line = " ".repeat(self.width as usize);
        }
    }

    pub fn add_border(&mut self, border: char){
        self.border.push(border);
    }

    pub fn draw(&mut self){
        self.cursor.jump_up(self.cursor.curr_line - self.start_line as i32 - 1);

        self.draw_top_border();
        
        let gap: String = " ".repeat(self.start_column as usize);
        let reverse = self.border.chars().rev().collect::<String>();
        for line in &self.content{
            println!("{gap}{}{line}{}", self.border, reverse);
        }
        
        self.draw_bottom_border();

        self.cursor.set_line(self.height as i32 + self.border.len() as i32 * 2 + 1);
    }

    fn draw_top_border(&mut self) {
        let mut layers = self.border.len();
        let mut used = "".to_string();
        let mut used_rev = used.clone();
        let gap: String = " ".repeat(self.start_column as usize);
        for layer in self.border.chars(){
            let top: String = iter::repeat(layer).take(self.width as usize + layers * 2).collect();
            layers -= 1;
            println!("{gap}{used}{top}{used_rev}");
            used.push(layer);
            used_rev.insert(0, layer);
        }
    }

    fn draw_bottom_border(&mut self) {
        let mut layers = self.border.len();
        let mut used = self.border.clone();
        let gap: String = " ".repeat(self.start_column as usize);
        for layer in self.border.chars().rev(){
            let top: String = iter::repeat(layer).take(self.width as usize + (self.border.len() - layers + 1) * 2).collect();
            layers -= 1;
            used.pop();
            let used_rev = used.chars().rev().collect::<String>();
            println!("{gap}{used}{top}{used_rev}");
        }
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

    ///offset is inverse to the "intuitive" line value, i.e. after printing two lines you should
    ///add 2  to offset, even though you're 2 lines lower
    fn adjust(&mut self, offset: i32){
        self.curr_line += offset;
    }

    fn set_line(&mut self, new_line: i32){
        self.curr_line = new_line;
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::cursor::{jump, Cursor};
//
//     use super::*;
//
//     #[test]
//     fn piss_off(){
//         let area = DrawingArea::new(2, 2);
//         assert_eq!(area.num_of_lines, 2);
//         assert_eq!(area.start_line, 2);
//     }
//     #[test]
//     fn cursor_jumpin(){
//         println!("hello\nhi\nwhaddup");
//         println!("nevermind");
//     }
// }
