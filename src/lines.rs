
enum ThinLine{
    Vert = '│' as isize,
    Hori = '─' as isize,
    TopLeft = '┌' as isize,
    TopRight = '┐' as isize,
    BotLeft = '└' as isize,
    BotRight = '┘' as isize,
}
enum FatLine{
    Vert = '┃' as isize,
    Hori = '━' as isize,
    TopLeft = '┏' as isize,
    TopRight = '┓' as isize,
    BotLeft = '┗' as isize,
    BotRight = '┛' as isize,
}
enum DoubleLine{
    Vert = '║' as isize,
    Hori = '═' as isize,
    TopLeft = '╔' as isize,
    TopRight = '╗' as isize,
    BotLeft = '╚' as isize,
    BotRight = '╝' as isize,
}

trait Line{

}
