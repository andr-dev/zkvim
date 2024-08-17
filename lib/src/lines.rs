use thiserror::Error;

#[derive(Clone, Debug)]
pub struct Lines {
    lines: Vec<String>,
}

impl Lines {
    pub fn new(text: String) -> Self {
        Self {
            lines: text.lines().map(ToString::to_string).collect(),
        }
    }

    pub fn max_line(&self) -> usize {
        self.lines.len().saturating_sub(1)
    }

    pub fn max_col(&self, line: usize) -> Result<usize, LineError> {
        let Some(line) = self.lines.get(line) else {
            return Err(LineError::PositionDoesNotExist {
                lines: self.clone(),
                line,
                col: usize::MAX,
            });
        };

        Ok(line.len().saturating_sub(1))
    }

    pub fn end_of_line(&self, line: usize) -> usize {
        let Some(line_str) = self.lines.get(line) else {
            return 0;
        };

        line_str.len()
    }

    pub fn insert_char(
        &mut self,
        line: usize,
        col: usize,
        char: char,
    ) -> Result<(usize, usize), LineError> {
        if self.lines.is_empty() {
            panic!("lines is empty :(");
        }

        let Some(line_str) = self.lines.get_mut(line) else {
            return Err(LineError::PositionDoesNotExist {
                lines: self.clone(),
                line,
                col,
            });
        };

        if char == '\n' {
            let new_line = line_str.split_off(col);
            self.lines.insert(line + 1, new_line);

            Ok((line.saturating_add(1), 0))
        } else {
            line_str.insert(col, char);

            Ok((line, col.saturating_add(1)))
        }
    }

    pub fn jump_to(&mut self, line: usize) -> (usize, usize) {
        if self.lines.is_empty() {
            return (0, 0);
        }

        let line = line.max(self.max_line());

        let mut col = 0;

        let line_str = self.lines.get(line).unwrap();

        while let Some(char) = line_str.chars().nth(col) {
            if char.is_whitespace() {
                col += 1
            } else {
                break;
            }
        }

        (line, col)
    }
}

impl ToString for Lines {
    fn to_string(&self) -> String {
        self.lines.join("\n")
    }
}

#[derive(Error, Debug)]
pub enum LineError {
    #[error("text position does not exist")]
    PositionDoesNotExist {
        lines: Lines,
        line: usize,
        col: usize,
    },
}
