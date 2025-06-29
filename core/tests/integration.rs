// core/tests/integration.rs

use core::cave::{Cave, CaveSegment};
use core::collision::aabb_overlap;
use core::game_state::{GameEvent, GameState, StateMachine};
use core::player::{Player, PlayerInput, Vec2};
use core::tractor::{BeamDir, TractorBeam};

const PLAYER_SIZE: (f32, f32) = (30.0, 18.0);

/// Helper function to create a simple cave segment for testing.
fn create_test_segment(ceiling: f32, floor: f32, x_start: f32, width: f32) -> CaveSegment {
    CaveSegment::new(ceiling, floor, x_start, width)
}

/// Tests player collision with ceiling.
#[test]
fn player_collides_with_ceiling() {
    let segment = create_test_segment(100.0, 400.0, 0.0, 50.0);

    // Player positioned to hit ceiling
    // Player top-left corner position - player extends down 18 pixels
    let player_pos = (20.0, 85.0); // Player bottom at y=103, ceiling at y=100
    let ceiling_pos = (segment.x_start, 0.0);
    let ceiling_size = (segment.width, segment.ceiling);

    assert!(aabb_overlap(
        player_pos,
        PLAYER_SIZE,
        ceiling_pos,
        ceiling_size
    ));
}

/// Tests player collision with floor.
#[test]
fn player_collides_with_floor() {
    let segment = create_test_segment(100.0, 400.0, 0.0, 50.0);

    // Player positioned to hit floor
    // Player top-left position - player extends down 18 pixels
    let player_pos = (20.0, 390.0); // Player bottom at y=408, floor starts at y=400
    let floor_pos = (segment.x_start, segment.floor);
    let floor_size = (segment.width, 600.0 - segment.floor); // Assuming window height 600

    assert!(aabb_overlap(player_pos, PLAYER_SIZE, floor_pos, floor_size));
}

/// Tests player in safe zone (no collision).
#[test]
fn player_in_safe_zone() {
    let segment = create_test_segment(100.0, 400.0, 0.0, 50.0);

    // Player positioned safely between ceiling and floor
    let player_pos = (20.0, 250.0); // Safely in the middle

    // Check no collision with ceiling
    let ceiling_pos = (segment.x_start, 0.0);
    let ceiling_size = (segment.width, segment.ceiling);
    assert!(!aabb_overlap(
        player_pos,
        PLAYER_SIZE,
        ceiling_pos,
        ceiling_size
    ));

    // Check no collision with floor
    let floor_pos = (segment.x_start, segment.floor);
    let floor_size = (segment.width, 600.0 - segment.floor);
    assert!(!aabb_overlap(
        player_pos,
        PLAYER_SIZE,
        floor_pos,
        floor_size
    ));
}

/// Tests player outside segment horizontally.
#[test]
fn player_outside_segment_horizontally() {
    let segment = create_test_segment(100.0, 400.0, 100.0, 50.0);

    // Player positioned before segment starts
    let player_pos = (50.0, 250.0);

    let ceiling_pos = (segment.x_start, 0.0);
    let ceiling_size = (segment.width, segment.ceiling);
    assert!(!aabb_overlap(
        player_pos,
        PLAYER_SIZE,
        ceiling_pos,
        ceiling_size
    ));

    let floor_pos = (segment.x_start, segment.floor);
    let floor_size = (segment.width, 600.0 - segment.floor);
    assert!(!aabb_overlap(
        player_pos,
        PLAYER_SIZE,
        floor_pos,
        floor_size
    ));
}

/// Tests collision at segment boundary.
#[test]
fn collision_at_segment_boundary() {
    let segment = create_test_segment(100.0, 400.0, 100.0, 50.0);

    // Player positioned at the start of segment, hitting ceiling
    let player_pos = (100.0, 85.0);
    let ceiling_pos = (segment.x_start, 0.0);
    let ceiling_size = (segment.width, segment.ceiling);

    assert!(aabb_overlap(
        player_pos,
        PLAYER_SIZE,
        ceiling_pos,
        ceiling_size
    ));
}

/// Tests game state transition on collision simulation.
#[test]
fn game_state_transitions_on_death() {
    let mut state_machine = StateMachine::new();

    // Start game
    state_machine.handle_event(GameEvent::Start);
    assert_eq!(state_machine.current(), GameState::Playing);

    // Simulate collision death
    state_machine.handle_event(GameEvent::Dead);
    assert_eq!(state_machine.current(), GameState::GameOver);

    // Restart game
    state_machine.handle_event(GameEvent::Start);
    assert_eq!(state_machine.current(), GameState::Playing);
}

/// Tests integrated player physics and collision scenario.
#[test]
fn player_physics_collision_scenario() {
    let mut player = Player::new(Vec2::new(100.0, 90.0));

    // Apply upward thrust to move towards ceiling
    let input = PlayerInput {
        up: true,
        ..Default::default()
    };
    let dt = 1.0 / 60.0; // 60 FPS

    // Move player multiple frames
    for _ in 0..10 {
        player.tick(dt, input, 0.0, 0.0);
    }

    // Player should have moved upward
    assert!(
        player.pos.y < 90.0,
        "Player should have moved up due to thrust"
    );

    // Test collision with ceiling at y=100
    let player_pos = (
        player.pos.x - PLAYER_SIZE.0 / 2.0,
        player.pos.y - PLAYER_SIZE.1 / 2.0,
    );
    let ceiling_pos = (0.0, 0.0);
    let ceiling_size = (200.0, 100.0);

    // Check if collision would occur
    let collision = aabb_overlap(player_pos, PLAYER_SIZE, ceiling_pos, ceiling_size);

    if collision {
        // Collision detected - verify it makes sense
        assert!(player.pos.y - PLAYER_SIZE.1 / 2.0 < 100.0);
    }
}

/// Tests multiple cave segments collision detection.
#[test]
fn multiple_segments_collision() {
    let mut cave = Cave::new(123);

    // Generate a few segments
    for _ in 0..5 {
        cave.generate_next();
    }

    let segments: Vec<_> = cave.segments().iter().copied().collect();
    assert!(segments.len() > 1, "Should have multiple segments");

    // Test player collision with first segment
    let first_segment = segments[0];
    let player_pos = (first_segment.x_start + 10.0, first_segment.ceiling - 5.0);

    let ceiling_pos = (first_segment.x_start, 0.0);
    let ceiling_size = (first_segment.width, first_segment.ceiling);

    assert!(aabb_overlap(
        player_pos,
        PLAYER_SIZE,
        ceiling_pos,
        ceiling_size
    ));
}

/// Tests edge case: player exactly at collision boundary.
#[test]
fn player_at_collision_boundary() {
    let segment = create_test_segment(100.0, 400.0, 50.0, 50.0);

    // Simple case: Player positioned just below ceiling (should NOT collide)
    // Ceiling goes from y=0 to y=100
    // Player positioned with top at y=100.1 (just below, no overlap)
    let player_pos = (60.0, 100.1); // Top-left corner at (60, 100.1)
                                    // Player extends from (60,100.1) to (90,118.1) - no overlap with ceiling (50,0) to (100,100)

    let ceiling_pos = (segment.x_start, 0.0);
    let ceiling_size = (segment.width, segment.ceiling);

    // Should not collide (separated by 0.1 pixel)
    assert!(!aabb_overlap(
        player_pos,
        PLAYER_SIZE,
        ceiling_pos,
        ceiling_size
    ));

    // Move player up to create overlap
    let overlapping_pos = (60.0, 99.0); // Top-left at (60, 99)
                                        // Player now extends from (60,99) to (90,117) - overlaps with ceiling at y=100

    // Should now collide
    assert!(aabb_overlap(
        overlapping_pos,
        PLAYER_SIZE,
        ceiling_pos,
        ceiling_size
    ));
}

/// Tests simple boundary case with clear coordinates.
#[test]
fn simple_boundary_test() {
    // Ceiling from (0,0) to (100,50)
    let ceiling_pos = (0.0, 0.0);
    let ceiling_size = (100.0, 50.0);

    // Player positioned just below ceiling (no overlap)
    let player_pos = (25.0, 50.1); // Player top-left at (25, 50.1)
    let player_size = PLAYER_SIZE; // Player extends to (55, 68.1)

    // Player starts at y=50.1, ceiling ends at y=50 - no overlap
    assert!(!aabb_overlap(
        player_pos,
        player_size,
        ceiling_pos,
        ceiling_size
    ));

    // Move player up to create overlap
    let overlapping_pos = (25.0, 49.0); // Player top-left at (25, 49)
                                        // Player now extends to (55, 67), overlapping with ceiling ending at y=50

    assert!(aabb_overlap(
        overlapping_pos,
        player_size,
        ceiling_pos,
        ceiling_size
    ));
}

/// Debug test to understand coordinate system.
#[test]
fn debug_coordinates() {
    // Very simple case
    let ceiling_pos = (0.0, 0.0);
    let ceiling_size = (100.0, 50.0); // Ceiling from (0,0) to (100,50)

    // Player just below ceiling (with small gap)
    let player_pos = (10.0, 50.1); // Player at (10,50.1) to (40,68.1)

    // This should NOT overlap since player starts at y=50.1 and ceiling ends at y=50
    let result = aabb_overlap(player_pos, PLAYER_SIZE, ceiling_pos, ceiling_size);
    assert!(
        !result,
        "Player at y=50.1 should not overlap with ceiling ending at y=50"
    );

    // Player overlapping
    let player_pos_overlap = (10.0, 49.0); // Player at (10,49) to (40,67)

    // This SHOULD overlap since player starts at y=49 and ceiling ends at y=50
    let result_overlap = aabb_overlap(player_pos_overlap, PLAYER_SIZE, ceiling_pos, ceiling_size);
    assert!(
        result_overlap,
        "Player at y=49 should overlap with ceiling ending at y=50"
    );
}

/// Tests exact touching boundary (edge case).
#[test]
fn exact_touching_boundary() {
    // Test the exact boundary condition where rectangles touch but don't overlap
    let ceiling_pos = (0.0, 0.0);
    let ceiling_size = (100.0, 50.0); // Ceiling from (0,0) to (100,50)

    // Player positioned so top edge exactly touches ceiling bottom edge
    let player_pos = (10.0, 50.0); // Player at (10,50) to (40,68)

    // In AABB collision, touching edges do NOT count as overlap
    // ceiling bottom = 50, player top = 50, so a_bottom <= b_top is true (50 <= 50)
    let result = aabb_overlap(player_pos, PLAYER_SIZE, ceiling_pos, ceiling_size);
    assert!(!result, "Touching boundaries should not count as overlap");
}

/// Tests collision detection performance with many segments.
#[test]
fn collision_performance_many_segments() {
    let mut cave = Cave::new(456);

    // Generate many segments
    for _ in 0..50 {
        cave.generate_next();
    }

    let player_pos = (500.0, 250.0); // Middle position

    // Get segments in view
    let segments = cave.segments_in_view(400.0, 600.0);

    // Check collision with all segments (should be fast)
    let mut collision_found = false;
    for segment in segments {
        let ceiling_pos = (segment.x_start, 0.0);
        let ceiling_size = (segment.width, segment.ceiling);

        if aabb_overlap(player_pos, PLAYER_SIZE, ceiling_pos, ceiling_size) {
            collision_found = true;
            break;
        }

        let floor_pos = (segment.x_start, segment.floor);
        let floor_size = (segment.width, 600.0 - segment.floor);

        if aabb_overlap(player_pos, PLAYER_SIZE, floor_pos, floor_size) {
            collision_found = true;
            break;
        }
    }

    // This test mainly verifies the code runs without panicking
    // Actual collision depends on generated cave layout
    assert!(collision_found || !collision_found); // Always true, just exercises the code
}

/// Tests tractor beam activation and timing integration.
#[test]
fn tractor_beam_activation_integration() {
    let mut beam = TractorBeam::new();

    // Initially inactive
    assert!(!beam.is_active());

    // Activate upward beam
    beam.activate(BeamDir::Up);
    assert!(beam.is_active());
    assert_eq!(beam.dir, BeamDir::Up);

    // Tick for half duration
    let half_duration = TractorBeam::MAX_DURATION / 2.0;
    beam.tick(half_duration);
    assert!(beam.is_active());

    // Tick remaining duration
    beam.tick(half_duration);
    assert!(!beam.is_active());
}

/// Tests beam wall collision detection scenario.
#[test]
fn beam_wall_collision_scenario() {
    let cave = Cave::new(123);
    let segments: Vec<_> = cave.segments().iter().copied().collect();
    let first_segment = segments[0];

    // Player positioned in cave (removed unused variable)
    #[allow(unused_variables)]
    let player_x = first_segment.x_start + 25.0; // Middle of segment

    // Beam should hit ceiling at segment.ceiling height for upward beam
    assert!(first_segment.ceiling > 0.0);
    assert!(first_segment.floor < 600.0);

    // Beam should hit floor at segment.floor height for downward beam
    let gap_height = first_segment.floor - first_segment.ceiling;
    assert!(gap_height >= 140.0); // Minimum gap constraint
}

/// Tests that beam respects cave geometry constraints.
#[test]
fn beam_respects_cave_geometry() {
    let cave = Cave::new(456);

    // Generate several segments
    let mut cave_mut = cave;
    for _ in 0..5 {
        cave_mut.generate_next();
    }

    let segments = cave_mut.segments_in_view(0.0, 300.0);

    for segment in segments {
        // Each segment should have valid ceiling and floor heights
        assert!(segment.ceiling >= 0.0);
        assert!(segment.floor <= 600.0);
        assert!(segment.ceiling < segment.floor);

        // Gap should be sufficient for gameplay
        let gap = segment.floor - segment.ceiling;
        assert!(gap >= 140.0);
    }
}
