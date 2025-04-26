pub struct Cursor{
    curr_line:i32,
    curr_col:i32,
}

impl Cursor{
    pub fn new(curr_line: i32, curr_col: i32) -> Self{
        Cursor{curr_line, curr_col}
    }

    pub fn jump_up(&mut self, offset:i32){
        if self.curr_line - offset < 0{
            return;
        }
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

    pub fn move_to_col(&mut self, col: i32){
        self.curr_col = col;
        print!("\r\x1B[{col}C");
    }

    ///offset is inverse to the "intuitive" line value, i.e. after printing two lines you should
    ///add 2  to offset, even though you're 2 lines lower
    fn adjust(&mut self, offset: i32){
        self.curr_line += offset;
    }

    pub fn move_to_line(&mut self, new_line: i32){
        self.curr_line = new_line;
    }
}
