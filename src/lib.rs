#![allow(dead_code)]

mod cursor;

pub struct DrawingArea{
    start_line: u32,
    num_of_lines: u32,
}

impl DrawingArea{
    fn new(start_line: u32, num_of_lines: u32) -> Self{
        DrawingArea{start_line, num_of_lines}
    }


}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn piss_off(){
        let area = DrawingArea::new(2, 2);
        assert_eq!(area.num_of_lines, 2);
        assert_eq!(area.start_line, 2);
    }
}
