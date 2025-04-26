trait Line{
    const VERT: char;
    const HORI: char;
    const TOP_LEFT: char;
    const TOP_RIGHT: char;
    const BOT_LEFT: char;
    const BOT_RIGHT: char;
}

struct ThinLine;
struct FatLine;
struct DoubleLine;

impl Line for ThinLine{
    const VERT: char = '│';
    const HORI: char = '─';
    const TOP_LEFT: char = '┌';
    const TOP_RIGHT: char = '┐';
    const BOT_LEFT: char = '└';
    const BOT_RIGHT: char = '┘';
}
impl Line for FatLine{
    const VERT: char = '┃';
    const HORI: char = '━';
    const TOP_LEFT: char = '┏';
    const TOP_RIGHT: char = '┓';
    const BOT_LEFT: char = '┗';
    const BOT_RIGHT: char = '┛';
}
impl Line for DoubleLine{
    const VERT: char = '║';
    const HORI: char = '═';
    const TOP_LEFT: char = '╔';
    const TOP_RIGHT: char = '╗';
    const BOT_LEFT: char = '╚';
    const BOT_RIGHT: char = '╝';
}

