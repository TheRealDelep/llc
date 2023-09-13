pub struct FileStream {
    lines: Vec<FileLine>,
    current_index: usize,
}

pub struct FileLine {
    pub number: usize,
    pub content: Vec<char>,
    pub current_index: usize,
}

impl FileStream {
    pub fn new(content: &str) -> FileStream {
        FileStream {
            lines: content
                .split('\n')
                .enumerate()
                .map(|it| FileLine::new(it.1, it.0))
                .collect(),

            current_index: 0,
        }
    }

    pub fn get_next<'a>(&'a mut self) -> Option<&mut FileLine> {
        match self.lines.get_mut(self.current_index) {
            Some(i) => {
                self.current_index += 1;
                Some(i)
            }
            None => None,
        }
    }
}

impl FileLine {
    pub fn new(content: &str, number: usize) -> FileLine {
        FileLine {
            number,
            content: content.chars().collect(),
            current_index: 0,
        }
    }

    pub fn get_next<'a>(&'a mut self) -> Option<&char> {
        match self.content.get(self.current_index) {
            Some(i) => {
                self.current_index += 1;
                Some(i)
            }
            None => None,
        }
    }

    pub fn can_read(&self) -> bool {
        self.current_index < self.content.len()
    }

    pub fn backtrack(&mut self, steps: usize) {
        self.current_index = self.current_index.saturating_sub(steps);
    }
}
