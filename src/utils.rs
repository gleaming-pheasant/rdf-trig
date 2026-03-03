//! Multi-purpose utilities used throughout this crate.
//! 
//! These function focus mainly on escaping strings for 
//! [`crate::traits::WriteTriG`] implementations.
use std::io::{Result as IoResult, Write};

// 256 is all possible u8 options. This const creates a single true byte for 
// each matching character in the targets table, based on the u8 value of the 
// character.
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

/// This takes an impl [`Write`] and a [`str`] slice and escapes any characters 
/// that cannot be stored as local names (blank nodes, prefixes) in the TriG 
/// format. It writes the escape sequence of each character as it does so, to 
/// prevent allocations occuring anywhere other than the mandatory output 
/// buffer.
pub(crate) fn write_trig_escaped_local_name<W: Write>(
    writer: &mut W, input: &str
) -> IoResult<()> {    
    let bytes = input.as_bytes();
    let mut last_idx = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        if LOCAL_ESCAPE_LUT[bytes[i] as usize] {
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
    fn test_escape_one_char() {
        let mut buf = vec![];
        write_trig_escaped_local_name(
            &mut buf, "myweird*prefix"
        ).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from("myweird\\*prefix")
        )
    }

    #[test]
    fn test_escape_multiple_chars() {
        let mut buf = vec![];
        write_trig_escaped_local_name(
            &mut buf, "my/weird*prefix"
        ).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from("my\\/weird\\*prefix")
        )
    }

    #[test]
    fn test_escape_no_chars() {
        let mut buf = vec![];
        write_trig_escaped_local_name(
            &mut buf, "myprefix"
        ).unwrap();

        assert_eq!(
            String::from_utf8(buf).unwrap(),
            String::from("myprefix")
        )
    }
}