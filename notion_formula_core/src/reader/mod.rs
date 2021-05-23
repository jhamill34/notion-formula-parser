use std::io::Read;
use pipeline::HandlerResult;

const BUFFER_SIZE: usize = 1000; // 1kb

pub fn read(input: &mut impl Read) -> HandlerResult<Vec<char>> {
    let mut result = vec![];
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    let mut bytes_read = input.read(&mut buffer)?;
    while bytes_read > 0 {
        for i in 0..bytes_read {
            result.push(buffer[i]);
        }
        bytes_read = input.read(&mut buffer)?;
    }

    let result_string: String = String::from_utf8(result)?;
    Ok(result_string.chars().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gets_input() {
        let mut input = "1+2".as_bytes();
        let result = read(&mut input).unwrap();

        assert_eq!(
            vec!['1','+','2'],
            result
        )
    }
}
