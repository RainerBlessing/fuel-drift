/// Level system module for Fuel Drift
/// 
/// Manages level progression and difficulty parameters

/// Errors that can occur in the level system
#[derive(Debug, Clone, PartialEq)]
pub enum LevelError {
    /// Invalid level index requested
    InvalidLevelIndex(usize),
    /// No levels configured in the manager
    EmptyLevelList,
}

impl std::fmt::Display for LevelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LevelError::InvalidLevelIndex(index) => {
                write!(f, "Invalid level index: {}", index)
            }
            LevelError::EmptyLevelList => {
                write!(f, "No levels configured in level manager")
            }
        }
    }
}

impl std::error::Error for LevelError {}

/// Result type for level operations
pub type LevelResult<T> = Result<T, LevelError>;

/// Represents a single level configuration
#[derive(Debug, Clone, PartialEq)]
pub struct Level {
    /// Level number (1, 2, 3, ...)
    pub number: u32,
    /// Duration of the level in seconds
    pub duration_seconds: f32,
    /// Average distance between fuel spawns in pixels
    pub fuel_spawn_distance: f32,
    /// Width of the cave passage in pixels
    pub cave_width: f32,
}

impl Level {
    /// Creates a new level with the given parameters
    pub fn new(number: u32, duration_seconds: f32, fuel_spawn_distance: f32, cave_width: f32) -> Self {
        Self {
            number,
            duration_seconds,
            fuel_spawn_distance,
            cave_width,
        }
    }
}

/// Manages level progression and configuration
pub struct LevelManager {
    levels: Vec<Level>,
    current_level_index: usize,
    level_start_time: f32,
}

impl LevelManager {
    /// Creates a new level manager with default level configurations
    pub fn new() -> Self {
        let levels = vec![
            // Level 1: Easy introduction
            Level::new(1, 60.0, 300.0, 200.0),
            // Level 2: Slightly harder
            Level::new(2, 90.0, 400.0, 180.0),
            // Level 3: Medium difficulty
            Level::new(3, 120.0, 500.0, 160.0),
            // Level 4: Getting challenging
            Level::new(4, 120.0, 600.0, 140.0),
            // Level 5: Hard
            Level::new(5, 150.0, 700.0, 120.0),
            // Level 6+: Very hard (repeats)
            Level::new(6, 180.0, 800.0, 100.0),
        ];

        Self {
            levels,
            current_level_index: 0,
            level_start_time: 0.0,
        }
    }

    /// Creates a level manager with custom levels
    pub fn with_levels(levels: Vec<Level>) -> LevelResult<Self> {
        if levels.is_empty() {
            return Err(LevelError::EmptyLevelList);
        }
        Ok(Self {
            levels,
            current_level_index: 0,
            level_start_time: 0.0,
        })
    }

    /// Gets the current level
    pub fn current_level(&self) -> LevelResult<&Level> {
        self.levels.get(self.current_level_index)
            .ok_or(LevelError::InvalidLevelIndex(self.current_level_index))
    }

    /// Updates the level manager, checking for level progression
    pub fn update(&mut self, current_time: f32) -> LevelResult<bool> {
        let elapsed = current_time - self.level_start_time;
        let current_level = self.levels.get(self.current_level_index)
            .ok_or(LevelError::InvalidLevelIndex(self.current_level_index))?;
        
        if elapsed >= current_level.duration_seconds {
            Ok(self.advance_level(current_time))
        } else {
            Ok(false)
        }
    }

    /// Advances to the next level
    fn advance_level(&mut self, current_time: f32) -> bool {
        if self.current_level_index < self.levels.len() - 1 {
            self.current_level_index += 1;
        }
        self.level_start_time = current_time;
        true
    }

    /// Gets the progress through the current level (0.0 to 1.0)
    pub fn level_progress(&self, current_time: f32) -> LevelResult<f32> {
        let elapsed = current_time - self.level_start_time;
        let current_level = self.levels.get(self.current_level_index)
            .ok_or(LevelError::InvalidLevelIndex(self.current_level_index))?;
        Ok((elapsed / current_level.duration_seconds).min(1.0))
    }

    /// Resets the level manager to the first level
    pub fn reset(&mut self) {
        self.current_level_index = 0;
        self.level_start_time = 0.0;
    }
}

impl Default for LevelManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_creation() {
        let level = Level::new(1, 60.0, 300.0, 200.0);
        assert_eq!(level.number, 1);
        assert_eq!(level.duration_seconds, 60.0);
        assert_eq!(level.fuel_spawn_distance, 300.0);
        assert_eq!(level.cave_width, 200.0);
    }

    #[test]
    fn test_level_manager_creation() {
        let manager = LevelManager::new();
        assert_eq!(manager.current_level().unwrap().number, 1);
        assert_eq!(manager.current_level_index, 0);
    }

    #[test]
    fn test_level_progression() {
        let mut manager = LevelManager::new();
        
        // Should not advance before duration
        assert!(!manager.update(30.0).unwrap());
        assert_eq!(manager.current_level().unwrap().number, 1);
        
        // Should advance after duration
        assert!(manager.update(60.1).unwrap());
        assert_eq!(manager.current_level().unwrap().number, 2);
        
        // Progress through level 2
        assert!(!manager.update(120.0).unwrap());
        assert!(manager.update(150.1).unwrap());
        assert_eq!(manager.current_level().unwrap().number, 3);
    }

    #[test]
    fn test_level_progress() {
        let manager = LevelManager::new();
        
        assert_eq!(manager.level_progress(0.0).unwrap(), 0.0);
        assert_eq!(manager.level_progress(30.0).unwrap(), 0.5);
        assert_eq!(manager.level_progress(60.0).unwrap(), 1.0);
        assert_eq!(manager.level_progress(90.0).unwrap(), 1.0); // Capped at 1.0
    }

    #[test]
    fn test_max_level() {
        let mut manager = LevelManager::with_levels(vec![
            Level::new(1, 10.0, 100.0, 200.0),
            Level::new(2, 10.0, 100.0, 200.0),
        ]).unwrap();
        
        assert!(manager.update(11.0).unwrap());
        assert_eq!(manager.current_level().unwrap().number, 2);
        
        // Should stay at max level
        assert!(manager.update(22.0).unwrap());
        assert_eq!(manager.current_level().unwrap().number, 2);
        assert_eq!(manager.current_level_index, 1);
    }

    #[test]
    fn test_reset() {
        let mut manager = LevelManager::new();
        manager.update(61.0).unwrap();
        assert_eq!(manager.current_level().unwrap().number, 2);
        
        manager.reset();
        assert_eq!(manager.current_level().unwrap().number, 1);
        assert_eq!(manager.current_level_index, 0);
        assert_eq!(manager.level_start_time, 0.0);
    }

    #[test]
    fn test_error_handling() {
        // Test empty level list
        let result = LevelManager::with_levels(vec![]);
        assert!(matches!(result, Err(LevelError::EmptyLevelList)));
        
        // Test error display
        let error = LevelError::InvalidLevelIndex(5);
        assert_eq!(error.to_string(), "Invalid level index: 5");
    }
}