pub struct FileWriter {
    pub content: String,
    pub indentation_lvl: u8,
}

impl FileWriter {
    pub fn new() -> Self {
        FileWriter {
            content: String::default(),
            indentation_lvl: 0,
        }
    }

    pub fn append_str(&mut self, value: &str) -> &mut Self {
        self.content.push_str(value);
        self
    }

    pub fn append(&mut self, value: &char) -> &mut Self {
        self.content.push(*value);
        self
    }

    pub fn append_space(&mut self) -> &mut Self {
        self.append(&' ')
    }

    pub fn append_line(&mut self) -> &mut Self {
        self.append(&'\n');
        for _ in 0..self.indentation_lvl {
            self.append(&'\t');
        }

        self
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn indent_right(&mut self) -> &mut Self {
        self.indentation_lvl += 1;
        self
    }

    pub fn indent_left(&mut self) -> &mut Self {
        if self.indentation_lvl != 0 {
            self.indentation_lvl -= 1;
        }

        self
    }
}
