use nom::{named, tag};

named!(parse_magic_number, tag!(&[0x4e, 0x45, 0x53, 0x1a]));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_number() {
        let data = &[b'N', b'E', b'S', 0x1a];
        let (_, result) = parse_magic_number(data).unwrap();
        assert_eq!(result, data);
    }
}
