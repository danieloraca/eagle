use std::collections::HashMap;

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

pub const LETTERS: [[u8; 7]; 26] = [
    [
        0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
    ], // A
    [
        0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
    ], // B
    [
        0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110,
    ], // C
    [
        0b11100, 0b10010, 0b10001, 0b10001, 0b10001, 0b10010, 0b11100,
    ], // D
    [
        0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
    ], // E
    [
        0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
    ], // F
    [
        0b01110, 0b10001, 0b10000, 0b10000, 0b10011, 0b10001, 0b01110,
    ], // G
    [
        0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
    ], // H
    [
        0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
    ], // I
    [
        0b00111, 0b00010, 0b00010, 0b00010, 0b10010, 0b10010, 0b01100,
    ], // J
    [
        0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
    ], // K
    [
        0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
    ], // L
    [
        0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
    ], // M
    [
        0b10001, 0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001,
    ], // N
    [
        0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
    ], // O
    [
        0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
    ], // P
    [
        0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101,
    ], // Q
    [
        0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
    ], // R
    [
        0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
    ], // S
    [
        0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
    ], // T
    [
        0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
    ], // U
    [
        0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
    ], // V
    [
        0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010,
    ], // W
    [
        0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
    ], // X
    [
        0b10001, 0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100,
    ], // Y
    [
        0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111,
    ], // Z
];

pub fn get_symbols() -> HashMap<char, [u8; 7]> {
    use std::collections::HashMap;
    let mut symbols: HashMap<char, [u8; 7]> = HashMap::new();

    symbols.insert(
        ':',
        [
            0b00000, 0b00100, 0b00000, 0b00000, 0b00100, 0b00000, 0b00000,
        ],
    );
    symbols.insert(
        '(',
        [
            0b00010, 0b00100, 0b01000, 0b01000, 0b01000, 0b00100, 0b00010,
        ],
    );
    symbols.insert(
        ')',
        [
            0b01000, 0b00100, 0b00010, 0b00010, 0b00010, 0b00100, 0b01000,
        ],
    );
    symbols.insert(
        '.',
        [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00100, 0b00000,
        ],
    );
    symbols.insert(
        '-',
        [
            0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000, 0b00000,
        ],
    );
    symbols.insert(
        '+',
        [
            0b00000, 0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000,
        ],
    );
    symbols.insert(
        '/',
        [
            0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b00000, 0b00000,
        ],
    );
    symbols.insert(
        '!',
        [
            0b00100, 0b00100, 0b00100, 0b00100, 0b00000, 0b00100, 0b00000,
        ],
    );
    symbols.insert(
        '?',
        [
            0b01110, 0b10001, 0b00010, 0b00100, 0b00100, 0b00000, 0b00100,
        ],
    );
    symbols.insert(
        ' ',
        [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000,
        ],
    );

    symbols
}

fn draw_digit(
    buffer: &mut [u32],
    width: usize,
    x: usize,
    y: usize,
    digit: usize,
    color: u32,
    scale: usize,
) {
    let font: [u8; 5] = FONT[digit];
    for (dy, row) in font.iter().enumerate() {
        for dx in 0..5 {
            if (row >> (4 - dx)) & 1 == 1 {
                // Draw a scale x scale block
                for sy in 0..scale {
                    for sx in 0..scale {
                        let px: usize = x + dx * scale + sx;
                        let py: usize = y + dy * scale + sy;
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

fn draw_letter(
    buffer: &mut [u32],
    width: usize,
    x: usize,
    y: usize,
    ch: char,
    color: u32,
    scale: usize,
) {
    if ch.is_ascii_alphabetic() {
        let index: usize = (ch.to_ascii_uppercase() as u8 - b'A') as usize;
        if index < 26 {
            let font: [u8; 7] = LETTERS[index];
            for (dy, row) in font.iter().enumerate() {
                for dx in 0..5 {
                    if (row >> (4 - dx)) & 1 == 1 {
                        for sy in 0..scale {
                            for sx in 0..scale {
                                let px: usize = x + dx * scale + sx;
                                let py: usize = y + dy * scale + sy;
                                if px < width && py < buffer.len() / width {
                                    buffer[py * width + px] = color;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn draw_text(
    buffer: &mut [u32],
    width: usize,
    x: usize,
    y: usize,
    text: &str,
    color: u32,
    scale: usize,
) {
    let symbols: HashMap<char, [u8; 7]> = get_symbols();

    for (i, ch) in text.chars().enumerate() {
        let px: usize = x + i * (5 * scale + scale);

        if ch.is_ascii_digit() {
            draw_digit(
                buffer,
                width,
                px,
                y,
                ch.to_digit(10).unwrap() as usize,
                color,
                scale,
            );
        } else if ch.is_ascii_alphabetic() {
            draw_letter(buffer, width, px, y, ch, color, scale);
        } else if let Some(pattern) = symbols.get(&ch) {
            draw_custom_glyph(buffer, width, px, y, pattern, color, scale);
        }
    }
}

fn draw_custom_glyph(
    buffer: &mut [u32],
    width: usize,
    x: usize,
    y: usize,
    pattern: &[u8; 7],
    color: u32,
    scale: usize,
) {
    for (dy, row) in pattern.iter().enumerate() {
        for dx in 0..5 {
            if (row >> (4 - dx)) & 1 == 1 {
                for sy in 0..scale {
                    for sx in 0..scale {
                        let px: usize = x + dx * scale + sx;
                        let py: usize = y + dy * scale + sy;
                        if px < width && py < buffer.len() / width {
                            buffer[py * width + px] = color;
                        }
                    }
                }
            }
        }
    }
}
