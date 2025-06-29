/// UI layout constants
pub mod constants {
    use macroquad::prelude::{Color, WHITE, YELLOW};

    /// Window dimensions
    pub const WINDOW_WIDTH: i32 = 800;
    pub const WINDOW_HEIGHT: i32 = 600;
    
    /// Button styling
    pub const BUTTON_WIDTH: f32 = 200.0;
    pub const BUTTON_HEIGHT: f32 = 50.0;
    pub const MENU_SPACING: f32 = 20.0;
    pub const BUTTON_BORDER_OFFSET: f32 = 5.0;
    pub const BUTTON_BORDER_THICKNESS: f32 = 2.0;
    
    /// Text positioning offsets
    pub const TITLE_OFFSET_X: f32 = 80.0;
    pub const INSTRUCTIONS_OFFSET_X: f32 = 180.0;
    pub const GAME_OVER_TITLE_OFFSET_X: f32 = 70.0;
    pub const DISTANCE_OFFSET_X: f32 = 80.0;
    pub const KEYBOARD_INSTRUCTIONS_OFFSET_X: f32 = 120.0;
    
    /// Font sizes
    pub const TITLE_FONT_SIZE: f32 = 40.0;
    pub const GAME_OVER_FONT_SIZE: f32 = 30.0;
    pub const PAUSED_FONT_SIZE: f32 = 30.0;
    pub const INSTRUCTIONS_FONT_SIZE: f32 = 16.0;
    pub const KEYBOARD_HELP_FONT_SIZE: f32 = 14.0;
    pub const DEATH_MESSAGE_FONT_SIZE: f32 = 18.0;
    
    /// Colors
    pub const SELECTED_COLOR: Color = YELLOW;
    pub const UNSELECTED_COLOR: Color = WHITE;
}

/// Helper functions for UI rendering
pub mod helpers {
    use super::constants::*;
    use macroquad::prelude::*;

    /// Draws a button with optional selection highlighting
    pub fn draw_highlighted_button(
        text: &str,
        position: Vec2,
        size: Vec2,
        selected: bool,
    ) -> bool {
        // Draw selection highlight
        if selected {
            macroquad::prelude::draw_rectangle_lines(
                position.x - BUTTON_BORDER_OFFSET,
                position.y - BUTTON_BORDER_OFFSET,
                size.x + BUTTON_BORDER_OFFSET * 2.0,
                size.y + BUTTON_BORDER_OFFSET * 2.0,
                BUTTON_BORDER_THICKNESS,
                SELECTED_COLOR,
            );
        } else {
            macroquad::prelude::draw_rectangle_lines(
                position.x - BUTTON_BORDER_OFFSET,
                position.y - BUTTON_BORDER_OFFSET,
                size.x + BUTTON_BORDER_OFFSET * 2.0,
                size.y + BUTTON_BORDER_OFFSET * 2.0,
                BUTTON_BORDER_THICKNESS,
                UNSELECTED_COLOR,
            );
        }

        // Draw the actual button using macroquad widgets
        macroquad::ui::widgets::Button::new(text)
            .position(position)
            .size(size)
            .ui(&mut macroquad::ui::root_ui())
    }

    /// Draws centered text with automatic positioning
    pub fn draw_centered_text(
        text: &str,
        center_y: f32,
        offset_x: f32,
        font_size: f32,
        color: Color,
    ) {
        macroquad::prelude::draw_text(
            text,
            WINDOW_WIDTH as f32 / 2.0 - offset_x,
            center_y,
            font_size,
            color,
        );
    }

    /// Draws the keyboard instruction text
    pub fn draw_keyboard_instructions(center_y: f32, y_offset: f32) {
        draw_centered_text(
            "Press SPACE or ENTER to select",
            center_y + y_offset,
            KEYBOARD_INSTRUCTIONS_OFFSET_X,
            KEYBOARD_HELP_FONT_SIZE,
            macroquad::prelude::GRAY,
        );
    }

    /// Calculates centered button position
    pub fn center_button_position(center_y: f32, button_index: usize) -> Vec2 {
        let center_x = WINDOW_WIDTH as f32 / 2.0 - BUTTON_WIDTH / 2.0;
        let y_position = center_y + (button_index as f32) * (BUTTON_HEIGHT + MENU_SPACING);
        Vec2::new(center_x, y_position)
    }

    /// Standard button size
    pub fn button_size() -> Vec2 {
        Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT)
    }

    /// Handles standard up/down menu navigation
    pub fn handle_menu_navigation(current_selection: usize, max_options: usize) -> usize {
        if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Up) && current_selection > 0 {
            current_selection - 1
        } else if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Down) && current_selection < max_options - 1 {
            current_selection + 1
        } else if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Up) && current_selection == 0 {
            max_options - 1 // Wrap around to bottom
        } else if macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Down) && current_selection == max_options - 1 {
            0 // Wrap around to top
        } else {
            current_selection
        }
    }

    /// Checks if selection key (Enter or Space) was pressed
    pub fn is_selection_key_pressed() -> bool {
        macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Enter) || macroquad::prelude::is_key_pressed(macroquad::prelude::KeyCode::Space)
    }
}