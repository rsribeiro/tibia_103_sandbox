use std::fs;

fn read_u16(data: &[u8], pos: usize) -> u16 {
    u16::from_le_bytes([data[pos], data[pos + 1]])
}

fn main() {
    let data = fs::read("MUDOBJ.CLI").unwrap();

    for offset in (0..16).step_by(2) {
        println!("\n-------------------- OFFSET {} --------------------", offset);

        let mut pos = offset;
        let mut found = 0;

        while pos + 8 <= data.len() {
            let id = read_u16(&data, pos);
            let flags = read_u16(&data, pos + 2);
            let extra = read_u16(&data, pos + 4);
            let sprite = read_u16(&data, pos + 6);

            if sprite <= 281 {
                println!("id=0x{:04X} flags=0x{:04X} extra=0x{:04X} sprite={}", id, flags, extra, sprite);
                found += 1;
            }

            pos += 8;
        }

        println!("Found {} objects.", found);
    }

    println!("To find the ID of an item, look at its sprite image file name. For example 'a bag' is sprite 45. Now look at its hexadecimal value in the output from this script, which is is 0x013D. That's the item ID of a bag.");
}