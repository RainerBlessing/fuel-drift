use std::collections::VecDeque;
use crate::pickup::PickupManager;

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
    pub const MIN_GAP: f32 = 150.0; // Minimum gap between ceiling and floor
    pub const SEGMENT_WIDTH: f32 = 50.0; // Width of each segment
    pub const MAX_HEIGHT_CHANGE: f32 = 5.0; // Maximum height change per segment (reduced for more horizontal cave)
    pub const INITIAL_CEILING: f32 = 50.0; // High ceiling for level 1
    pub const INITIAL_FLOOR: f32 = 450.0; // Low floor for level 1
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
    pickup_manager: PickupManager,
    base_ceiling: f32,
    base_floor: f32,
}

impl Cave {
    /// Creates a new cave with the given seed.
    pub fn new(seed: u32) -> Self {
        let mut cave = Self {
            segments: VecDeque::new(),
            rng: SimpleRng::new(seed),
            next_x: 0.0,
            pickup_manager: PickupManager::new(seed),
            base_ceiling: CaveConstants::INITIAL_CEILING,
            base_floor: CaveConstants::INITIAL_FLOOR,
        };

        // Generate initial segment
        cave.generate_initial_segment();
        cave
    }

    /// Updates the cave configuration for a new level.
    /// Level 1 has the highest cave (400px gap), each level reduces by 50px.
    pub fn configure_for_level(&mut self, level_number: u32) {
        // Level 1: 400px, Level 2: 350px, ..., Level 6+: MIN_GAP (140px)
        let initial_gap = CaveConstants::INITIAL_FLOOR - CaveConstants::INITIAL_CEILING;
        let level_reduction = (level_number - 1).min(5) as f32 * 50.0;
        let gap = (initial_gap - level_reduction).max(CaveConstants::MIN_GAP);
        
        // Center the cave vertically
        let center_y = 300.0; // Center of 600px high window
        self.base_ceiling = center_y - gap / 2.0;
        self.base_floor = center_y + gap / 2.0;
        
        // Clear existing segments and pickups, then regenerate initial segment
        self.segments.clear();
        self.pickup_manager.clear_all_pickups();
        self.next_x = 0.0;
        self.generate_initial_segment();
    }

    /// Generates the initial cave segment.
    fn generate_initial_segment(&mut self) {
        let segment = CaveSegment::new(
            self.base_ceiling,
            self.base_floor,
            0.0,
            CaveConstants::SEGMENT_WIDTH,
        );

        self.segments.push_back(segment);
        self.next_x = segment.x_end();
    }

    /// Generates the next cave segment with random variation.
    ///
    /// Ensures minimum gap is maintained and segments are contiguous.
    pub fn generate_next(&mut self, fuel_spawn_distance: f32) {
        let _prev_segment = self
            .segments
            .back()
            .expect("Cave should always have at least one segment");

        // Small variation around the base heights
        let ceiling_variation = self.rng.range(
            -CaveConstants::MAX_HEIGHT_CHANGE,
            CaveConstants::MAX_HEIGHT_CHANGE,
        );
        let floor_variation = self.rng.range(
            -CaveConstants::MAX_HEIGHT_CHANGE,
            CaveConstants::MAX_HEIGHT_CHANGE,
        );

        let mut new_ceiling = self.base_ceiling + ceiling_variation;
        let mut new_floor = self.base_floor + floor_variation;

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

        // Check if we should spawn a pickup in this segment
        if self.pickup_manager.should_spawn_pickup(segment.x_start + segment.width / 2.0, fuel_spawn_distance) {
            self.pickup_manager.spawn_fuel_pickup(
                segment.x_start + segment.width / 2.0,
                segment.ceiling,
                segment.floor,
            );
        }

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
    pub fn segments_in_view(&mut self, x_min: f32, x_max: f32, fuel_spawn_distance: f32) -> Vec<CaveSegment> {
        // Generate segments until we cover the view range
        while self.next_x < x_max + CaveConstants::SEGMENT_WIDTH {
            self.generate_next(fuel_spawn_distance);
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

    /// Gets a reference to the pickup manager.
    pub fn pickup_manager(&self) -> &PickupManager {
        &self.pickup_manager
    }

    /// Gets a mutable reference to the pickup manager.
    pub fn pickup_manager_mut(&mut self) -> &mut PickupManager {
        &mut self.pickup_manager
    }
}
