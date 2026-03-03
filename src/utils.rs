//! Multi-purpose utilities used throughout this crate.
//! 
//! These function focus mainly on escaping strings for 
//! [`crate::traits::WriteTriG`] implementations.
use std::io::{Result as IoResult, Write};

const LOCAL_ESCAPE_LUT: [bool; 256] = {
    let mut table = [false; 256];
    let targets = b"~.-!$&'()*+,;=/?#@%_";
    let mut i = 0;
    while i < targets.len() {
        table[targets[i] as usize] = true;
        i += 1;
    }
    table
};

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

/// This takes an impl [`Write`] and a [`str`] slice and escapes any characters 
/// that cannot be stored as local names (blank nodes, prefixes) in the TriG 
/// format. It writes the escape sequence of each character as it does so, to 
/// prevent allocations occuring anywhere other than the mandatory output 
/// buffer.
pub(crate) fn write_escaped_local_name<W: Write>(
    writer: &mut W, input: &str
) -> IoResult<()> {
    let bytes = input.as_bytes();
    let mut last_idx = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        if LOCAL_ESCAPE_LUT[byte as usize] {
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
}