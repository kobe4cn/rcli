use std::io::Read;

pub fn reader_from_input(input: &str) -> anyhow::Result<String> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    };
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    //avoid trailing newline
    buffer = buffer.trim().to_string();
    Ok(buffer)
}
