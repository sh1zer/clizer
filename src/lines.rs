pub trait Line{
    fn vert(&self) -> char;
    fn hori(&self) -> char;
    fn top_left(&self) -> char;
    fn top_right(&self) -> char;
    fn bot_left(&self) -> char;
    fn bot_right(&self) -> char;
}

#[derive(Debug, Clone, Copy)]
pub struct ThinLine;
#[derive(Debug, Clone, Copy)]
pub struct FatLine;
#[derive(Debug, Clone, Copy)]
pub struct DoubleLine;
#[derive(Debug, Clone, Copy)]
pub struct CustomLine(pub char);

impl Line for ThinLine{
    fn vert(&self) -> char { '│' }
    fn hori(&self) -> char { '─' }
    fn top_left(&self) -> char { '┌' }
    fn top_right(&self) -> char { '┐' }
    fn bot_left(&self) -> char { '└' }
    fn bot_right(&self) -> char { '┘' }
}
impl Line for FatLine{
    fn vert(&self) -> char { '┃' }
    fn hori(&self) -> char { '━' }
    fn top_left(&self) -> char { '┏' }
    fn top_right(&self) -> char { '┓' }
    fn bot_left(&self) -> char { '┗' }
    fn bot_right(&self) -> char { '┛' }
}
impl Line for DoubleLine{
    fn vert(&self) -> char { '║' }
    fn hori(&self) -> char { '═' }
    fn top_left(&self) -> char { '╔' }
    fn top_right(&self) -> char { '╗' }
    fn bot_left(&self) -> char { '╚' }
    fn bot_right(&self) -> char { '╝' }
}
impl Line for CustomLine{
    fn vert(&self) -> char { self.0 }
    fn hori(&self) -> char { self.0 }
    fn top_left(&self) -> char { self.0 }
    fn top_right(&self) -> char { self.0 }
    fn bot_left(&self) -> char { self.0 }
    fn bot_right(&self) -> char { self.0 }
}
impl Line for char{
    fn vert(&self) -> char { *self }
    fn hori(&self) -> char { *self }
    fn top_left(&self) -> char { *self }
    fn top_right(&self) -> char { *self }
    fn bot_left(&self) -> char { *self }
    fn bot_right(&self) -> char { *self }
}


