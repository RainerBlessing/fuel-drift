/// Menu handling for the Fuel Drift game.
///
/// This module contains all menu-related logic separated from the main game loop,
/// following the Single Responsibility Principle for UI management.
use crate::ui::{constants::*, helpers::*};
use core::audio::AudioEvent;
use macroquad::prelude::*;

/// Menu selection state for the main menu
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MainMenuSelection {
    Start,
    Quit,
}

impl MainMenuSelection {
    /// Toggle between Start and Quit options
    pub fn toggle(self) -> Self {
        match self {
            MainMenuSelection::Start => MainMenuSelection::Quit,
            MainMenuSelection::Quit => MainMenuSelection::Start,
        }
    }
    
    /// Convert to index for array-based operations
    pub fn to_index(self) -> usize {
        match self {
            MainMenuSelection::Start => 0,
            MainMenuSelection::Quit => 1,
        }
    }
}

/// Menu state for keyboard navigation
pub struct MenuState {
    pub main_menu_selection: MainMenuSelection,
    pub pause_menu_selection: usize, // 0 = Resume, 1 = Back to Menu
    pub game_over_menu_selection: usize, // 0 = Replay, 1 = Back to Menu
}

impl MenuState {
    pub fn new() -> Self {
        Self {
            main_menu_selection: MainMenuSelection::Start,
            pause_menu_selection: 0,
            game_over_menu_selection: 0,
        }
    }
}

/// Main menu controller
pub struct MainMenuController;

impl MainMenuController {
    /// Handles main menu input and state updates
    pub fn handle_input(
        menu_state: &mut MenuState,
        audio_events: &mut Vec<AudioEvent>
    ) -> Option<MainMenuAction> {
        // Handle keyboard navigation
        if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Up) || macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Down) {
            menu_state.main_menu_selection = menu_state.main_menu_selection.toggle();
            audio_events.push(AudioEvent::ButtonClick);
        }

        // Handle selection
        if is_selection_key_pressed() {
            audio_events.push(AudioEvent::ButtonClick);
            match menu_state.main_menu_selection {
                MainMenuSelection::Start => Some(MainMenuAction::StartGame),
                MainMenuSelection::Quit => Some(MainMenuAction::QuitGame),
            }
        } else {
            None
        }
    }

    /// Renders the main menu
    pub fn render(menu_state: &MenuState, audio_events: &mut Vec<AudioEvent>) -> Option<MainMenuAction> {
        let center_y = WINDOW_HEIGHT as f32 / 2.0;

        // Title
        draw_centered_text(
            "FUEL DRIFT",
            center_y - 100.0,
            TITLE_OFFSET_X,
            TITLE_FONT_SIZE,
            WHITE,
        );

        // Instructions
        draw_centered_text(
            "Arrow keys: Move | W/S: Tractor Beam | Watch your fuel!",
            center_y - 50.0,
            INSTRUCTIONS_OFFSET_X,
            INSTRUCTIONS_FONT_SIZE,
            GRAY,
        );

        // Keyboard instructions
        draw_keyboard_instructions(center_y, -30.0);

        // Start button
        let start_position = center_button_position(center_y, 0);
        let start_selected = menu_state.main_menu_selection == MainMenuSelection::Start;
        
        if draw_highlighted_button("Start Game", start_position, button_size(), start_selected) {
            audio_events.push(AudioEvent::ButtonClick);
            return Some(MainMenuAction::StartGame);
        }

        // Quit button
        let quit_position = center_button_position(center_y, 1);
        let quit_selected = menu_state.main_menu_selection == MainMenuSelection::Quit;
        
        if draw_highlighted_button("Quit", quit_position, button_size(), quit_selected) {
            audio_events.push(AudioEvent::ButtonClick);
            return Some(MainMenuAction::QuitGame);
        }

        None
    }
}

/// Pause menu controller
pub struct PauseMenuController;

impl PauseMenuController {
    /// Handles pause menu input and state updates
    pub fn handle_input(
        menu_state: &mut MenuState,
        audio_events: &mut Vec<AudioEvent>
    ) -> Option<PauseMenuAction> {
        // Handle keyboard navigation
        if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Up) || macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Down) {
            menu_state.pause_menu_selection = 1 - menu_state.pause_menu_selection;
            audio_events.push(AudioEvent::ButtonClick);
        }

        // Handle selection
        if is_selection_key_pressed() {
            audio_events.push(AudioEvent::ButtonClick);
            match menu_state.pause_menu_selection {
                0 => Some(PauseMenuAction::Resume),
                1 => Some(PauseMenuAction::BackToMenu),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Renders the pause menu
    pub fn render(menu_state: &MenuState, audio_events: &mut Vec<AudioEvent>) -> Option<PauseMenuAction> {
        // Semi-transparent overlay
        draw_rectangle(
            0.0,
            0.0,
            WINDOW_WIDTH as f32,
            WINDOW_HEIGHT as f32,
            Color::new(0.0, 0.0, 0.0, 0.7),
        );

        let center_y = WINDOW_HEIGHT as f32 / 2.0;

        // Title
        draw_centered_text(
            "PAUSED",
            center_y - 50.0,
            50.0, // Custom offset for "PAUSED"
            PAUSED_FONT_SIZE,
            WHITE,
        );

        // Keyboard instructions
        draw_keyboard_instructions(center_y, -10.0);

        // Resume button
        let resume_position = center_button_position(center_y, 0);
        let resume_selected = menu_state.pause_menu_selection == 0;
        
        if draw_highlighted_button("Resume", resume_position, button_size(), resume_selected) {
            audio_events.push(AudioEvent::ButtonClick);
            return Some(PauseMenuAction::Resume);
        }

        // Back to Menu button
        let back_position = center_button_position(center_y, 1);
        let back_selected = menu_state.pause_menu_selection == 1;
        
        if draw_highlighted_button("Back to Menu", back_position, button_size(), back_selected) {
            audio_events.push(AudioEvent::ButtonClick);
            return Some(PauseMenuAction::BackToMenu);
        }

        None
    }
}

/// Game over menu controller
pub struct GameOverMenuController;

impl GameOverMenuController {
    /// Handles game over menu input and state updates
    pub fn handle_input(
        menu_state: &mut MenuState,
        audio_events: &mut Vec<AudioEvent>
    ) -> Option<GameOverMenuAction> {
        // Handle keyboard navigation
        if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Up) || macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Down) {
            menu_state.game_over_menu_selection = 1 - menu_state.game_over_menu_selection;
            audio_events.push(AudioEvent::ButtonClick);
        }

        // Handle selection
        if is_selection_key_pressed() {
            audio_events.push(AudioEvent::ButtonClick);
            match menu_state.game_over_menu_selection {
                0 => Some(GameOverMenuAction::Replay),
                1 => Some(GameOverMenuAction::BackToMenu),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Renders the game over menu
    pub fn render(
        menu_state: &MenuState, 
        fuel_empty: bool,
        distance_text: &str,
        audio_events: &mut Vec<AudioEvent>
    ) -> Option<GameOverMenuAction> {
        let center_y = WINDOW_HEIGHT as f32 / 2.0;

        let death_message = if fuel_empty {
            "OUT OF FUEL!"
        } else {
            "CRASHED!"
        };

        // Game Over title
        draw_centered_text(
            "GAME OVER",
            center_y - 80.0,
            GAME_OVER_TITLE_OFFSET_X,
            GAME_OVER_FONT_SIZE,
            RED,
        );

        // Death message
        draw_centered_text(
            death_message,
            center_y - 50.0,
            65.0, // Custom offset for death message
            DEATH_MESSAGE_FONT_SIZE,
            WHITE,
        );

        // Show final distance
        let distance_display = format!("Distance: {}", distance_text);
        draw_centered_text(
            &distance_display,
            center_y - 25.0,
            DISTANCE_OFFSET_X,
            INSTRUCTIONS_FONT_SIZE,
            YELLOW,
        );

        // Keyboard instructions
        draw_keyboard_instructions(center_y, 0.0);

        // Replay button
        let replay_position = center_button_position(center_y, 0);
        let replay_position = Vec2::new(replay_position.x, replay_position.y + 20.0); // Offset for distance text
        let replay_selected = menu_state.game_over_menu_selection == 0;
        
        if draw_highlighted_button("Replay", replay_position, button_size(), replay_selected) {
            audio_events.push(AudioEvent::ButtonClick);
            return Some(GameOverMenuAction::Replay);
        }

        // Back to Menu button
        let back_position = center_button_position(center_y, 1);
        let back_position = Vec2::new(back_position.x, back_position.y + 20.0); // Offset for distance text
        let back_selected = menu_state.game_over_menu_selection == 1;
        
        if draw_highlighted_button("Back to Menu", back_position, button_size(), back_selected) {
            audio_events.push(AudioEvent::ButtonClick);
            return Some(GameOverMenuAction::BackToMenu);
        }

        None
    }
}

/// Actions that can be triggered from the main menu
#[derive(Debug, Clone, Copy)]
pub enum MainMenuAction {
    StartGame,
    QuitGame,
}

/// Actions that can be triggered from the pause menu
#[derive(Debug, Clone, Copy)]
pub enum PauseMenuAction {
    Resume,
    BackToMenu,
}

/// Actions that can be triggered from the game over menu
#[derive(Debug, Clone, Copy)]
pub enum GameOverMenuAction {
    Replay,
    BackToMenu,
}