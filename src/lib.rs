#![allow(dead_code)]
pub mod lines;
pub mod cursor;
use std::iter;
use lines::{CustomLine, Line};
use cursor::Cursor;

pub struct Canvas{
    start_line: u32,
    height: usize,
    start_column: u32,
    width: usize,
    cursor: Cursor,
    content: Vec<String>,
    border: Vec<Box<dyn Line>>,
}

impl Canvas {
    pub fn new(start_column: u32, start_line: u32, height: usize, width: usize) -> Self{
        let mut content: Vec<String> = Vec::new(); 
        for _ in 0..height{
            let line: String = "".to_string();
            content.push(line);
        }
        Canvas{
            start_line,
            height,
            cursor: Cursor::new(start_line as i32, start_column as i32),
            start_column,
            width,
            content,
            border: Vec::new(),
        }
    }

    pub fn clear(mut self){
        for line in &mut self.content{
            *line = " ".repeat(self.width);
        }
        for layer in &mut self.border{
            *layer = Box::new(CustomLine(' '));
        }
        self.draw();
    }

    pub fn clear_content(&mut self){
        for line in &mut self.content{
            *line = " ".repeat(self.width);
        }
    }

    pub fn add_border<L: Line + 'static>(&mut self, border: L){
        self.border.push(Box::new(border));
    }

    pub fn draw(&mut self){
        self.cursor.jump_up(self.cursor.curr_line - self.start_line as i32 - 1);

        self.draw_top_border();

        let gap: String = " ".repeat(self.start_column as usize);
        let left_border = self.vert_border();
        let right_border = left_border.chars().rev().collect::<String>();

        for line in &self.content{
            let buf = " ".repeat(self.width - line.len());
            println!("{gap}{left_border}{line}{buf}{right_border}");
        }

        self.draw_bottom_border();

        self.cursor.move_to_line(self.height as i32 + self.border.len() as i32 * 2 + 1);
    }

    fn draw_top_border(&mut self) {
        if self.border.is_empty() {return;}

        let mut layers = self.border.len();
        let mut used = "".to_string();
        let gap: String = " ".repeat(self.start_column as usize);

        for layer in &self.border{
            let mut top: String = iter::repeat(layer.hori()).take(self.width + layers * 2 - 2).collect();
            top.insert(0, layer.top_left());
            top.push(layer.top_right());

            layers -= 1;
            let used_rev = used.chars().rev().collect::<String>();
            println!("{gap}{used}{top}{used_rev}");
            used.push(layer.vert());
        }
    }
    fn draw_bottom_border(&mut self) {
        if self.border.is_empty() {return;}

        let mut layers = self.border.len();
        let mut used = self.vert_border();
        let gap: String = " ".repeat(self.start_column as usize);
        
        for layer in self.border.iter().rev(){
            let mut top: String = iter::repeat(layer.hori()).take(self.width + (self.border.len() - layers + 1) * 2 - 2).collect();
            top.insert(0, layer.bot_left());
            top.push(layer.bot_right());
            
            layers -= 1;
            used.pop();
            let used_rev = used.chars().rev().collect::<String>();

            println!("{gap}{used}{top}{used_rev}");
        }
    }
    fn vert_border(&self) -> String{
        let mut res = "".to_string();
        for layer in &self.border{
            res.push(layer.vert());
        }
        res
    }

    pub fn write(&mut self, text: String, row: usize){
        if row >= self.height {return;}
        if text.len() > self.width{
            let fin_text = text.as_str()[0..self.width].to_string();
            self.content[row] = fin_text; 
        }
        else{
            self.content[row] = text;
        }
    }

    pub fn write_middle(&mut self, text: String, row: usize){
        if row >= self.height {return;}
        if text.len() > self.width{
            let fin_text = text.as_str()[0..self.width].to_string();
            self.content[row] = fin_text; 
        }
        else{
            let buf = " ".repeat((self.width - text.len()) / 2);
            self.content[row] = format!("{buf}{text}{buf}");
        }
    }

    pub fn write_right(&mut self, text: String, row: usize){
        if row >= self.height {return;}
        if text.len() > self.width{
            let fin_text = text.as_str()[0..self.width].to_string();
            self.content[row] = fin_text; 
        }
        else{
            let buf = " ".repeat(self.width - text.len());
            self.content[row] = format!("{buf}{text}");
        }
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
