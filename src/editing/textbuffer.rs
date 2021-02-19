use std::error::Error;
struct TextBuffer {
    buf: String,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self { buf: String::new() }
    }

    pub fn replace(
        &mut self,
        start: usize,
        length: usize,
        replacement: &str,
    ) -> Result<(), Box<dyn Error>> {
        if self.buf.len() < start + length {
            return Err("Replacing more of the string than exists".into());
        }
        self.buf = format!(
            "{}{}{}",
            &self.buf[..start],
            replacement,
            &self.buf[start + length..]
        )
        .to_string();
        Ok(())
    }

    pub fn add(&mut self, s: &str) {
        self.buf += s;
    }

    pub fn read(&self) -> &String {
        &self.buf
    }

    pub fn consume(self) -> String {
        self.buf
    }
}

#[test]
fn buffer_add() {
    let mut tb = TextBuffer::new();
    tb.add("hello");
    tb.add(" world!");
    assert_eq!("hello world!", tb.read());
    assert_eq!("hello world!", tb.consume());
}

#[test]
fn buffer_replace() {
    let mut tb = TextBuffer::new();
    tb.add("hello world!");
    tb.replace(1, 3, "ooooo").unwrap();
    assert_eq!("hoooooo world!", tb.read());
    tb.replace(0, 1, "b").unwrap();
    assert_eq!("boooooo world!", tb.read());
    tb.replace(13, 1, "?").unwrap();
    assert_eq!("boooooo world?", tb.read());
}
