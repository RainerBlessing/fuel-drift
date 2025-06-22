use core::game_state::{GameEvent, GameState, StateMachine};

/// Tests for valid state transitions from Menu state.
#[test]
fn menu_state_transitions() {
    assert_eq!(GameState::Menu.next(GameEvent::Start), GameState::Playing);
    assert_eq!(GameState::Menu.next(GameEvent::Reset), GameState::Menu);
}

/// Tests for valid state transitions from Playing state.
#[test]
fn playing_state_transitions() {
    assert_eq!(GameState::Playing.next(GameEvent::PauseToggle), GameState::Paused);
    assert_eq!(GameState::Playing.next(GameEvent::Dead), GameState::GameOver);
}

/// Tests for valid state transitions from Paused state.
#[test]
fn paused_state_transitions() {
    assert_eq!(GameState::Paused.next(GameEvent::PauseToggle), GameState::Playing);
    assert_eq!(GameState::Paused.next(GameEvent::Reset), GameState::Menu);
}

/// Tests for valid state transitions from GameOver state.
#[test]
fn game_over_state_transitions() {
    assert_eq!(GameState::GameOver.next(GameEvent::Start), GameState::Playing);
    assert_eq!(GameState::GameOver.next(GameEvent::Reset), GameState::Menu);
}

/// Tests that invalid transitions keep the current state.
#[test]
fn invalid_transitions_remain_in_current_state() {
    // Menu state invalid transitions
    assert_eq!(GameState::Menu.next(GameEvent::PauseToggle), GameState::Menu);
    assert_eq!(GameState::Menu.next(GameEvent::Dead), GameState::Menu);

    // Playing state invalid transitions
    assert_eq!(GameState::Playing.next(GameEvent::Start), GameState::Playing);
    assert_eq!(GameState::Playing.next(GameEvent::Reset), GameState::Playing);

    // Paused state invalid transitions
    assert_eq!(GameState::Paused.next(GameEvent::Start), GameState::Paused);
    assert_eq!(GameState::Paused.next(GameEvent::Dead), GameState::Paused);

    // GameOver state invalid transitions
    assert_eq!(GameState::GameOver.next(GameEvent::PauseToggle), GameState::GameOver);
    assert_eq!(GameState::GameOver.next(GameEvent::Dead), GameState::GameOver);
}

/// Tests the StateMachine wrapper functionality.
#[test]
fn state_machine_functionality() {
    let mut state_machine = StateMachine::new();

    // Initial state should be Menu
    assert_eq!(state_machine.current(), GameState::Menu);

    // Start game
    state_machine.handle_event(GameEvent::Start);
    assert_eq!(state_machine.current(), GameState::Playing);

    // Pause game
    state_machine.handle_event(GameEvent::PauseToggle);
    assert_eq!(state_machine.current(), GameState::Paused);

    // Resume game
    state_machine.handle_event(GameEvent::PauseToggle);
    assert_eq!(state_machine.current(), GameState::Playing);

    // Player dies
    state_machine.handle_event(GameEvent::Dead);
    assert_eq!(state_machine.current(), GameState::GameOver);

    // Reset to menu
    state_machine.handle_event(GameEvent::Reset);
    assert_eq!(state_machine.current(), GameState::Menu);
}

/// Tests that StateMachine implements Default correctly.
#[test]
fn state_machine_default() {
    let state_machine = StateMachine::default();
    assert_eq!(state_machine.current(), GameState::Menu);
}