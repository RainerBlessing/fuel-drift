use std::collections::VecDeque;

/// A single segment of the cave with ceiling and floor heights.
///
/// Each segment represents a vertical slice of the cave tunnel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CaveSegment {
    pub ceiling: f32,
    pub floor: f32,
    pub x_start: f32,
    pub width: f32,
}

impl CaveSegment {
    /// Creates a new cave segment.
    pub fn new(ceiling: f32, floor: f32, x_start: f32, width: f32) -> Self {
        Self {
            ceiling,
            floor,
            x_start,
            width,
        }
    }

    /// Gets the end x-coordinate of this segment.
    pub fn x_end(&self) -> f32 {
        self.x_start + self.width
    }

    /// Gets the gap height between floor and ceiling.
    pub fn gap_height(&self) -> f32 {
        self.floor - self.ceiling
    }
}

/// Cave generation constants.
pub struct CaveConstants;

impl CaveConstants {
    pub const MIN_GAP: f32 = 140.0; // Minimum gap between ceiling and floor
    pub const SEGMENT_WIDTH: f32 = 50.0; // Width of each segment
    pub const MAX_HEIGHT_CHANGE: f32 = 20.0; // Maximum height change per segment
    pub const INITIAL_CEILING: f32 = 100.0;
    pub const INITIAL_FLOOR: f32 = 400.0;
}

/// Simple linear congruential generator for deterministic randomness.
///
/// Provides consistent cave generation for testing and reproducibility.
#[derive(Debug, Clone, Copy)]
pub struct SimpleRng {
    seed: u32,
}

impl SimpleRng {
    /// Creates a new RNG with the given seed.
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    /// Generates the next random number in range [0, 1).
    pub fn next_f32(&mut self) -> f32 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        (self.seed as f32) / (u32::MAX as f32)
    }

    /// Generates a random float in the given range.
    pub fn range(&mut self, min: f32, max: f32) -> f32 {
        min + self.next_f32() * (max - min)
    }
}

/// Procedural cave generator maintaining an endless tunnel.
///
/// Manages cave segments and ensures consistent generation rules.
#[derive(Debug)]
pub struct Cave {
    segments: VecDeque<CaveSegment>,
    rng: SimpleRng,
    next_x: f32,
}

impl Cave {
    /// Creates a new cave with the given seed.
    pub fn new(seed: u32) -> Self {
        let mut cave = Self {
            segments: VecDeque::new(),
            rng: SimpleRng::new(seed),
            next_x: 0.0,
        };

        // Generate initial segment
        cave.generate_initial_segment();
        cave
    }

    /// Generates the initial cave segment.
    fn generate_initial_segment(&mut self) {
        let segment = CaveSegment::new(
            CaveConstants::INITIAL_CEILING,
            CaveConstants::INITIAL_FLOOR,
            0.0,
            CaveConstants::SEGMENT_WIDTH,
        );

        self.segments.push_back(segment);
        self.next_x = segment.x_end();
    }

    /// Generates the next cave segment with random variation.
    ///
    /// Ensures minimum gap is maintained and segments are contiguous.
    pub fn generate_next(&mut self) {
        let prev_segment = self
            .segments
            .back()
            .expect("Cave should always have at least one segment");

        let ceiling_change = self.rng.range(
            -CaveConstants::MAX_HEIGHT_CHANGE,
            CaveConstants::MAX_HEIGHT_CHANGE,
        );
        let floor_change = self.rng.range(
            -CaveConstants::MAX_HEIGHT_CHANGE,
            CaveConstants::MAX_HEIGHT_CHANGE,
        );

        let mut new_ceiling = prev_segment.ceiling + ceiling_change;
        let mut new_floor = prev_segment.floor + floor_change;

        // Ensure minimum gap is maintained
        if new_floor - new_ceiling < CaveConstants::MIN_GAP {
            let gap_center = (new_ceiling + new_floor) / 2.0;
            new_ceiling = gap_center - CaveConstants::MIN_GAP / 2.0;
            new_floor = gap_center + CaveConstants::MIN_GAP / 2.0;
        }

        let segment = CaveSegment::new(
            new_ceiling,
            new_floor,
            self.next_x,
            CaveConstants::SEGMENT_WIDTH,
        );

        self.segments.push_back(segment);
        self.next_x = segment.x_end();

        // Remove old segments to prevent infinite memory growth
        const MAX_SEGMENTS: usize = 100;
        while self.segments.len() > MAX_SEGMENTS {
            self.segments.pop_front();
        }
    }

    /// Returns segments visible in the given x range.
    ///
    /// Generates new segments as needed to fill the view.
    pub fn segments_in_view(&mut self, x_min: f32, x_max: f32) -> Vec<CaveSegment> {
        // Generate segments until we cover the view range
        while self.next_x < x_max + CaveConstants::SEGMENT_WIDTH {
            self.generate_next();
        }

        // Return segments that intersect with the view range
        self.segments
            .iter()
            .filter(|segment| segment.x_start < x_max && segment.x_end() > x_min)
            .copied()
            .collect()
    }

    /// Gets all current segments (for testing).
    pub fn segments(&self) -> &VecDeque<CaveSegment> {
        &self.segments
    }
}
