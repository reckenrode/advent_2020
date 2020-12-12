use std::{ fmt, str };

pub const SEAT: u8 = 'L' as u8;
pub const PERSON: u8 = '#' as u8;
pub const FLOOR: u8 = '.' as u8;

#[derive(Debug, PartialEq)]
pub struct WaitingArea {
    pub width: usize,
    pub height: usize,
    grid: Vec<u8>
}

impl WaitingArea {
    pub fn apply_rules(&mut self, f: impl FnOnce(&mut [u8], usize, usize)) {
        f(&mut self.grid, self.width, self.height)
    }

    pub fn parse(grid: impl AsRef<str>) -> Option<WaitingArea> {
        let grid = grid.as_ref();
        let mut rows = grid.lines().peekable();
        let width = rows.peek()?.len();
        let (height, valid) = rows
            .map(|s| s.len())
            .fold((0, true), |(height, valid), w| {
                (height + 1, valid && w == width)
            });
        if valid {
            Some (WaitingArea {
                width: width,
                height: height,
                grid: grid.replace('\n', "").as_bytes().to_vec(),
            })
        } else {
            None
        }
    }
}

impl fmt::Display for WaitingArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chunks: Result<Vec<&str>, str::Utf8Error> = self.grid.chunks(self.width)
            .map(str::from_utf8)
            .collect();
        let chunks = chunks.or(Err(fmt::Error))?;
        let formatted_str = chunks.join("\n");
        write!(f, "{}", formatted_str)
    }
}
