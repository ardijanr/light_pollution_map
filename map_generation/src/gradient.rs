type RGBA = (u8, u8, u8, u8);

pub struct Gradient {
    colors: Vec<(u32, RGBA)>,
}

impl Gradient {
    /// Creates a new empty gradient
    pub fn new(start_color: RGBA, end_color: RGBA) -> Self {
        Gradient {
            colors: vec![(u32::MIN, start_color), (u32::MAX, end_color)],
        }
    }

    /// Adds a value and color
    /// This will insert the value and the color and ensure that they are
    /// always ordered according to their value
    ///
    /// If the value is equal to T::MAX or T::MIN it will be ignored.
    pub fn add_indexed_color(&mut self, value: u32, color: RGBA) {
        self.colors
            .insert(self.find_sorted_position(value) + 1, (value, color));
    }

    /// Returns the index of where it would fit in a sorted array
    /// It returns the index of the smallest member.
    ///
    /// Returns none if the value is less than or equal to min or greater than or equal max.
    fn find_sorted_position(&self, value: u32) -> usize {
        (0..self.colors.len() - 1)
            .find(|&i| self.colors[i].0 <= value && value <= self.colors[i + 1].0)
            .unwrap_or_else(|| -> usize {
                if value <= self.colors[0].0 {
                    0
                } else {
                    self.colors.len() - 1
                }
            })
    }

    /// Returns the color based on how far the value is from the closest color assigned values
    /// Example:
    ///     30%      60%
    ///   |-----V----------|
    ///   A  Value      B
    ///
    /// The color returned is a mix of 60% A and 30% B
    pub fn sample_gradient(&self, value: u32) -> RGBA {
        let index = self.find_sorted_position(value);
        let low = self.colors[index];
        let high = self.colors[index + 1];

        let delta_high_low = high.0 - low.0;
        //Percentage Distance from low
        let p_dist_low = (value - low.0) as f32 / delta_high_low as f32;
        //Percentage Distance from hight
        let p_dist_high = (high.0 - value) as f32 / delta_high_low as f32;

        (
            (low.1 .0 as f32 * p_dist_high + high.1 .0 as f32 * p_dist_low).clamp(0., 255.) as u8,
            (low.1 .1 as f32 * p_dist_high + high.1 .1 as f32 * p_dist_low).clamp(0., 255.) as u8,
            (low.1 .2 as f32 * p_dist_high + high.1 .2 as f32 * p_dist_low).clamp(0., 255.) as u8,
            (low.1 .3 as f32 * p_dist_high + high.1 .3 as f32 * p_dist_low).clamp(0., 255.) as u8,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient() {
        let mut grad = Gradient::new((0, 0, 0, 255), (255, 255, 255, 255));

        grad.add_indexed_color(1, (100, 100, 100, 255));
        grad.add_indexed_color(500, (100, 100, 100, 255));
        grad.add_indexed_color(300, (100, 100, 100, 255));
        grad.add_indexed_color(2, (100, 100, 100, 255));

        println!("This array looks like: {:?}", grad.colors);

        let grad2 = Gradient::new((0, 0, 0, 255), (100, 255, 255, 255));

        let sample1 = grad2.sample_gradient(0);
        let sample2 = grad2.sample_gradient(u32::MAX);
        let sample3 = grad2.sample_gradient(u32::MAX / 2);
        let sample4 = grad2.sample_gradient(u32::MAX / 254);

        println!("Sample 0 returns color: {:?}", sample1);
        println!("Sample MAX returns color: {:?}", sample2);
        println!("Sample center value: {:?}", sample3);
        println!("Sample center value: {:?}", sample4);
    }
}
