/********************************************************************************
 * 
 * Translate Unicode chat text into Tibia version 1.03's single-byte character encoding.
 * Handles German umlauts (äöüÄÖÜ), ß, and Scandinavian characters.
 * 
 ********************************************************************************/
pub fn translate(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| match c {
            'Ä' => 0x00C4,
            'Å' => 0x00C5,
            'Æ' => 0x00C6,
            'Ö' => 0x00D6,
            'Ø' => 0x00D8,
            'Ü' => 0x00DC,
            'ß' => 0x00DF,
            'ä' => 0x00E4,
            'å' => 0x00E5,
            'æ' => 0x00E6,
            'ö' => 0x00F6,
            'ø' => 0x00F8,
            'ü' => 0x00FC,
            c => c as u8,
        })
        .collect()
}

pub fn translate_upper(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| match c {
            'Ä' | 'ä' => 0x00C4,
            'Å' | 'å' => 0x00C5,
            'Ö' | 'ö' => 0x00D6,
            'Ü' | 'ü' => 0x00DC,
            'ß' => 0x00DF,
            'æ' | 'Æ' => 0x00E6,
            'ø' | 'Ø' => 0x00F8,
            c => c as u8,
        }).collect()
}