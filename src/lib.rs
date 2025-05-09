#![allow(dead_code)]
pub mod lines;
pub mod cursor;

use std::io::{self, Write};
use std::iter;
use lines::{CustomLine, Line};
use cursor::Cursor;

struct Canvas{
    start_line: i32,
    height: usize,
    start_column: i32,
    width: usize,
    cursor: Cursor,
    content: Vec<String>,
    border: Vec<Box<dyn Line>>,
}

impl Canvas {
    fn new(start_column: i32, start_line: i32, width: usize, height: usize) -> Self{
        let mut content: Vec<String> = Vec::new(); 
        for _ in 0..height{
            let line: String = "".to_string();
            content.push(line);
        }
        Canvas{
            start_line,
            height,
            cursor: Cursor::new(1, 1),
            start_column,
            width,
            content,
            border: Vec::new(),
        }
    }


    fn clear(mut self){
        for line in &mut self.content{
            *line = " ".repeat(self.width);
        }
        for layer in &mut self.border{
            *layer = Box::new(CustomLine(' '));
        }
        self.draw();
    }
    fn clear_content(&mut self){
        for line in &mut self.content{
            *line = " ".repeat(self.width);
        }
    }

    fn add_border<L: Line + 'static>(&mut self, border: L){
        self.border.push(Box::new(border));
    }

    fn draw(&mut self){
        //self.cursor.jump_up(self.cursor.curr_line - self.start_line - 1);
        self.cursor.move_to_line(self.start_line);
        self.cursor.move_to_col(0);

        self.draw_top_border();

        let left_border = self.vert_border();
        let right_border = left_border.chars().rev().collect::<String>();

        for line in &self.content{
            let buf = " ".repeat(self.width - line.len());
            self.cursor.jump_right(self.start_column);
            print!("{left_border}{line}{buf}{right_border}");
            self.cursor.jump_down(1);
            self.cursor.start_of_line();
        }

        self.draw_bottom_border();

        // self.cursor.move_to_line(self.height as i32 + self.border.len() as i32 * 2 + 1);
        self.reset_cursor();
        let _ = io::stdout().flush();
    }
    fn draw_top_border(&mut self) {
        if self.border.is_empty() {return;}

        let mut layers = self.border.len();
        let mut used = "".to_string();
        // let gap: String = " ".repeat(self.start_column as usize);

        for layer in &self.border{
            let mut top: String = iter::repeat(layer.hori()).take(self.width + layers * 2 - 2).collect();
            top.insert(0, layer.top_left());
            top.push(layer.top_right());

            layers -= 1;
            let used_rev = used.chars().rev().collect::<String>();
            self.cursor.jump_right(self.start_column - self.border.len() as i32);
            print!("{used}{top}{used_rev}");
            self.cursor.jump_down(1);
            self.cursor.start_of_line();

            used.push(layer.vert());
        }
        //self.cursor.jump_up(1);
    }
    fn draw_bottom_border(&mut self) {
        if self.border.is_empty() {return;}

        let mut layers = self.border.len();
        let mut used = self.vert_border();
        // let gap: String = " ".repeat(self.start_column as usize);
        
        for layer in self.border.iter().rev(){
            let mut top: String = iter::repeat(layer.hori()).take(self.width + (self.border.len() - layers + 1) * 2 - 2).collect();
            top.insert(0, layer.bot_left());
            top.push(layer.bot_right());
            
            layers -= 1;
            used.pop();
            let used_rev = used.chars().rev().collect::<String>();

            self.cursor.jump_right(self.start_column);
            print!("{used}{top}{used_rev}");
            self.cursor.jump_down(1);
            self.cursor.start_of_line();
        }
    }
    fn vert_border(&self) -> String{
        let mut res = "".to_string();
        for layer in &self.border{
            res.push(layer.vert());
        }
        res
    }

    fn write(&mut self, text: String, row: usize){
        if row >= self.height {return;}
        if text.len() > self.width{
            let fin_text = text.as_str()[0..self.width].to_string();
            self.content[row] = fin_text; 
        }
        else{
            self.content[row] = text;
        }
    }
    fn write_middle(&mut self, text: String, row: usize){
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
    fn write_right(&mut self, text: String, row: usize){
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

    fn get_cursor(&self) -> &Cursor{
        &self.cursor
    }
    fn reset_cursor(&mut self) {
        self.cursor.move_to_line(1);
        print!("\r");
        self.cursor.curr_line = 1;
        self.cursor.curr_col = 1;
    }

}


pub struct TextBox{
    canvas: Canvas,
}

impl TextBox{
    pub fn new(start_column: i32, start_line: i32, width: usize, height: usize) -> Self{ TextBox{canvas: Canvas::new(start_column, start_line, width, height)} }
    pub fn clear(self){ self.canvas.clear() }
    pub fn clear_content(&mut self){ self.canvas.clear_content() }
    pub fn add_border<L: Line + 'static>(&mut self, border: L){ self.canvas.add_border(border) }
    pub fn draw(&mut self){ self.canvas.draw() }
    pub fn write(&mut self, text: String, row: usize){ self.canvas.write(text, row) }
    pub fn write_middle(&mut self, text: String, row: usize){ self.canvas.write_middle(text, row) }
    pub fn write_right(&mut self, text: String, row: usize){ self.canvas.write_right(text, row) }
    pub fn get_cursor(&self) -> &Cursor { self.canvas.get_cursor() }
    pub fn reset_cursor(&mut self){ self.canvas.reset_cursor() }
}

