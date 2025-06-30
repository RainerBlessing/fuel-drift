/// Pickup system for collectible items in the game.
///
/// Manages fuel depots and other collectibles that spawn on cave walls.

use crate::cave::SimpleRng;
use crate::constants::{PickupConstants, TractorBeamConstants};
use crate::tractor::TractorBeam;

/// Types of pickups available in the game.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PickupType {
    /// Fuel depot that refills the player's fuel tank
    Fuel,
}

/// A collectible item positioned on cave walls.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pickup {
    /// Position in world coordinates (x, y)
    pub position: (f32, f32),
    /// Original position when spawned (for beam attraction)
    pub original_position: (f32, f32),
    /// Type of pickup
    pub pickup_type: PickupType,
    /// Whether the pickup is attached to the ceiling (true) or floor (false)
    pub is_on_ceiling: bool,
    /// Whether this pickup has been collected
    pub collected: bool,
    /// Whether this pickup is currently being attracted by a tractor beam
    pub being_attracted: bool,
}

impl Pickup {
    /// Creates a new pickup at the specified position.
    pub fn new(position: (f32, f32), pickup_type: PickupType, is_on_ceiling: bool) -> Self {
        Self {
            position,
            original_position: position,
            pickup_type,
            is_on_ceiling,
            collected: false,
            being_attracted: false,
        }
    }

    /// Marks this pickup as collected.
    pub fn collect(&mut self) {
        self.collected = true;
    }

    /// Resets pickup position to original wall position.
    pub fn reset_to_wall(&mut self) {
        self.position = self.original_position;
        self.being_attracted = false;
    }

    /// Updates pickup position during tractor beam attraction.
    ///
    /// # Arguments
    /// * `force` - Normalized attraction force vector
    /// * `speed` - Attraction speed multiplier
    /// * `dt` - Delta time for frame-rate independent movement
    pub fn apply_attraction(&mut self, force: (f32, f32), speed: f32, dt: f32) {
        if !self.collected {
            self.position.0 += force.0 * speed * dt;
            self.position.1 += force.1 * speed * dt;
            self.being_attracted = true;
        }
    }
}

/// Manages spawning and tracking of pickups.
#[derive(Debug)]
pub struct PickupManager {
    /// Active pickups in the world
    pickups: Vec<Pickup>,
    /// Random number generator for spawn decisions
    rng: SimpleRng,
    /// X-coordinate where the last pickup was spawned
    last_pickup_x: f32,
    /// Distance to next pickup spawn
    next_pickup_distance: f32,
}

impl PickupManager {
    /// Creates a new pickup manager.
    pub fn new(seed: u32) -> Self {
        Self {
            pickups: Vec::new(),
            rng: SimpleRng::new(seed.wrapping_add(PickupConstants::RNG_SEED_OFFSET)),
            last_pickup_x: -PickupConstants::INITIAL_SPAWN_DELAY,
            next_pickup_distance: 0.0,
        }
    }

    /// Checks if a pickup should spawn at the given x-coordinate.
    ///
    /// # Arguments
    /// * `x` - Current x-coordinate to check
    /// * `average_distance` - Average distance between pickups from level config
    pub fn should_spawn_pickup(&mut self, x: f32, average_distance: f32) -> bool {
        if x >= self.last_pickup_x + self.next_pickup_distance {
            // Calculate next spawn distance with random variation
            let variation = average_distance * PickupConstants::SPAWN_DISTANCE_VARIATION;
            self.next_pickup_distance = self.rng.range(
                average_distance - variation,
                average_distance + variation,
            );
            self.last_pickup_x = x;
            true
        } else {
            false
        }
    }

    /// Spawns a new fuel pickup at the specified position.
    ///
    /// # Arguments
    /// * `x` - X-coordinate for the pickup
    /// * `ceiling_y` - Y-coordinate of the ceiling at this x position
    /// * `floor_y` - Y-coordinate of the floor at this x position
    pub fn spawn_fuel_pickup(&mut self, x: f32, ceiling_y: f32, floor_y: f32) {
        // Randomly choose ceiling or floor
        let is_on_ceiling = self.rng.next_f32() < 0.5;
        
        // Position pickup on the wall with a small offset
        let y = if is_on_ceiling {
            ceiling_y + PickupConstants::WALL_OFFSET
        } else {
            floor_y - PickupConstants::SIZE - PickupConstants::WALL_OFFSET
        };
        
        let pickup = Pickup::new((x, y), PickupType::Fuel, is_on_ceiling);
        self.pickups.push(pickup);
    }

    /// Gets all active (non-collected) pickups in the specified x-range.
    ///
    /// # Arguments
    /// * `x_min` - Minimum x-coordinate
    /// * `x_max` - Maximum x-coordinate
    pub fn get_pickups_in_range(&self, x_min: f32, x_max: f32) -> Vec<&Pickup> {
        self.pickups
            .iter()
            .filter(|p| !p.collected && p.position.0 >= x_min && p.position.0 <= x_max)
            .collect()
    }

    /// Checks for collision between player and pickups.
    ///
    /// Returns the index of the first pickup that collides with the player.
    ///
    /// # Arguments
    /// * `player_pos` - Player position (x, y)
    /// * `player_size` - Player size (width, height)
    pub fn check_collision(&self, player_pos: (f32, f32), player_size: (f32, f32)) -> Option<usize> {
        let pickup_size = (PickupConstants::SIZE, PickupConstants::SIZE);
        
        for (index, pickup) in self.pickups.iter().enumerate() {
            if pickup.collected {
                continue;
            }
            
            // Simple AABB collision
            let pickup_right = pickup.position.0 + pickup_size.0;
            let pickup_bottom = pickup.position.1 + pickup_size.1;
            let player_right = player_pos.0 + player_size.0;
            let player_bottom = player_pos.1 + player_size.1;
            
            if pickup.position.0 < player_right
                && pickup_right > player_pos.0
                && pickup.position.1 < player_bottom
                && pickup_bottom > player_pos.1
            {
                return Some(index);
            }
        }
        
        None
    }

    /// Collects a pickup at the specified index.
    ///
    /// # Arguments
    /// * `index` - Index of the pickup to collect
    pub fn collect_pickup(&mut self, index: usize) -> Option<PickupType> {
        if let Some(pickup) = self.pickups.get_mut(index) {
            if !pickup.collected {
                pickup.collect();
                return Some(pickup.pickup_type);
            }
        }
        None
    }

    /// Removes collected pickups that are far behind the camera.
    ///
    /// # Arguments
    /// * `camera_x` - Current camera x-position
    pub fn cleanup_old_pickups(&mut self, camera_x: f32) {
        const CLEANUP_DISTANCE: f32 = 1000.0;
        self.pickups.retain(|p| {
            !p.collected || p.position.0 > camera_x - CLEANUP_DISTANCE
        });
    }

    /// Gets the total number of active pickups (for debugging).
    pub fn active_pickup_count(&self) -> usize {
        self.pickups.iter().filter(|p| !p.collected).count()
    }

    /// Updates pickups affected by tractor beam attraction.
    ///
    /// Uses hysteresis: narrow beam for activation, wider area for maintaining attraction.
    ///
    /// # Arguments
    /// * `tractor_beam` - The tractor beam system
    /// * `player_pos` - Current player position
    /// * `dt` - Delta time for frame-rate independent movement
    pub fn update_tractor_beam_attraction(
        &mut self,
        tractor_beam: &TractorBeam,
        player_pos: (f32, f32),
        dt: f32,
    ) {
        for pickup in &mut self.pickups {
            if pickup.collected {
                continue;
            }

            if tractor_beam.is_active() {
                Self::update_pickup_attraction(pickup, tractor_beam, player_pos, dt);
            } else if pickup.being_attracted {
                pickup.reset_to_wall();
            }
        }
    }

    /// Updates attraction for a single pickup based on its current state.
    fn update_pickup_attraction(
        pickup: &mut Pickup,
        tractor_beam: &TractorBeam,
        player_pos: (f32, f32),
        dt: f32,
    ) {
        if pickup.being_attracted {
            Self::handle_ongoing_attraction(pickup, tractor_beam, player_pos, dt);
        } else {
            Self::handle_initial_attraction(pickup, tractor_beam, player_pos, dt);
        }
    }

    /// Handles attraction for pickups that are already being attracted.
    fn handle_ongoing_attraction(
        pickup: &mut Pickup,
        tractor_beam: &TractorBeam,
        player_pos: (f32, f32),
        dt: f32,
    ) {
        if tractor_beam.should_maintain_attraction(player_pos, pickup.position) {
            Self::apply_attraction_force(pickup, tractor_beam, player_pos, dt);
        } else {
            pickup.reset_to_wall();
        }
    }

    /// Handles initial attraction activation for new pickups.
    fn handle_initial_attraction(
        pickup: &mut Pickup,
        tractor_beam: &TractorBeam,
        player_pos: (f32, f32),
        dt: f32,
    ) {
        if tractor_beam.is_point_in_beam(player_pos, pickup.position) {
            Self::apply_attraction_force(pickup, tractor_beam, player_pos, dt);
        }
    }

    /// Applies attraction force to a pickup if force is non-zero.
    fn apply_attraction_force(
        pickup: &mut Pickup,
        tractor_beam: &TractorBeam,
        player_pos: (f32, f32),
        dt: f32,
    ) {
        let force = tractor_beam.get_attraction_force(player_pos, pickup.position);
        if force != (0.0, 0.0) {
            pickup.apply_attraction(force, TractorBeamConstants::ATTRACTION_SPEED, dt);
        }
    }

    /// Gets all pickups currently being attracted by the tractor beam.
    pub fn get_attracted_pickups(&self) -> Vec<&Pickup> {
        self.pickups
            .iter()
            .filter(|p| !p.collected && p.being_attracted)
            .collect()
    }

    /// Checks if any pickup is within tractor beam range.
    ///
    /// # Arguments
    /// * `tractor_beam` - The tractor beam system
    /// * `player_pos` - Current player position
    pub fn has_pickups_in_beam_range(&self, tractor_beam: &TractorBeam, player_pos: (f32, f32)) -> bool {
        if !tractor_beam.is_active() {
            return false;
        }

        self.pickups
            .iter()
            .filter(|p| !p.collected)
            .any(|p| tractor_beam.is_point_in_beam(player_pos, p.position))
    }
    
    /// Clears all pickups (used when transitioning to a new level).
    pub fn clear_all_pickups(&mut self) {
        self.pickups.clear();
        self.last_pickup_x = 0.0;
        self.next_pickup_distance = PickupConstants::INITIAL_SPAWN_DELAY;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pickup_creation() {
        let pickup = Pickup::new((100.0, 50.0), PickupType::Fuel, true);
        assert_eq!(pickup.position, (100.0, 50.0));
        assert_eq!(pickup.pickup_type, PickupType::Fuel);
        assert!(pickup.is_on_ceiling);
        assert!(!pickup.collected);
    }

    #[test]
    fn test_pickup_collection() {
        let mut pickup = Pickup::new((0.0, 0.0), PickupType::Fuel, false);
        assert!(!pickup.collected);
        pickup.collect();
        assert!(pickup.collected);
    }

    #[test]
    fn test_spawn_distance_variation() {
        let mut manager = PickupManager::new(12345);
        let average_distance = 300.0;
        
        // First spawn should happen immediately
        assert!(manager.should_spawn_pickup(0.0, average_distance));
        
        // Next spawn should respect distance
        assert!(!manager.should_spawn_pickup(100.0, average_distance));
        
        // Should spawn when distance is exceeded
        let mut x = 0.0;
        let mut spawn_count = 0;
        while spawn_count < 5 {
            x += 50.0;
            if manager.should_spawn_pickup(x, average_distance) {
                spawn_count += 1;
                // Check that distance is within expected range (±30%)
                assert!(manager.next_pickup_distance >= average_distance * 0.7);
                assert!(manager.next_pickup_distance <= average_distance * 1.3);
            }
        }
    }

    #[test]
    fn test_collision_detection() {
        let mut manager = PickupManager::new(0);
        manager.spawn_fuel_pickup(100.0, 50.0, 300.0);
        
        let player_size = (30.0, 30.0);
        
        // No collision when player is far away
        assert_eq!(manager.check_collision((0.0, 0.0), player_size), None);
        
        // Collision when player overlaps with pickup
        let pickup_pos = manager.pickups[0].position;
        assert_eq!(
            manager.check_collision((pickup_pos.0 - 10.0, pickup_pos.1), player_size),
            Some(0)
        );
    }

    #[test]
    fn test_collected_pickups_ignored() {
        let mut manager = PickupManager::new(0);
        manager.spawn_fuel_pickup(100.0, 50.0, 300.0);
        
        // Collect the pickup
        manager.collect_pickup(0);
        
        // Should not detect collision with collected pickup
        let pickup_pos = manager.pickups[0].position;
        assert_eq!(
            manager.check_collision((pickup_pos.0, pickup_pos.1), (30.0, 30.0)),
            None
        );
    }

    #[test]
    fn test_cleanup_old_pickups() {
        let mut manager = PickupManager::new(0);
        
        // Spawn pickups at different positions
        manager.spawn_fuel_pickup(100.0, 50.0, 300.0);
        manager.spawn_fuel_pickup(500.0, 50.0, 300.0);
        manager.spawn_fuel_pickup(900.0, 50.0, 300.0);
        
        // Collect first pickup
        manager.collect_pickup(0);
        
        assert_eq!(manager.pickups.len(), 3);
        
        // Cleanup with camera at x=1200
        manager.cleanup_old_pickups(1200.0);
        
        // Only the collected pickup at x=100 should be removed
        assert_eq!(manager.pickups.len(), 2);
        assert_eq!(manager.active_pickup_count(), 2);
    }
    
    #[test]
    fn test_integration_level_affects_spawning() {
        use crate::level::Level;
        
        let mut manager = PickupManager::new(123);
        
        // Test with different level configurations
        let level1 = Level::new(1, 60.0, 200.0, 200.0); // Closer spawns
        let level2 = Level::new(2, 60.0, 600.0, 180.0); // Further spawns
        
        // Test spawning with level 1 distance
        let mut spawn_count_1 = 0;
        for x in (0..2000).step_by(50) {
            if manager.should_spawn_pickup(x as f32, level1.fuel_spawn_distance) {
                spawn_count_1 += 1;
            }
        }
        
        // Reset manager for level 2 test
        let mut manager2 = PickupManager::new(123);
        let mut spawn_count_2 = 0;
        for x in (0..2000).step_by(50) {
            if manager2.should_spawn_pickup(x as f32, level2.fuel_spawn_distance) {
                spawn_count_2 += 1;
            }
        }
        
        // Level 1 should spawn more pickups due to closer distance
        assert!(spawn_count_1 > spawn_count_2, 
            "Level 1 (distance {}) should spawn more pickups than Level 2 (distance {})", 
            level1.fuel_spawn_distance, level2.fuel_spawn_distance);
    }
    
    #[test]
    fn test_integration_cave_pickup_workflow() {
        use crate::cave::Cave;
        
        let mut cave = Cave::new(456);
        let fuel_distance = 300.0;
        
        // Generate several cave segments
        for _ in 0..10 {
            cave.generate_next(fuel_distance);
        }
        
        // Check that pickups were spawned
        let pickups = cave.pickup_manager().get_pickups_in_range(0.0, 1000.0);
        assert!(!pickups.is_empty(), "Pickups should be spawned during cave generation");
        
        // Test collision and collection workflow
        if let Some(pickup) = pickups.first() {
            let pickup_pos = pickup.position;
            
            // Check collision detection
            let collision_index = cave.pickup_manager().check_collision(
                (pickup_pos.0, pickup_pos.1),
                (30.0, 18.0) // Player size
            );
            assert!(collision_index.is_some(), "Should detect collision with pickup");
            
            // Collect pickup
            let pickup_type = cave.pickup_manager_mut().collect_pickup(collision_index.unwrap());
            assert_eq!(pickup_type, Some(crate::pickup::PickupType::Fuel));
            
            // Verify pickup is marked as collected
            let _pickups_after = cave.pickup_manager().get_pickups_in_range(0.0, 1000.0);
            let collected_pickup = cave.pickup_manager().pickups.iter()
                .find(|p| p.position == pickup_pos);
            assert!(collected_pickup.unwrap().collected);
        }
    }

    #[test]
    fn test_pickup_attraction_basic() {
        let mut pickup = Pickup::new((100.0, 50.0), PickupType::Fuel, true);
        assert_eq!(pickup.position, (100.0, 50.0));
        assert_eq!(pickup.original_position, (100.0, 50.0));
        assert!(!pickup.being_attracted);
        
        // Apply attraction force
        pickup.apply_attraction((1.0, 0.0), 100.0, 0.1);
        
        // Position should have moved
        assert_eq!(pickup.position, (110.0, 50.0));
        assert!(pickup.being_attracted);
        
        // Reset to wall
        pickup.reset_to_wall();
        assert_eq!(pickup.position, pickup.original_position);
        assert!(!pickup.being_attracted);
    }

    #[test]
    fn test_tractor_beam_pickup_integration() {
        use crate::tractor::{TractorBeam, BeamDir};
        
        let mut manager = PickupManager::new(0);
        let mut beam = TractorBeam::new();
        
        // Manually create a pickup in the correct position (above player for upward beam)
        let pickup = Pickup::new((100.0, 100.0), PickupType::Fuel, true); // Above player
        manager.pickups.push(pickup);
        
        let player_pos = (100.0, 200.0); // Player below pickup
        let dt = 0.016; // ~60 FPS
        
        // Without beam activation, no attraction
        manager.update_tractor_beam_attraction(&beam, player_pos, dt);
        let pickups = manager.get_attracted_pickups();
        assert_eq!(pickups.len(), 0);
        
        // Activate beam upward (toward the pickup)
        beam.activate(BeamDir::Up);
        
        // Update attraction several times
        for _ in 0..10 {
            manager.update_tractor_beam_attraction(&beam, player_pos, dt);
        }
        
        // Pickup should be attracted
        let attracted_pickups = manager.get_attracted_pickups();
        assert_eq!(attracted_pickups.len(), 1);
        
        // Pickup should have moved toward player (y position should increase)
        let pickup = &manager.pickups[0];
        assert!(pickup.position.1 > pickup.original_position.1); // Moved down toward player
    }

    #[test]
    fn test_beam_direction_filtering() {
        use crate::tractor::{TractorBeam, BeamDir};
        
        let mut manager = PickupManager::new(0);
        let mut beam = TractorBeam::new();
        
        // Manually create pickups in correct positions
        let ceiling_pickup = Pickup::new((150.0, 100.0), PickupType::Fuel, true); // Above player
        let floor_pickup = Pickup::new((150.0, 200.0), PickupType::Fuel, false); // Below player
        manager.pickups.push(ceiling_pickup);
        manager.pickups.push(floor_pickup);
        
        let player_pos = (150.0, 150.0); // Between ceiling and floor
        let dt = 0.016;
        
        // Activate beam upward (should only affect ceiling pickup)
        beam.activate(BeamDir::Up);
        
        for _ in 0..5 {
            manager.update_tractor_beam_attraction(&beam, player_pos, dt);
        }
        
        // Check which pickups are being attracted
        let attracted_count = manager.get_attracted_pickups().len();
        assert_eq!(attracted_count, 1); // Only ceiling pickup should be attracted
        
        // The ceiling pickup (index 0) should be attracted
        assert!(manager.pickups[0].being_attracted);
        // The floor pickup (index 1) should not be attracted
        assert!(!manager.pickups[1].being_attracted);
    }

    #[test]
    fn test_beam_range_limits() {
        use crate::tractor::{TractorBeam, BeamDir};
        
        let mut manager = PickupManager::new(0);
        let mut beam = TractorBeam::new();
        
        // Create pickup outside beam width (horizontally too far)
        let pickup_outside_width = Pickup::new((150.0, 150.0), PickupType::Fuel, true); // 50px away horizontally
        manager.pickups.push(pickup_outside_width);
        
        // Create pickup outside beam range (vertically too far)
        let pickup_outside_range = Pickup::new((100.0, -110.0), PickupType::Fuel, true); // 310px away vertically
        manager.pickups.push(pickup_outside_range);
        
        let player_pos = (100.0, 200.0);
        let dt = 0.016;
        
        beam.activate(BeamDir::Up);
        
        for _ in 0..10 {
            manager.update_tractor_beam_attraction(&beam, player_pos, dt);
        }
        
        // Neither pickup should be attracted due to being outside beam area
        assert_eq!(manager.get_attracted_pickups().len(), 0);
        assert!(!manager.pickups[0].being_attracted); // Outside width
        assert!(!manager.pickups[1].being_attracted); // Outside range
    }

    #[test]
    fn test_beam_deactivation_resets_pickups() {
        use crate::tractor::{TractorBeam, BeamDir};
        
        let mut manager = PickupManager::new(0);
        let mut beam = TractorBeam::new();
        
        // Manually create pickup in correct position
        let pickup = Pickup::new((100.0, 150.0), PickupType::Fuel, true); // Above player
        manager.pickups.push(pickup);
        
        let player_pos = (100.0, 200.0);
        let dt = 0.016;
        
        // Activate beam and attract pickup
        beam.activate(BeamDir::Up);
        for _ in 0..5 {
            manager.update_tractor_beam_attraction(&beam, player_pos, dt);
        }
        
        // Verify pickup is being attracted
        assert!(manager.pickups[0].being_attracted);
        let moved_position = manager.pickups[0].position;
        assert_ne!(moved_position, manager.pickups[0].original_position);
        
        // Deactivate beam by letting it expire
        beam.timer = 0.0;
        beam.active = false;
        
        manager.update_tractor_beam_attraction(&beam, player_pos, dt);
        
        // Pickup should be reset to original position
        assert!(!manager.pickups[0].being_attracted);
        assert_eq!(manager.pickups[0].position, manager.pickups[0].original_position);
    }

    #[test]
    fn test_has_pickups_in_beam_range() {
        use crate::tractor::{TractorBeam, BeamDir};
        
        let mut manager = PickupManager::new(0);
        let beam = TractorBeam::new(); // Inactive
        
        // Manually create pickup in correct position
        let pickup = Pickup::new((100.0, 150.0), PickupType::Fuel, true); // Above player
        manager.pickups.push(pickup);
        
        let player_pos = (100.0, 200.0);
        
        // Inactive beam should return false
        assert!(!manager.has_pickups_in_beam_range(&beam, player_pos));
        
        let mut beam = TractorBeam::new();
        beam.activate(BeamDir::Up);
        
        // Active beam with pickup in range should return true
        assert!(manager.has_pickups_in_beam_range(&beam, player_pos));
        
        // Move player far away
        let far_player_pos = (500.0, 200.0);
        assert!(!manager.has_pickups_in_beam_range(&beam, far_player_pos));
    }

    #[test]
    fn test_beam_width_precision() {
        use crate::tractor::{TractorBeam, BeamDir};
        
        let mut manager = PickupManager::new(0);
        let mut beam = TractorBeam::new();
        
        let player_pos = (100.0, 200.0);
        
        // Create pickup exactly at beam edge (should be attracted)
        let pickup_at_edge = Pickup::new((116.0, 150.0), PickupType::Fuel, true); // 16px right (beam half-width)
        manager.pickups.push(pickup_at_edge);
        
        // Create pickup just outside beam edge (should NOT be attracted)
        let pickup_outside = Pickup::new((117.0, 150.0), PickupType::Fuel, true); // 17px right (outside beam)
        manager.pickups.push(pickup_outside);
        
        beam.activate(BeamDir::Up);
        let dt = 0.016;
        
        for _ in 0..5 {
            manager.update_tractor_beam_attraction(&beam, player_pos, dt);
        }
        
        // Only the pickup at beam edge should be attracted
        let attracted_pickups = manager.get_attracted_pickups();
        assert_eq!(attracted_pickups.len(), 1);
        assert!(manager.pickups[0].being_attracted); // At edge - should be attracted
        assert!(!manager.pickups[1].being_attracted); // Outside - should NOT be attracted
    }

    #[test]
    fn test_attraction_hysteresis() {
        use crate::tractor::{TractorBeam, BeamDir};
        
        let mut manager = PickupManager::new(0);
        let mut beam = TractorBeam::new();
        
        let player_pos = (100.0, 200.0);
        
        // Create pickup within initial beam width (32px)
        let pickup = Pickup::new((115.0, 150.0), PickupType::Fuel, true); // 15px right (within 16px half-width)
        manager.pickups.push(pickup);
        
        beam.activate(BeamDir::Up);
        let dt = 0.016;
        
        // First update - should start attraction
        manager.update_tractor_beam_attraction(&beam, player_pos, dt);
        assert!(manager.pickups[0].being_attracted);
        
        // Manually move pickup slightly outside initial beam width but within hold width
        manager.pickups[0].position.0 = 120.0; // 20px right (outside 16px, but within 24px hold half-width)
        
        // Should continue being attracted due to hysteresis
        manager.update_tractor_beam_attraction(&beam, player_pos, dt);
        assert!(manager.pickups[0].being_attracted);
        
        // Move pickup way outside hold width
        manager.pickups[0].position.0 = 140.0; // 40px right (outside 24px hold half-width)
        
        // Should stop being attracted and reset to wall
        manager.update_tractor_beam_attraction(&beam, player_pos, dt);
        assert!(!manager.pickups[0].being_attracted);
        assert_eq!(manager.pickups[0].position, manager.pickups[0].original_position);
    }

    #[test]
    fn test_ceiling_pickup_positioning() {
        let mut manager = PickupManager::new(0);
        
        // Test ceiling pickup positioning
        manager.spawn_fuel_pickup(100.0, 50.0, 300.0); // ceiling_y=50, floor_y=300
        
        // Check if pickup was placed on ceiling
        let pickup = &manager.pickups[0];
        
        // If on ceiling, y should be ceiling_y + WALL_OFFSET = 50 + 5 = 55
        if pickup.is_on_ceiling {
            let expected_y = 50.0 + PickupConstants::WALL_OFFSET;
            assert_eq!(pickup.position.1, expected_y);
        }
    }

    #[test]
    fn test_beam_direction_coordinates() {
        use crate::tractor::{TractorBeam, BeamDir};
        
        let mut beam = TractorBeam::new();
        beam.activate(BeamDir::Up);
        
        let player_pos = (100.0, 200.0); // Player at y=200
        
        // Test pickup above player (ceiling) - y=55 (above player)
        let ceiling_pickup_pos = (100.0, 55.0);
        let ceiling_in_beam = beam.is_point_in_beam(player_pos, ceiling_pickup_pos);
        
        // Test pickup below player (floor) - y=295 (below player)  
        let floor_pickup_pos = (100.0, 295.0);
        let floor_in_beam = beam.is_point_in_beam(player_pos, floor_pickup_pos);
        
        // Upward beam should detect ceiling pickup (y < player_y)
        assert!(ceiling_in_beam, "Upward beam should detect ceiling pickup");
        assert!(!floor_in_beam, "Upward beam should NOT detect floor pickup");
    }

    #[test]
    fn test_real_world_ceiling_pickup_attraction() {
        use crate::tractor::{TractorBeam, BeamDir};
        
        let mut manager = PickupManager::new(0);
        let mut beam = TractorBeam::new();
        
        // Manually create a ceiling pickup at realistic coordinates
        let ceiling_y = 50.0;
        let pickup_y = ceiling_y + PickupConstants::WALL_OFFSET; // 55.0
        let ceiling_pickup = Pickup::new((100.0, pickup_y), PickupType::Fuel, true);
        manager.pickups.push(ceiling_pickup);
        
        let player_pos = (100.0, 200.0); // Player at middle of screen
        let dt = 0.016;
        
        // Test upward beam
        beam.activate(BeamDir::Up);
        
        // Try attraction
        for _ in 0..5 {
            manager.update_tractor_beam_attraction(&beam, player_pos, dt);
        }
        
        assert!(manager.pickups[0].being_attracted, "Ceiling pickup should be attracted by upward beam");
    }

    #[test]
    fn test_ceiling_pickup_with_game_coordinates() {
        use crate::tractor::{TractorBeam, BeamDir};
        
        let mut manager = PickupManager::new(0);
        let mut beam = TractorBeam::new();
        
        // Use actual game coordinates
        let player_pos = (100.0, 300.0); // Actual game player position
        let ceiling_y = 50.0;
        
        // Create ceiling pickup using spawn logic
        let pickup_y = ceiling_y + PickupConstants::WALL_OFFSET; // 55.0
        let ceiling_pickup = Pickup::new((100.0, pickup_y), PickupType::Fuel, true);
        manager.pickups.push(ceiling_pickup);
        
        let dt = 0.016;
        
        // Test upward beam
        beam.activate(BeamDir::Up);
        
        // Check if pickup is in beam
        let in_beam = beam.is_point_in_beam(player_pos, manager.pickups[0].position);
        
        // Calculate expected distance
        let dx = manager.pickups[0].position.0 - player_pos.0;
        let dy = manager.pickups[0].position.1 - player_pos.1;
        let distance = (dx*dx + dy*dy).sqrt();
        
        // Verify pickup is within range
        assert!(distance <= TractorBeam::MAX_RANGE, 
            "Pickup should be within beam range. Distance: {:.1}, Max: {}", 
            distance, TractorBeam::MAX_RANGE);
        
        assert!(in_beam, "Ceiling pickup should be detected by upward beam");
        
        // Try attraction
        manager.update_tractor_beam_attraction(&beam, player_pos, dt);
        
        assert!(manager.pickups[0].being_attracted, "Ceiling pickup should be attracted with game coordinates");
    }
}