/// Pickup system for collectible items in the game.
///
/// Manages fuel depots and other collectibles that spawn on cave walls.

use crate::cave::SimpleRng;
use crate::constants::PickupConstants;

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
    /// Type of pickup
    pub pickup_type: PickupType,
    /// Whether the pickup is attached to the ceiling (true) or floor (false)
    pub is_on_ceiling: bool,
    /// Whether this pickup has been collected
    pub collected: bool,
}

impl Pickup {
    /// Creates a new pickup at the specified position.
    pub fn new(position: (f32, f32), pickup_type: PickupType, is_on_ceiling: bool) -> Self {
        Self {
            position,
            pickup_type,
            is_on_ceiling,
            collected: false,
        }
    }

    /// Marks this pickup as collected.
    pub fn collect(&mut self) {
        self.collected = true;
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
}