pub(crate) fn byte_array_to_rust_string(array: &[u8]) -> String {
    array.iter().position(|&x| x == 0).map_or_else(
        || String::from_utf8_lossy(array).to_string(),
        |pos| String::from_utf8_lossy(&array[..pos]).to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_array_to_rust_string_with_multiple_zero_bytes_at_end() {
        let input = vec![104, 101, 108, 108, 111, 0, 0, 0]; // "hello" with three zero bytes at the end
        let expected = "hello".to_string();
        assert_eq!(byte_array_to_rust_string(&input), expected);
    }
}
