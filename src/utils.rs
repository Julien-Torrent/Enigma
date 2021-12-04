pub fn char_to_usize(c: char) -> usize {
    return c as usize - 65;
}

pub fn usize_to_char(i: usize) -> char {
    ((i % 26) as u8 + 65) as char
}
