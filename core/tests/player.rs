use core::player::{Player, PlayerConstants, PlayerInput, Vec2};

const DT: f32 = 1.0 / 60.0; // 60 FPS
const EPSILON: f32 = 0.001;

/// Helper function to create a player at origin.
fn create_test_player() -> Player {
    Player::new(Vec2::ZERO)
}

/// Helper function to assert floating point equality with epsilon.
fn assert_float_eq(a: f32, b: f32) {
    assert!((a - b).abs() < EPSILON, "Expected {}, got {}", b, a);
}

/// Tests that gravity is disabled (remains at zero).
#[test]
fn gravity_is_disabled() {
    let mut player = create_test_player();
    let initial_vel_y = player.vel.y;

    player.tick(DT, PlayerInput::default());

    // Velocity should remain unchanged since gravity is disabled
    assert_float_eq(player.vel.y, initial_vel_y);
}

/// Tests that upward thrust changes vertical velocity.
#[test]
fn up_thrust_affects_velocity_y() {
    let mut player = create_test_player();
    let initial_vel_y = player.vel.y;

    let input = PlayerInput {
        up: true,
        ..Default::default()
    };
    player.tick(DT, input);

    // Should have thrust applied (no gravity since it's disabled)
    let expected_vel_y = initial_vel_y + PlayerConstants::THRUST * DT;
    assert_float_eq(player.vel.y, expected_vel_y);
    assert!(
        player.vel.y < initial_vel_y,
        "Thrust should create upward velocity"
    );
}

/// Tests that downward thrust increases downward velocity.
#[test]
fn down_thrust_increases_velocity_y() {
    let mut player = create_test_player();
    let initial_vel_y = player.vel.y;

    let input = PlayerInput {
        down: true,
        ..Default::default()
    };
    player.tick(DT, input);

    // Only thrust force applied (no gravity)
    let thrust_force = -PlayerConstants::THRUST * PlayerConstants::DOWN_THRUST_MULTIPLIER;
    let expected_vel_y = initial_vel_y + thrust_force * DT;
    assert_float_eq(player.vel.y, expected_vel_y);
    assert!(
        player.vel.y > initial_vel_y,
        "Down thrust should create downward velocity"
    );
}

/// Tests that no horizontal input leaves horizontal velocity unchanged.
#[test]
fn no_horizontal_input_preserves_velocity_x() {
    let mut player = create_test_player();
    player.vel.x = 50.0; // Set some initial horizontal velocity

    player.tick(DT, PlayerInput::default());

    // Horizontal velocity should remain unchanged (no friction implemented)
    assert_float_eq(player.vel.x, 50.0);
}

/// Tests horizontal movement input affects velocity.
#[test]
fn horizontal_input_affects_velocity_x() {
    let mut player = create_test_player();

    // Test left input
    let left_input = PlayerInput {
        left: true,
        ..Default::default()
    };
    player.tick(DT, left_input);
    assert!(
        player.vel.x < 0.0,
        "Left input should create negative velocity"
    );

    // Reset player
    player = create_test_player();

    // Test right input
    let right_input = PlayerInput {
        right: true,
        ..Default::default()
    };
    player.tick(DT, right_input);
    assert!(
        player.vel.x > 0.0,
        "Right input should create positive velocity"
    );
}

/// Tests that horizontal speed is clamped to maximum.
#[test]
fn horizontal_speed_is_clamped() {
    let mut player = create_test_player();
    player.vel.x = PlayerConstants::MAX_HORIZONTAL_SPEED + 100.0; // Exceed max speed

    player.tick(DT, PlayerInput::default());

    assert_float_eq(player.vel.x, PlayerConstants::MAX_HORIZONTAL_SPEED);
}

/// Tests position updates based on velocity.
#[test]
fn position_updates_with_velocity() {
    let mut player = create_test_player();
    player.vel = Vec2::new(100.0, 50.0);

    player.tick(DT, PlayerInput::default());

    let expected_x = 100.0 * DT;
    let expected_y = 50.0 * DT; // No gravity effect

    assert_float_eq(player.pos.x, expected_x);
    assert_float_eq(player.pos.y, expected_y);
}

/// Tests multiple physics interactions in one tick.
#[test]
fn combined_physics_interactions() {
    let mut player = Player::new(Vec2::new(100.0, 200.0));
    player.vel = Vec2::new(50.0, -30.0); // Some initial velocity

    let input = PlayerInput {
        up: true,
        right: true,
        ..Default::default()
    };

    let initial_pos = player.pos;
    let initial_vel = player.vel;

    player.tick(DT, input);

    // Position should have changed
    assert!(player.pos.x != initial_pos.x);
    assert!(player.pos.y != initial_pos.y);

    // Velocity should be affected by thrust forces (no gravity)
    assert!(player.vel.x > initial_vel.x); // Right thrust
    assert!(player.vel.y != initial_vel.y); // Up thrust
}
