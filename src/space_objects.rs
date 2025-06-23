pub struct Star {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct BigStar {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub hit: bool,
    pub was_missed: bool,
}

impl BigStar {
    pub fn is_off_screen(&self, width: usize, height: usize) -> bool {
        // Project 3D coordinates to 2D screen space
        let sx: f32 = self.x / self.z * width as f32 / 2.0 + width as f32 / 2.0;
        let sy: f32 = self.y / self.z * height as f32 / 2.0 + height as f32 / 2.0;
        // Check if projected coordinates are outside screen bounds
        sx < 0.0 || sx > width as f32 || sy < 0.0 || sy > height as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_star_center_on_screen() {
        let width = 800;
        let height = 600;
        let star = BigStar {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            hit: false,
            was_missed: false,
        };
        assert_eq!(star.is_off_screen(width, height), false);
    }

    #[test]
    fn test_star_far_left_off_screen() {
        let width = 800;
        let height = 600;
        let star = BigStar {
            x: -20.0,
            y: 0.0,
            z: 10.0,
            hit: false,
            was_missed: false,
        };
        assert_eq!(star.is_off_screen(width, height), true);
    }

    #[test]
    fn test_star_far_right_off_screen() {
        let width = 800;
        let height = 600;
        let star = BigStar {
            x: 10.0,
            y: 0.0,
            z: 1.0,
            hit: false,
            was_missed: false,
        };
        assert_eq!(star.is_off_screen(width, height), true);
    }

    #[test]
    fn test_star_below_screen() {
        let width = 800;
        let height = 600;
        let star = BigStar {
            x: 0.0,
            y: 10.0,
            z: 1.0,
            hit: false,
            was_missed: false,
        };
        assert_eq!(star.is_off_screen(width, height), true);
    }
}