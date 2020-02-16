fn get_string_from_raw(data: &[u8]) -> String {
    let vectorized_data = Vec::from(data);
    String::from_utf8(vectorized_data).unwrap()
}
