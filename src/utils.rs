//! Multi-purpose utilities used throughout this crate.
//! 
//! These function focus mainly on escaping strings for 
//! [`crate::traits::WriteTriG`] implementations.
use std::io::{Result as IoResult, Write};

const LITERAL_ESCAPE_SYMBOLS: [u8; 256] = {
    let mut table = [0u8; 256];
    table[0x08] = b'b'; // Backspace (shouldn't be encountered, only ASCII)
    table[0x0C] = b'f'; // Form Feed (as above)
    table[b'\t' as usize] = b't';
    table[b'\n' as usize] = b'n';
    table[b'\r' as usize] = b'r';
    table[b'\"' as usize] = b'\"';
    table[b'\'' as usize] = b'\'';
    table[b'\\' as usize] = b'\\';
    table
};

/// This takes an impl [`Write`] and a [`str`] slice and escapes any characters 
/// that cannot be stored in literals (only string literals, as other types are 
/// known to not contain invalid values) in the TriG format. It writes the 
/// escape sequence of each character as it does so, to prevent allocations 
/// occuring anywhere other than the mandatory output buffer.
/// 
/// This function DOES NOT escape sequences which appear in the TriG 
/// specification, but which are invalid unicode (\b for backspace and \f for 
/// form feed).
#[inline]
pub(crate) fn write_escaped_literal<W: Write>(
    writer: &mut W, input: &str
) -> IoResult<()> {
    let bytes = input.as_bytes();
    let mut last_idx = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        let symbol = LITERAL_ESCAPE_SYMBOLS[byte as usize];
        
        if symbol != 0 {
            if i > last_idx {
                writer.write_all(&bytes[last_idx..i])?;
            }
            
            writer.write_all(&[b'\\', symbol])?;
            
            last_idx = i + 1;
        }
    }

    if last_idx < bytes.len() {
        writer.write_all(&bytes[last_idx..])?;
    }

    Ok(())
}

const LOCAL_ESCAPE_LU: [bool; 256] = {
    let mut table = [false; 256];
    let targets = b"~.-!$&'()*+,;=/?#@%_";
    let mut i = 0;
    while i < targets.len() {
        table[targets[i] as usize] = true;
        i += 1;
    }
    table
};

/// This takes an impl [`Write`] and a [`str`] slice and escapes any characters 
/// that cannot be stored as local names (blank nodes, prefixes) in the TriG 
/// format. It writes the escape sequence of each character as it does so, to 
/// prevent allocations occuring anywhere other than the mandatory output 
/// buffer.
#[inline]
pub(crate) fn write_escaped_local_name<W: Write>(
    writer: &mut W, input: &str
) -> IoResult<()> {
    let bytes = input.as_bytes();
    let mut last_idx = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        if LOCAL_ESCAPE_LU[byte as usize] {
            if i > last_idx {
                writer.write_all(&bytes[last_idx..i])?;
            }
            
            writer.write_all(&[b'\\', byte])?;
            last_idx = i + 1;
        }
    }

    if last_idx < bytes.len() {
        writer.write_all(&bytes[last_idx..])?;
    }

    Ok(())
}

/// All characters to be escaped are set as true in the map. Includes 
/// non-printable, non-ascii and "unsafe" URL characters.
const URL_NEEDS_ESCAPE: [bool; 256] = {
    let mut table = [false; 256];
    let mut i = 0;
    while i < 256 { // Non-printable.
        if i <= 0x1F || i == 0x7F {
            table[i] = true;
        }
        if i >= 0x80 { // Non-ASCII
            table[i] = true;
        }
        i += 1;
    }
    table[b'>' as usize] = true; // URL "unsafe"
    table[b'<' as usize] = true;
    table[b'"' as usize] = true;
    table[b'{' as usize] = true;
    table[b'}' as usize] = true;
    table[b'|' as usize] = true;
    table[b'^' as usize] = true;
    table[b'[' as usize] = true;
    table[b']' as usize] = true;
    table[b'{' as usize] = true;
    table[b'\\' as usize] = true;
    table[b'`' as usize] = true;
    table
};

const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF"; // Lookup for zero-allocation.

/// This takes an impl [`Write`] and a [`str`] slice and escapes any characters 
/// that cannot be stored as part of an IRI/URL.
/// 
/// This involves escaping non-printable ASCII characters, URL "unsafe" 
/// characters (<, >, ", {, etc.) and non-ASCII characters.
/// 
/// This function does not validate URL structure (such as ensuring a schema is 
/// present), because of RDF's flexible interpretation of URLs (e.g. through 
/// allowance of urn/uuid "schemas").
pub(crate) fn write_escaped_url_component<W: Write>(
    writer: &mut W, input: &str
) -> IoResult<()> {
    let bytes = input.as_bytes();
    let mut last_idx = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        if URL_NEEDS_ESCAPE[byte as usize] {
            if i > last_idx {
                writer.write_all(&bytes[last_idx..i])?;
            }

            let encoded = [
                b'%',
                HEX_DIGITS[(byte >> 4) as usize],   // High nibble
                HEX_DIGITS[(byte & 0x0F) as usize], // Low nibble
            ];
            writer.write_all(&encoded)?;

            last_idx = i + 1;
        }
    }

    if last_idx < bytes.len() {
        writer.write_all(&bytes[last_idx..])?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_local_one_char() {
        let mut buf = vec![];
        write_escaped_local_name(
            &mut buf, "myweird*prefix"
        ).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from(r"myweird\*prefix")
        )
    }

    #[test]
    fn test_escape_local_multiple_chars() {
        let mut buf = vec![];
        write_escaped_local_name(
            &mut buf, "my/weird*prefix"
        ).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from(r"my\/weird\*prefix")
        )
    }

    #[test]
    fn test_escape_local_no_chars() {
        let mut buf = vec![];
        write_escaped_local_name(
            &mut buf, "myprefix"
        ).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from(r"myprefix")
        )
    }

    #[test]
    fn test_escape_literal_one_char() {
        let mut buf = vec![];
        write_escaped_literal(
            &mut buf, "my broken\tliteral\x08"
        ).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from(r"my broken\tliteral\b")
        )
    }

    #[test]
    fn test_escape_literal_multiple_chars() {
        let mut buf = vec![];
        write_escaped_literal(
            &mut buf, "my\r\nbroken\tliteral"
        ).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from(r"my\r\nbroken\tliteral")
        )
    }

    #[test]
    fn test_escape_literal_no_chars() {
        let mut buf = vec![];
        write_escaped_literal(
            &mut buf, "myliteral"
        ).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from(r"myliteral")
        )
    }

    #[test]
    fn test_escape_url_no_chars() {
        let mut buf = vec![];
        write_escaped_url_component(
            &mut buf, "http://www.example.com/my_url"
        ).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from(r"http://www.example.com/my_url")
        )
    }

    #[test]
    fn test_escape_url_non_printable_chars() {
        let mut buf = vec![];
        write_escaped_url_component(
            &mut buf, "http://www.example.com/\r\nmy_url"
        ).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from(r"http://www.example.com/%0D%0Amy_url")
        )
    }

    #[test]
    fn test_escape_url_unsafe_chars() {
        let mut buf = vec![];
        write_escaped_url_component(
            &mut buf, "http://www.example.com/|my_url|"
        ).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from(r"http://www.example.com/%7Cmy_url%7C")
        )
    }

    #[test]
    fn test_escape_url_non_ascii_chars() {
        let mut buf = vec![];
        write_escaped_url_component(
            &mut buf, "http://www.example.com/|my_Ȗrl|"
        ).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from(r"http://www.example.com/%7Cmy_%C8%96rl%7C")
        )
    }
}