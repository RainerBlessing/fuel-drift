use core::fuel::Fuel;

const EPSILON: f32 = 0.001;
const DT: f32 = 1.0 / 60.0; // 60 FPS

/// Helper function to assert floating point equality.
fn assert_float_eq(a: f32, b: f32) {
    assert!((a - b).abs() < EPSILON, "Expected {}, got {}", b, a);
}

/// Tests fuel creation with correct initial values.
#[test]
fn fuel_creation() {
    let fuel = Fuel::new(100.0, 5.0);

    assert_float_eq(fuel.current, 100.0);
    assert_float_eq(fuel.max, 100.0);
    assert_float_eq(fuel.burn_rate, 5.0);
    assert!(!fuel.is_empty());
    assert_float_eq(fuel.ratio(), 1.0);
}

/// Tests that no fuel burns when consuming is false.
#[test]
fn no_burn_when_not_consuming() {
    let mut fuel = Fuel::new(100.0, 10.0);
    let initial_fuel = fuel.current;

    let became_empty = fuel.burn(DT, false);

    assert_float_eq(fuel.current, initial_fuel);
    assert!(!became_empty);
}

/// Tests fuel consumption when consuming is true.
#[test]
fn burns_fuel_when_consuming() {
    let mut fuel = Fuel::new(100.0, 60.0); // Burns 1 unit per frame at 60 FPS

    let became_empty = fuel.burn(DT, true);

    assert_float_eq(fuel.current, 99.0);
    assert!(!became_empty);
}

/// Tests that fuel cannot go below zero.
#[test]
fn fuel_cannot_go_negative() {
    let mut fuel = Fuel::new(10.0, 600.0); // Burns 10 units per frame

    let became_empty = fuel.burn(DT, true);

    assert_float_eq(fuel.current, 0.0);
    assert!(became_empty);
    assert!(fuel.is_empty());
}

/// Tests that burn returns true exactly when fuel becomes empty.
#[test]
fn burn_returns_true_when_becoming_empty() {
    let mut fuel = Fuel::new(1.0, 60.0); // Will become empty in one frame

    // First burn - becomes empty
    let became_empty = fuel.burn(DT, true);
    assert!(became_empty);
    assert!(fuel.is_empty());

    // Second burn - already empty, should return false
    let became_empty_again = fuel.burn(DT, true);
    assert!(!became_empty_again);
}

/// Tests fuel refilling functionality.
#[test]
fn fuel_refill_basic() {
    let mut fuel = Fuel::new(100.0, 5.0);
    fuel.current = 50.0;

    fuel.refill(20.0);

    assert_float_eq(fuel.current, 70.0);
}

/// Tests that refill cannot exceed maximum.
#[test]
fn refill_cannot_exceed_max() {
    let mut fuel = Fuel::new(100.0, 5.0);
    fuel.current = 90.0;

    fuel.refill(20.0); // Would go to 110, but should cap at 100

    assert_float_eq(fuel.current, 100.0);
}

/// Tests refilling completely empty fuel.
#[test]
fn refill_empty_fuel() {
    let mut fuel = Fuel::new(100.0, 5.0);
    fuel.current = 0.0;

    fuel.refill(30.0);

    assert_float_eq(fuel.current, 30.0);
    assert!(!fuel.is_empty());
}

/// Tests fuel ratio calculation.
#[test]
fn fuel_ratio_calculation() {
    let mut fuel = Fuel::new(100.0, 5.0);

    // Full fuel
    assert_float_eq(fuel.ratio(), 1.0);

    // Half fuel
    fuel.current = 50.0;
    assert_float_eq(fuel.ratio(), 0.5);

    // Quarter fuel
    fuel.current = 25.0;
    assert_float_eq(fuel.ratio(), 0.25);

    // Empty fuel
    fuel.current = 0.0;
    assert_float_eq(fuel.ratio(), 0.0);
}

/// Tests edge case with zero maximum fuel.
#[test]
fn zero_max_fuel_edge_case() {
    let fuel = Fuel::new(0.0, 5.0);

    assert_float_eq(fuel.ratio(), 0.0);
    assert!(fuel.is_empty());
}

/// Tests multiple burn cycles.
#[test]
fn multiple_burn_cycles() {
    let mut fuel = Fuel::new(10.0, 60.0); // Burns 1 unit per frame

    // Burn for 5 frames
    for _ in 0..5 {
        let became_empty = fuel.burn(DT, true);
        assert!(!became_empty);
    }

    assert_float_eq(fuel.current, 5.0);

    // Burn until empty
    for i in 0..5 {
        let became_empty = fuel.burn(DT, true);
        if i == 4 { // Last frame
            assert!(became_empty);
        } else {
            assert!(!became_empty);
        }
    }

    assert!(fuel.is_empty());
}

/// Tests mixed consumption and non-consumption cycles.
#[test]
fn mixed_consumption_cycles() {
    let mut fuel = Fuel::new(10.0, 60.0);

    // Consume, skip, consume, skip pattern
    fuel.burn(DT, true);  // 9.0
    fuel.burn(DT, false); // 9.0 (no change)
    fuel.burn(DT, true);  // 8.0
    fuel.burn(DT, false); // 8.0 (no change)

    assert_float_eq(fuel.current, 8.0);
}

/// Tests fuel with very small burn rate.
#[test]
fn small_burn_rate() {
    let mut fuel = Fuel::new(10.0, 0.1);

    // Should barely consume any fuel
    fuel.burn(DT, true);

    let expected = 10.0 - (0.1 * DT);
    assert_float_eq(fuel.current, expected);
    assert!(fuel.current > 9.9); // Should still be very close to full
}

/// Tests fuel with very high burn rate.
#[test]
fn high_burn_rate() {
    let mut fuel = Fuel::new(1.0, 3600.0); // Burns 60 units per frame

    let became_empty = fuel.burn(DT, true);

    assert_float_eq(fuel.current, 0.0);
    assert!(became_empty);
}