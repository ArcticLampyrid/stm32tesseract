use std::fmt::Display;
use vte::Perform;

pub struct OffScreenTerminal {
    width: usize,
    // For we need to iterate over chars instead of bytes, we store each line as Vec<char> instead of String
    lines: Vec<Vec<char>>,
    cursor: (usize, usize),
    writer: Box<dyn std::io::Write>,
    error: Option<std::io::Error>,
}

impl OffScreenTerminal {
    pub fn new(width: usize, writer: Box<dyn std::io::Write>) -> Self {
        Self {
            width,
            lines: Vec::new(),
            cursor: (0, 0),
            writer,
            error: None,
        }
    }
    pub fn take_last_error(&mut self) -> Option<std::io::Error> {
        self.error.take()
    }
}

impl Display for OffScreenTerminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            let mut trailing = 0;
            for c in line.iter().rev() {
                if c.is_whitespace() {
                    trailing += 1;
                } else {
                    break;
                }
            }
            for c in line[..line.len() - trailing].iter() {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Perform for OffScreenTerminal {
    fn print(&mut self, c: char) {
        if self.cursor.1 >= self.lines.len() {
            self.lines.resize(self.cursor.1 + 1, Default::default());
        }
        let line = &mut self.lines[self.cursor.1];
        while self.cursor.0 > line.len() {
            line.push(' ');
        }
        if self.cursor.0 == line.len() {
            line.push(c);
        } else {
            line[self.cursor.0] = c;
        }
        self.cursor = (self.cursor.0 + 1, self.cursor.1);

        // reach the max width, move to the next line
        if self.cursor.0 >= self.width {
            self.cursor = (0, self.cursor.1 + 1);
        }
    }
    fn execute(&mut self, byte: u8) {
        match byte {
            0x08 => {
                // Backspace
                if self.cursor.0 > 0 {
                    self.cursor.0 -= 1;
                    let line = &mut self.lines[self.cursor.1];
                    line.remove(self.cursor.0);
                }
            }
            0x0A => {
                // Line Feed
                self.cursor = (self.cursor.0, self.cursor.1 + 1);
            }
            0x0D => {
                // Carriage Return
                self.cursor = (0, self.cursor.1);
            }
            _ => {}
        }
    }
    fn csi_dispatch(
        &mut self,
        params: &vte::Params,
        _intermediates: &[u8],
        _ignore: bool,
        action: char,
    ) {
        match action {
            'n' if params.len() == 1 && params.iter().next().unwrap() == [6] => {
                // Device Status Report
                let _ = self
                    .writer
                    .write_all(
                        format!("\x1b[{};{}R", self.cursor.1 + 1, self.cursor.0 + 1).as_bytes(),
                    )
                    .map_err(|e| self.error.replace(e));
            }
            'A' => {
                // Cursor Up
                let n = params.iter().next().unwrap_or(&[1])[0];
                self.cursor = (self.cursor.0, self.cursor.1.saturating_sub(n as usize));
            }
            'B' => {
                // Cursor Down
                let n = params.iter().next().unwrap_or(&[1])[0];
                self.cursor = (self.cursor.0, self.cursor.1 + n as usize);
            }
            'C' => {
                // Cursor Forward
                let n = params.iter().next().unwrap_or(&[1])[0];
                self.cursor = (
                    (self.cursor.0 + n as usize).min(self.width - 1),
                    self.cursor.1,
                );
            }
            'D' => {
                // Cursor Backward
                let n = params.iter().next().unwrap_or(&[1])[0];
                self.cursor = (self.cursor.0.saturating_sub(n as usize), self.cursor.1);
            }
            'H' | 'f' => {
                // Cursor Position
                let mut params = params.iter();
                let y = params.next().unwrap_or(&[1])[0].saturating_sub(1) as usize;
                let x = (params.next().unwrap_or(&[1])[0].saturating_sub(1) as usize)
                    .min(self.width - 1);
                self.cursor = (x, y);
            }
            'J' => match params.iter().next().unwrap_or(&[0]) {
                [0] => {
                    // clear from cursor to end of screen
                    self.lines.drain(self.cursor.1 + 1..);
                    let line = &mut self.lines[self.cursor.1];
                    line.truncate(self.cursor.0);
                }
                [1] => {
                    // clear from cursor to beginning of the screen
                    self.lines.drain(..self.cursor.1);
                    let line = &mut self.lines[self.cursor.1];
                    for c in line.iter_mut().take(self.cursor.0) {
                        *c = ' ';
                    }
                }
                [2] | [3] => {
                    // clear entire screen
                    self.lines.clear();
                }
                _ => {}
            },
            'K' => {
                let line = &mut self.lines[self.cursor.1];
                match params.iter().next().unwrap_or(&[0]) {
                    [0] => {
                        // clear from cursor to the end of the line
                        line.truncate(self.cursor.0);
                    }
                    [1] => {
                        // clear from cursor to beginning of the line
                        line.drain(..self.cursor.0);
                    }
                    [2] => {
                        // clear entire line
                        line.clear();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
