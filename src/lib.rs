pub mod write {
    use super::*;
    use std::io;
    use std::io::prelude::*;

    const FLAG: u8 = b'>' - b'"';

    macro_rules! escaping_body {
        ($start:ident, $i:ident, $writer:ident, $bytes:ident, $quote:expr) => {{
            if $start < $i {
                $writer.write_all(&$bytes[$start..$i])?;
            }
            $writer.write_all($quote)?;
            $start = $i + 1;
        }};
    }

    pub fn html<W: Write>(writer: &mut W, bytes: &[u8]) -> io::Result<()> {
        let mut start = 0;
        for (i, b) in bytes.iter().enumerate() {
            if b.wrapping_sub(b'"') <= FLAG {
                match *b {
                    b'<' => escaping_body!(start, i, writer, bytes, b"&lt;"),
                    b'>' => escaping_body!(start, i, writer, bytes, b"&gt;"),
                    b'&' => escaping_body!(start, i, writer, bytes, b"&amp;"),
                    _ => (),
                }
            }
        }
        writer.write_all(&bytes[start..])
    }

    pub fn attr_value<W: Write>(writer: &mut W, bytes: &[u8], quote: Quote) -> io::Result<()> {
        let byte = quote.as_byte();
        let entity = quote.as_entity();
        let mut start = 0;
        for (i, b) in bytes.iter().enumerate() {
            if b.wrapping_sub(b'"') <= FLAG {
                match *b {
                    b'&' => escaping_body!(start, i, writer, bytes, b"&amp;"),
                    b'"' => escaping_body!(start, i, writer, bytes, b"&quot;"),
                    b'\'' => escaping_body!(start, i, writer, bytes, b"&#x27;"),
                    _ => {
                        if *b == byte {
                            escaping_body!(start, i, writer, bytes, entity)
                        }
                    }
                }
            }
        }
        writer.write_all(&bytes[start..])
    }
}

pub fn html(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut writer = Vec::with_capacity(bytes.len());
    write::html(&mut writer, bytes).unwrap();
    unsafe { String::from_utf8_unchecked(writer) }
}

pub enum Quote {
    Double,
    Single,
}

impl Quote {
    fn as_byte(&self) -> u8 {
        match self {
            Quote::Double => b'"',
            Quote::Single => b'\'',
        }
    }

    fn as_entity(&self) -> &[u8] {
        match self {
            Quote::Double => b"&quot;",
            Quote::Single => b"&#x27;",
        }
    }
}

pub fn attr_value(value: &str, quote: Quote) -> String {
    let bytes = value.as_bytes();
    let mut writer = Vec::with_capacity(bytes.len());
    write::attr_value(&mut writer, bytes, quote).unwrap();
    unsafe { String::from_utf8_unchecked(writer) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_escape() {
        assert_eq!(html(""), "");
        assert_eq!(html("<&>"), "&lt;&amp;&gt;");
        assert_eq!(html("bla&"), "bla&amp;");
        assert_eq!(html("<foo"), "&lt;foo");
        assert_eq!(attr_value("bla&h", Quote::Double), "bla&amp;h");
    }
}
