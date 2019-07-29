use encoding::{DecoderTrap, EncodingRef};
use std::io::{self, BufRead, BufReader, Result};

#[derive(Debug)]
pub struct Lines<R>
where
    R: io::Read,
{
    buf: BufReaderEncoding<R>,
}

impl<R: io::Read> Iterator for Lines<R> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Result<String>> {
        let mut buf = String::new();
        match self.buf.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                if buf.ends_with("\n") {
                    buf.pop();
                    if buf.ends_with("\r") {
                        buf.pop();
                    }
                }
                Some(Ok(buf))
            }
            Err(e) => Some(Err(e)),
        }
    }
}
pub struct BufReaderEncoding<R> {
    encoder: EncodingRef,
    inner: BufReader<R>,
    buf: Vec<u8>,
}

impl<R> std::fmt::Debug for BufReaderEncoding<R>
where
    R: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<R: io::Read> BufReaderEncoding<R> {
    pub fn new(inner: R, encoder: EncodingRef) -> BufReaderEncoding<R> {
        BufReaderEncoding {
            encoder: encoder,
            inner: BufReader::new(inner),
            buf: Vec::new(),
        }
    }
    pub fn with_capacity(cap: usize, inner: R, encoder: EncodingRef) -> BufReaderEncoding<R> {
        BufReaderEncoding {
            encoder: encoder,
            inner: BufReader::with_capacity(cap, inner),
            buf: Vec::new(),
        }
    }
    fn append_to_string(&mut self, buf: &mut String) -> Result<usize> {
        let len = buf.len();
        let ret = self.inner.read_until(b'\n', &mut self.buf);

        if self
            .encoder
            .decode_to(&self.buf[len..], DecoderTrap::Replace, buf)
            .is_err()
        {
            ret.and_then(|_| {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "stream did not contain valid character",
                ))
            })
        } else {
            self.buf.clear();
            ret
        }
    }
    /// Returns an iterator over the lines of this reader.
    ///
    /// The iterator returned from this function will yield instances of
    /// [`io::Result`]`<`[`String`]`>`. Each string returned will *not* have a newline
    /// byte (the 0xA byte) or CRLF (0xD, 0xA bytes) at the end.
    ///
    /// [`io::Result`]: type.Result.html
    /// [`String`]: ../string/struct.String.html
    ///
    /// # Examples
    ///
    /// [`std::io::Cursor`][`Cursor`] is a type that implements `BufRead`. In
    /// this example, we use [`Cursor`] to iterate over all the lines in a byte
    /// slice.
    ///
    /// [`Cursor`]: struct.Cursor.html
    ///
    /// ```
    /// use encodingbufreader::{BufReaderEncoding};
    /// use encoding::all::UTF_8;
    /// let bytes = "This string\nwill be read".as_bytes();
    ///
    /// let mut lines_iter = BufReaderEncoding::new(bytes,UTF_8).map(|l| l.unwrap());
    /// assert_eq!(lines_iter.next(), Some(String::from("This string")));
    /// assert_eq!(lines_iter.next(), Some(String::from("will be read")));
    /// assert_eq!(lines_iter.next(), None);
    /// ```
    ///
    /// # Errors
    ///
    /// Each line of the iterator has the same error semantics as [`BufRead::read_line`].
    ///
    /// [`BufReaderEncoding::read_line`]: BufReaderEncoding.html#method.read_line
    pub fn lines(self) -> Lines<R> {
        Lines { buf: self }
    }
    pub fn read_line(&mut self, buf: &mut String) -> Result<usize> {
        self.append_to_string(buf)
    }
    pub fn set_encoder(&mut self, encoder: encoding::EncodingRef) {
        self.encoder = encoder;
    }
}

impl<R: io::Read> io::BufRead for BufReaderEncoding<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt);
    }
}
impl<R: io::Read> io::Read for BufReaderEncoding<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}
#[cfg(test)]
mod tests {
    use super::BufReaderEncoding;
    use encoding::all::{GB18030, UTF_8};

    #[test]
    fn test_decodeuft8() {
        let bytes = "This string\nwill be read".as_bytes();
        let mut lines_iter = BufReaderEncoding::new(bytes, UTF_8)
            .lines()
            .map(|l| l.unwrap());
        assert_eq!(lines_iter.next(), Some(String::from("This string")));
        assert_eq!(lines_iter.next(), Some(String::from("will be read")));
        assert_eq!(lines_iter.next(), None);
    }
    #[test]
    fn test_decode_gb18030() {
        let bytes: &[u8] = &[
            213, 226, 202, 199, 210, 187, 184, 246, 215, 214, 183, 251, 180, 174, 10, 189, 171,
            187, 225, 177, 187, 182, 193, 200, 161,
        ];
        let mut lines_iter = BufReaderEncoding::new(bytes, GB18030)
            .lines()
            .map(|l| l.unwrap());
        assert_eq!(
            lines_iter.next(),
            Some(String::from("这是一个字符串"))
        );
        assert_eq!(lines_iter.next(), Some(String::from("将会被读取")));
        assert_eq!(lines_iter.next(), None);
    }
}
