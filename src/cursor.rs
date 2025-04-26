pub struct Cursor{
    pub curr_line:i32,
    pub curr_col:i32,
}

impl Cursor{
    pub fn new(curr_line: i32, curr_col: i32) -> Self{
        Cursor{curr_line, curr_col}
    }
    
    pub fn reset(&mut self){
        self.curr_line = 1;
        self.curr_col = 1;
    }
    pub fn jump_up(&mut self, offset:i32){
        self.curr_line -= offset;
        print!("\x1B[{offset}A");
    }

    pub fn jump_down(&mut self, offset:i32){
        self.curr_line += offset;
        print!("\x1B[{offset}B");
    }

    pub fn jump_left(&mut self, offset: i32){
        self.curr_col -= offset;
        print!("\x1B[{offset}D");
    }

    pub fn jump_right(&mut self, offset: i32){
        self.curr_col += offset;
        print!("\x1B[{offset}C");
    }


    pub fn start_of_line(&mut self){
        self.curr_col = 1;
        print!("\r");
    }

    ///offset is inverse to the "intuitive" line value, i.e. after printing two lines you should
    ///add 2  to offset, even though you're 2 lines lower
    fn adjust(&mut self, offset: i32){
        self.curr_line += offset;
    }

    pub fn move_to_line(&mut self, new_line: i32){
        if self.curr_line > new_line{
            self.jump_up(self.curr_line - new_line);
        }
        else if self.curr_line < new_line{
            self.jump_down(new_line - self.curr_line);
        }
        self.curr_line = new_line;
    }

    pub fn move_to_col(&mut self, col: i32){
        self.curr_col = col;
        print!("\r\x1B[{col}C");
    }
}
pub fn cursor_jump_up(offset:i32){
    if offset < 1 {return;}
    print!("\x1B[{offset}A");
}

pub fn cursor_jump_down(offset:i32){
    if offset < 1 {return;}
    print!("\x1B[{offset}B");
}

pub fn cursor_jump_left(offset: i32){
    if offset < 1 {return;}
    print!("\x1B[{offset}D");
}

pub fn cursor_jump_right(offset: i32){
    if offset < 1 {return;}
    print!("\x1B[{offset}C");
}

pub fn cursor_move_to_col(col: i32){
    if col < 1 {return;}
    print!("\r\x1B[{col}C");
}
