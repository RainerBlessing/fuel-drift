/// Represents all possible states in the Fuel Drift game.
///
/// Following the State pattern for clear state management and transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}

/// Events that can trigger state transitions.
///
/// Each event represents a single user action or game condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameEvent {
    Start,
    PauseToggle,
    Dead,
    Reset,
}

impl GameState {
    /// Determines the next state based on current state and event.
    ///
    /// Uses pattern matching for clear, testable state transitions
    /// with low cyclomatic complexity per transition.
    pub fn next(self, event: GameEvent) -> Self {
        match (self, event) {
            // From Menu
            (GameState::Menu, GameEvent::Start) => GameState::Playing,
            (GameState::Menu, GameEvent::Reset) => GameState::Menu,

            // From Playing
            (GameState::Playing, GameEvent::PauseToggle) => GameState::Paused,
            (GameState::Playing, GameEvent::Dead) => GameState::GameOver,

            // From Paused
            (GameState::Paused, GameEvent::PauseToggle) => GameState::Playing,
            (GameState::Paused, GameEvent::Reset) => GameState::Menu,

            // From GameOver
            (GameState::GameOver, GameEvent::Start) => GameState::Playing,
            (GameState::GameOver, GameEvent::Reset) => GameState::Menu,

            // Invalid transitions remain in current state
            (state, _) => state,
        }
    }
}

/// State machine for managing game state transitions.
///
/// Encapsulates state and provides controlled access following
/// the Single Responsibility Principle.
#[derive(Debug, Clone, Copy)]
pub struct StateMachine {
    current_state: GameState,
}

impl StateMachine {
    /// Creates a new state machine starting in Menu state.
    pub fn new() -> Self {
        Self {
            current_state: GameState::Menu,
        }
    }

    /// Gets the current state.
    pub fn current(&self) -> GameState {
        self.current_state
    }

    /// Processes an event and transitions to the next state.
    pub fn handle_event(&mut self, event: GameEvent) {
        self.current_state = self.current_state.next(event);
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}