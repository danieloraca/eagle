pub const FONT: [[u8; 5]; 10] = [
    [0b01110, 0b10001, 0b10011, 0b10101, 0b01110], // 0
    [0b00100, 0b01100, 0b00100, 0b00100, 0b01110], // 1
    [0b01110, 0b10001, 0b00010, 0b00100, 0b11111], // 2
    [0b11110, 0b00001, 0b00110, 0b00001, 0b11110], // 3
    [0b00010, 0b00110, 0b01010, 0b11111, 0b00010], // 4
    [0b11111, 0b10000, 0b11110, 0b00001, 0b11110], // 5
    [0b01110, 0b10000, 0b11110, 0b10001, 0b01110], // 6
    [0b11111, 0b00001, 0b00010, 0b00100, 0b01000], // 7
    [0b01110, 0b10001, 0b01110, 0b10001, 0b01110], // 8
    [0b01110, 0b10001, 0b01111, 0b00001, 0b01110], // 9
];

fn draw_digit(
    buffer: &mut [u32],
    width: usize,
    x: usize,
    y: usize,
    digit: usize,
    color: u32,
    scale: usize,
) {
    let font = FONT[digit];
    for (dy, row) in font.iter().enumerate() {
        for dx in 0..5 {
            if (row >> (4 - dx)) & 1 == 1 {
                // Draw a scale x scale block
                for sy in 0..scale {
                    for sx in 0..scale {
                        let px = x + dx * scale + sx;
                        let py = y + dy * scale + sy;
                        if px < width && py < buffer.len() / width {
                            buffer[py * width + px] = color;
                        }
                    }
                }
            }
        }
    }
}

pub fn draw_number(
    buffer: &mut [u32],
    width: usize,
    x: usize,
    y: usize,
    number: usize,
    color: u32,
    scale: usize,
) {
    let digits: Vec<_> = number.to_string().chars().collect();
    for (i, d) in digits.iter().enumerate() {
        if let Some(digit) = d.to_digit(10) {
            draw_digit(
                buffer,
                width,
                x + i * (5 * scale + scale),
                y,
                digit as usize,
                color,
                scale,
            );
        }
    }
}
