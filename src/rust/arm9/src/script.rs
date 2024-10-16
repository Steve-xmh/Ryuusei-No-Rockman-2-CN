pub fn decode_script(script_data: &[u8]) -> (u16, bool) {
    let first_code = script_data[0] as u16;
    let second_code = script_data[1] as u16;
    match first_code {
        0x00..=0xCF => (first_code, false),
        0xD0..=0xE3 => ((first_code - 0xD0) * 0xE4 + second_code + 0xD0, true),
        0xE4 => (first_code + second_code, true),
        0xE9 => (u16::MAX - 1, false), // 换行符
        _ => (u16::MAX, false),
    }
}
