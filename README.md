# encoding bufreader

This is a bufreader with ecoding with wrap a BufReader inner.

# install
```
cargo add encodingbufreader
```
# usage
```
use encodingbufreader::{BufReaderEncoding};
use encoding::all::{UTF_8,GB18030};
let bytes = "This string\nwill be read".as_bytes();

let mut lines_iter = BufReaderEncoding::new(bytes,UTF_8).map(|l| l.unwrap());
assert_eq!(lines_iter.next(), Some(String::from("This string")));
assert_eq!(lines_iter.next(), Some(String::from("will be read")));
assert_eq!(lines_iter.next(), None);

let bytes: &[u8] = &[
            213, 226, 202, 199, 210, 187, 184, 246, 215, 214, 183, 251, 180, 174, 10, 189, 171,
            187, 225, 177, 187, 182, 193, 200, 161,
        ];
for line in BufReaderEncoding::new(bytes, GB18030)
            .lines()
            .map(|l| l.unwrap()){
    println!("{}",line);
            }
```