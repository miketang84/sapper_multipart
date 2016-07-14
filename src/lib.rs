


fn read_until_token(buffer: &[u8], token: &[u8], mut out: &mut Vec<u8>) -> Result<(usize, bool)>
{

    let mut found: bool = false;
    let mut used: usize = 0;

    if buffer.len() == 0 {
        return Ok((used, found));
    }

    // Get the index index of the first token in the middle of the buffer, if any
    let index = buffer
        .windows(token.len())
        .enumerate()
        .filter(|&(_, t)| t == token)
        .map(|(i, _)| i)
        .next();

    if let Some(index) = index {
        out.extend_from_slice(&buffer[..index]);
        found = true;
        used = index + token.len();
    }

    return Ok((if found { used - token.len() } else { used }, found));
}





#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
