// core/tests/tractor.rs

use core::tractor::{BeamDir, TractorBeam};

const DT: f32 = 1.0 / 60.0; // 60 FPS
const EPSILON: f32 = 0.001;

/// Helper function to assert floating point equality.
fn assert_float_eq(a: f32, b: f32) {
    assert!((a - b).abs() < EPSILON, "Expected {}, got {}", b, a);
}

/// Tests tractor beam creation with correct initial state.
#[test]
fn tractor_beam_creation() {
    let beam = TractorBeam::new();

    assert!(!beam.active);
    assert_eq!(beam.dir, BeamDir::Up); // Default direction
    assert_float_eq(beam.timer, 0.0);
    assert!(!beam.is_active());
    assert_float_eq(beam.remaining_time(), 0.0);
}

/// Tests default implementation.
#[test]
fn tractor_beam_default() {
    let beam = TractorBeam::default();
    let new_beam = TractorBeam::new();

    assert_eq!(beam.active, new_beam.active);
    assert_eq!(beam.dir, new_beam.dir);
    assert_float_eq(beam.timer, new_beam.timer);
}

/// Tests beam activation with up direction.
#[test]
fn activate_beam_up() {
    let mut beam = TractorBeam::new();

    beam.activate(BeamDir::Up);

    assert!(beam.active);
    assert_eq!(beam.dir, BeamDir::Up);
    assert_float_eq(beam.timer, TractorBeam::MAX_DURATION);
    assert!(beam.is_active());
    assert_float_eq(beam.remaining_time(), TractorBeam::MAX_DURATION);
}

/// Tests beam activation with down direction.
#[test]
fn activate_beam_down() {
    let mut beam = TractorBeam::new();

    beam.activate(BeamDir::Down);

    assert!(beam.active);
    assert_eq!(beam.dir, BeamDir::Down);
    assert_float_eq(beam.timer, TractorBeam::MAX_DURATION);
    assert!(beam.is_active());
}

/// Tests that beam cannot be reactivated while already active.
#[test]
fn cannot_reactivate_while_active() {
    let mut beam = TractorBeam::new();

    // Activate with up direction
    beam.activate(BeamDir::Up);
    let initial_timer = beam.timer;

    // Try to activate with down direction
    beam.activate(BeamDir::Down);

    // Should remain with original activation
    assert!(beam.active);
    assert_eq!(beam.dir, BeamDir::Up); // Unchanged
    assert_float_eq(beam.timer, initial_timer); // Unchanged
}

/// Tests beam timer countdown.
#[test]
fn beam_timer_countdown() {
    let mut beam = TractorBeam::new();
    beam.activate(BeamDir::Up);

    let initial_timer = beam.timer;

    // Tick for one frame
    beam.tick(DT);

    assert!(beam.active);
    assert_float_eq(beam.timer, initial_timer - DT);
    assert_float_eq(beam.remaining_time(), initial_timer - DT);
}

/// Tests beam automatic deactivation when timer reaches zero.
#[test]
fn beam_auto_deactivation() {
    let mut beam = TractorBeam::new();
    beam.activate(BeamDir::Up);

    // Tick for maximum duration to exhaust timer
    beam.tick(TractorBeam::MAX_DURATION);

    assert!(!beam.active);
    assert_float_eq(beam.timer, 0.0);
    assert!(!beam.is_active());
    assert_float_eq(beam.remaining_time(), 0.0);
}

/// Tests beam deactivation with slight over-tick.
#[test]
fn beam_deactivation_with_overtick() {
    let mut beam = TractorBeam::new();
    beam.activate(BeamDir::Down);

    // Tick for more than maximum duration
    beam.tick(TractorBeam::MAX_DURATION + 0.5);

    assert!(!beam.active);
    assert_float_eq(beam.timer, 0.0);
    assert!(!beam.is_active());
}

/// Tests multiple tick cycles until deactivation.
#[test]
fn multiple_tick_cycles() {
    let mut beam = TractorBeam::new();
    beam.activate(BeamDir::Up);

    // Calculate exact number of ticks needed
    // MAX_DURATION = 2.0, DT = 1/60 ≈ 0.0166667
    // Expected ticks = 2.0 / 0.0166667 = 120 ticks
    let expected_ticks = (TractorBeam::MAX_DURATION / DT).ceil() as i32;

    // Track elapsed time manually to ensure precision
    let mut elapsed_time = 0.0;
    let mut tick_count = 0;

    // Tick until we reach or exceed MAX_DURATION
    while elapsed_time < TractorBeam::MAX_DURATION {
        beam.tick(DT);
        elapsed_time += DT;
        tick_count += 1;

        // Safety check to prevent infinite loop
        if tick_count > expected_ticks + 5 {
            panic!("Too many ticks performed: {}", tick_count);
        }
    }

    // After exceeding MAX_DURATION, beam should be deactivated
    assert!(
        !beam.active,
        "Beam should be deactivated after {:.6} seconds ({} ticks, expected ~{})",
        elapsed_time, tick_count, expected_ticks
    );

    // Verify timer is reset
    assert_eq!(beam.timer, 0.0, "Timer should be reset to 0.0");
}

/// Tests tick on inactive beam does nothing.
#[test]
fn tick_inactive_beam_does_nothing() {
    let mut beam = TractorBeam::new();

    // Tick inactive beam
    beam.tick(DT);

    assert!(!beam.active);
    assert_float_eq(beam.timer, 0.0);
}

/// Tests reactivation after automatic deactivation.
#[test]
fn reactivation_after_deactivation() {
    let mut beam = TractorBeam::new();

    // First activation
    beam.activate(BeamDir::Up);
    beam.tick(TractorBeam::MAX_DURATION); // Deactivate

    assert!(!beam.active);

    // Second activation with different direction
    beam.activate(BeamDir::Down);

    assert!(beam.active);
    assert_eq!(beam.dir, BeamDir::Down);
    assert_float_eq(beam.timer, TractorBeam::MAX_DURATION);
}

/// Tests edge case with very small time increments.
#[test]
fn small_time_increments() {
    let mut beam = TractorBeam::new();
    beam.activate(BeamDir::Up);

    let small_dt = 0.001;
    let mut total_time = 0.0;

    while beam.active && total_time < TractorBeam::MAX_DURATION + 0.1 {
        beam.tick(small_dt);
        total_time += small_dt;
    }

    assert!(!beam.active);
    assert!(total_time >= TractorBeam::MAX_DURATION);
}

/// Tests remaining time calculation accuracy.
#[test]
fn remaining_time_accuracy() {
    let mut beam = TractorBeam::new();
    beam.activate(BeamDir::Up);

    // Tick for half duration
    let half_duration = TractorBeam::MAX_DURATION / 2.0;
    beam.tick(half_duration);

    let expected_remaining = TractorBeam::MAX_DURATION - half_duration;
    assert_float_eq(beam.remaining_time(), expected_remaining);
    assert!(beam.active);
}

/// Tests BeamDir enum properties.
#[test]
fn beam_dir_properties() {
    assert_eq!(BeamDir::Up, BeamDir::Up);
    assert_eq!(BeamDir::Down, BeamDir::Down);
    assert_ne!(BeamDir::Up, BeamDir::Down);

    // Test clone and copy
    let dir = BeamDir::Up;
    let dir_clone = dir;
    assert_eq!(dir, dir_clone);
}
