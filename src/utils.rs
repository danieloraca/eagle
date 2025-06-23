use crate::simple_random::SimpleRng;

pub fn blend_color(r: u8, g: u8, b: u8, alpha: f32) -> u32 {
    let r: u32 = (r as f32 * alpha) as u32;
    let g: u32 = (g as f32 * alpha) as u32;
    let b: u32 = (b as f32 * alpha) as u32;
    (r << 16) | (g << 8) | b
}

pub fn distance_squared(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (x2 - x1).powi(2) + (y2 - y1).powi(2)
}

pub fn generate_big_star_color() -> u32 {
    let mut rng = SimpleRng::new();
    let r = rng.random_u8() % 255; // Red component
    let g = rng.random_u8() % 255; // Green component
    let b = rng.random_u8() % 255; // Blue component
    let alpha = rng.random_range_f32(0.5..1.0); // Alpha between 0.5 and 1.0
    blend_color(r, g, b, alpha) // Blend with alpha 0.5 for a softer color
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blend_color_full_alpha() {
        let color = blend_color(255, 128, 64, 1.0);
        assert_eq!(color, (255 << 16) | (128 << 8) | 64);
    }

    #[test]
    fn test_blend_color_half_alpha() {
        let color = blend_color(200, 100, 50, 0.5);
        assert_eq!(color, ((100 as u32) << 16) | ((50 as u32) << 8) | 25);
    }

    #[test]
    fn test_blend_color_zero_alpha() {
        let color = blend_color(255, 255, 255, 0.0);
        assert_eq!(color, 0);
    }

    #[test]
    fn test_blend_color_rounding() {
        let color = blend_color(51, 102, 153, 0.5);
        assert_eq!(color, (25 << 16) | (51 << 8) | 76); // floor rounding
    }

    #[test]
    fn test_distance_squared() {
        // Zero distance
        assert_eq!(distance_squared(0.0, 0.0, 0.0, 0.0), 0.0);

        // Horizontal distance
        assert_eq!(distance_squared(0.0, 0.0, 3.0, 0.0), 9.0);

        // Vertical distance
        assert_eq!(distance_squared(0.0, 0.0, 0.0, 4.0), 16.0);

        // Pythagorean triple
        assert_eq!(distance_squared(0.0, 0.0, 3.0, 4.0), 25.0);

        // Negative coordinates
        assert_eq!(distance_squared(-1.0, -1.0, 2.0, 3.0), 25.0);
    }

    #[test]
    fn test_generate_big_star_color_valid_range() {
        for _ in 0..100 {
            let color = generate_big_star_color();
            // Color is ARGB packed into a u32; we expect RGB components only, so max 0xFFFFFF
            assert!(color <= 0xFFFFFF);
        }
    }

    #[test]
    fn test_generate_big_star_color_variability() {
        let mut seen = std::collections::HashSet::new();
        for _ in 0..100 {
            seen.insert(generate_big_star_color());
        }
        // Expect at least, say, 50 unique colors in 100 calls
        assert!(seen.len() > 50, "Too little variation in generated colors");
    }
}
