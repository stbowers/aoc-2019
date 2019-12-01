use std::fs::File;
use std::io::Read;

pub fn read_lines<'a>(
    filename: &str,
    buffer: &'a mut String,
    filter_blank_lines: bool,
) -> std::io::Result<Vec<&'a str>> {
    let mut file: File = File::open(filename)?;

    // read file into contents
    file.read_to_string(buffer)?;

    // get an iterator for lines in the file (contents split along newline char, remove trailing whitespace)
    let input_strings: Vec<&str> = buffer
        .split("\n")
        .map(|line: &str| -> &str {
            return line.trim_end();
        }).filter(|line| {
            if filter_blank_lines {
                line.len() > 0
            } else {
                true
            }
        }).collect();

    return Ok(input_strings);
}
