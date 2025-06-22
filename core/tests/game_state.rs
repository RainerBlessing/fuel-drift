// Datei: core/tests/game_state.rs (erweitert)

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
    assert_eq!(
        GameState::Playing.next(GameEvent::PauseToggle),
        GameState::Paused
    );
    assert_eq!(
        GameState::Playing.next(GameEvent::Dead),
        GameState::GameOver
    );
}

/// Tests for valid state transitions from Paused state.
#[test]
fn paused_state_transitions() {
    assert_eq!(
        GameState::Paused.next(GameEvent::PauseToggle),
        GameState::Playing
    );
    assert_eq!(GameState::Paused.next(GameEvent::Reset), GameState::Menu);
    assert_eq!(
        GameState::Paused.next(GameEvent::BackToMenu),
        GameState::Menu
    );
}

/// Tests for valid state transitions from GameOver state.
#[test]
fn game_over_state_transitions() {
    assert_eq!(
        GameState::GameOver.next(GameEvent::Start),
        GameState::Playing
    );
    assert_eq!(GameState::GameOver.next(GameEvent::Reset), GameState::Menu);
    assert_eq!(
        GameState::GameOver.next(GameEvent::BackToMenu),
        GameState::Menu
    );
}

/// Tests that BackToMenu event works correctly from Paused state.
#[test]
fn back_to_menu_from_paused() {
    let mut state_machine = StateMachine::new();

    // Start game and pause
    state_machine.handle_event(GameEvent::Start);
    state_machine.handle_event(GameEvent::PauseToggle);
    assert_eq!(state_machine.current(), GameState::Paused);

    // Go back to menu
    state_machine.handle_event(GameEvent::BackToMenu);
    assert_eq!(state_machine.current(), GameState::Menu);
}

/// Tests that BackToMenu event works correctly from GameOver state.
#[test]
fn back_to_menu_from_game_over() {
    let mut state_machine = StateMachine::new();

    // Start game and die
    state_machine.handle_event(GameEvent::Start);
    state_machine.handle_event(GameEvent::Dead);
    assert_eq!(state_machine.current(), GameState::GameOver);

    // Go back to menu
    state_machine.handle_event(GameEvent::BackToMenu);
    assert_eq!(state_machine.current(), GameState::Menu);
}

/// Tests complete game flow with menu navigation.
#[test]
fn complete_game_flow_with_menus() {
    let mut state_machine = StateMachine::new();

    // Start in menu
    assert_eq!(state_machine.current(), GameState::Menu);

    // Start game
    state_machine.handle_event(GameEvent::Start);
    assert_eq!(state_machine.current(), GameState::Playing);

    // Pause game
    state_machine.handle_event(GameEvent::PauseToggle);
    assert_eq!(state_machine.current(), GameState::Paused);

    // Go back to menu from pause
    state_machine.handle_event(GameEvent::BackToMenu);
    assert_eq!(state_machine.current(), GameState::Menu);

    // Start game again
    state_machine.handle_event(GameEvent::Start);
    assert_eq!(state_machine.current(), GameState::Playing);

    // Die
    state_machine.handle_event(GameEvent::Dead);
    assert_eq!(state_machine.current(), GameState::GameOver);

    // Go back to menu from game over
    state_machine.handle_event(GameEvent::BackToMenu);
    assert_eq!(state_machine.current(), GameState::Menu);
}

/// Tests that invalid transitions with BackToMenu remain in current state.
#[test]
fn invalid_back_to_menu_transitions() {
    // BackToMenu from Menu should remain in Menu
    assert_eq!(GameState::Menu.next(GameEvent::BackToMenu), GameState::Menu);

    // BackToMenu from Playing should remain in Playing
    assert_eq!(
        GameState::Playing.next(GameEvent::BackToMenu),
        GameState::Playing
    );
}

/// Tests that all existing invalid transitions still work correctly.
#[test]
fn invalid_transitions_remain_in_current_state() {
    // Menu state invalid transitions
    assert_eq!(
        GameState::Menu.next(GameEvent::PauseToggle),
        GameState::Menu
    );
    assert_eq!(GameState::Menu.next(GameEvent::Dead), GameState::Menu);
    assert_eq!(GameState::Menu.next(GameEvent::BackToMenu), GameState::Menu);

    // Playing state invalid transitions
    assert_eq!(
        GameState::Playing.next(GameEvent::Start),
        GameState::Playing
    );
    assert_eq!(
        GameState::Playing.next(GameEvent::Reset),
        GameState::Playing
    );
    assert_eq!(
        GameState::Playing.next(GameEvent::BackToMenu),
        GameState::Playing
    );

    // Paused state invalid transitions
    assert_eq!(GameState::Paused.next(GameEvent::Start), GameState::Paused);
    assert_eq!(GameState::Paused.next(GameEvent::Dead), GameState::Paused);

    // GameOver state invalid transitions
    assert_eq!(
        GameState::GameOver.next(GameEvent::PauseToggle),
        GameState::GameOver
    );
    assert_eq!(
        GameState::GameOver.next(GameEvent::Dead),
        GameState::GameOver
    );
}

/// Tests Reset vs BackToMenu distinction in different states.
#[test]
fn reset_vs_back_to_menu_distinction() {
    // From Paused: both Reset and BackToMenu go to Menu
    assert_eq!(GameState::Paused.next(GameEvent::Reset), GameState::Menu);
    assert_eq!(
        GameState::Paused.next(GameEvent::BackToMenu),
        GameState::Menu
    );

    // From GameOver: both Reset and BackToMenu go to Menu
    assert_eq!(GameState::GameOver.next(GameEvent::Reset), GameState::Menu);
    assert_eq!(
        GameState::GameOver.next(GameEvent::BackToMenu),
        GameState::Menu
    );

    // Both events have the same effect, but semantically different meanings
    // Reset implies "reset game state", BackToMenu implies "navigation"
}

/// Tests edge case scenarios with multiple rapid state changes.
#[test]
fn rapid_state_changes() {
    let mut state_machine = StateMachine::new();

    // Rapid transitions
    state_machine.handle_event(GameEvent::Start);
    state_machine.handle_event(GameEvent::PauseToggle);
    state_machine.handle_event(GameEvent::BackToMenu);
    state_machine.handle_event(GameEvent::Start);
    state_machine.handle_event(GameEvent::Dead);
    state_machine.handle_event(GameEvent::BackToMenu);

    // Should end up in Menu
    assert_eq!(state_machine.current(), GameState::Menu);
}

/// Tests that StateMachine wrapper handles BackToMenu correctly.
#[test]
fn state_machine_handles_back_to_menu() {
    let mut state_machine = StateMachine::new();

    // Navigate to paused state
    state_machine.handle_event(GameEvent::Start);
    state_machine.handle_event(GameEvent::PauseToggle);
    assert_eq!(state_machine.current(), GameState::Paused);

    // Use BackToMenu
    state_machine.handle_event(GameEvent::BackToMenu);
    assert_eq!(state_machine.current(), GameState::Menu);
}

/// Tests game state enum properties remain consistent.
#[test]
fn game_state_enum_properties() {
    // Test that all states are distinct
    let states = [
        GameState::Menu,
        GameState::Playing,
        GameState::Paused,
        GameState::GameOver,
    ];

    for (i, &state1) in states.iter().enumerate() {
        for (j, &state2) in states.iter().enumerate() {
            if i == j {
                assert_eq!(state1, state2);
            } else {
                assert_ne!(state1, state2);
            }
        }
    }
}

/// Tests game event enum properties.
#[test]
fn game_event_enum_properties() {
    // Test that all events are distinct
    let events = [
        GameEvent::Start,
        GameEvent::PauseToggle,
        GameEvent::Dead,
        GameEvent::Reset,
        GameEvent::BackToMenu,
    ];

    for (i, &event1) in events.iter().enumerate() {
        for (j, &event2) in events.iter().enumerate() {
            if i == j {
                assert_eq!(event1, event2);
            } else {
                assert_ne!(event1, event2);
            }
        }
    }
}

/// Tests specific menu navigation scenarios.
#[test]
fn menu_navigation_scenarios() {
    let mut state_machine = StateMachine::new();

    // Scenario 1: Start game, pause, back to menu, start again
    state_machine.handle_event(GameEvent::Start);
    state_machine.handle_event(GameEvent::PauseToggle);
    state_machine.handle_event(GameEvent::BackToMenu);
    state_machine.handle_event(GameEvent::Start);
    assert_eq!(state_machine.current(), GameState::Playing);

    // Scenario 2: Die, back to menu, start again
    state_machine.handle_event(GameEvent::Dead);
    state_machine.handle_event(GameEvent::BackToMenu);
    state_machine.handle_event(GameEvent::Start);
    assert_eq!(state_machine.current(), GameState::Playing);
}
