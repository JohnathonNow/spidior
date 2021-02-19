use std::error::Error;
/// A buffer for holding text, supporting operations
/// for replacement of text as well as appending
struct TextBuffer {
    buf: String,
}

impl TextBuffer {
    /// Creates a new text buffer
    ///
    /// # Returns
    ///
    /// A text buffer...
    pub fn new() -> Self {
        Self { buf: String::new() }
    }
    /// Replaces the text in the buffer starting at `start` and
    /// extending for `length` characters with the text from `replacement`.
    ///
    ///
    /// # Arguments
    ///
    /// * `start` - The index into the buffer we start replacing from
    /// * `length` - The length of the replacement we are performing
    /// * `replacement` - The new text to put in the buffer
    ///
    /// # Returns
    ///
    /// A Result<(), Box<dyn Error>>, where on success, it returns a
    /// unit. It will Err if you attempt to replace more text than
    /// exists in the buffer.
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
    /// Appends some text from `s` to the end of the buffer.
    ///
    ///
    /// # Arguments
    ///
    /// * `s` - The text we are appending
    /// exists in the buffer.
    pub fn add(&mut self, s: &str) {
        self.buf += s;
    }
    /// Returns a reference to the contents of the buffer
    ///
    ///
    /// # Returns
    ///
    /// A String reference with the buffer contents
    pub fn read(&self) -> &String {
        &self.buf
    }
    /// Consumes the TextBuffer, returning the contents
    ///
    ///
    /// # Returns
    ///
    /// A String with the buffer contents
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
