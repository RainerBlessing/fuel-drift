use core::cave::{Cave, CaveConstants, CaveSegment, SimpleRng};

const EPSILON: f32 = 0.001;

/// Helper function to assert floating point equality.
fn assert_float_eq(a: f32, b: f32) {
    assert!((a - b).abs() < EPSILON, "Expected {}, got {}", b, a);
}

/// Tests that SimpleRng produces deterministic sequences.
#[test]
fn simple_rng_deterministic() {
    let mut rng1 = SimpleRng::new(42);
    let mut rng2 = SimpleRng::new(42);

    for _ in 0..10 {
        assert_float_eq(rng1.next_f32(), rng2.next_f32());
    }
}

/// Tests that SimpleRng range function works correctly.
#[test]
fn simple_rng_range() {
    let mut rng = SimpleRng::new(123);

    for _ in 0..100 {
        let value = rng.range(10.0, 20.0);
        assert!(
            value >= 10.0 && value < 20.0,
            "Value {} not in range [10, 20)",
            value
        );
    }
}

/// Tests CaveSegment basic functionality.
#[test]
fn cave_segment_properties() {
    let segment = CaveSegment::new(100.0, 250.0, 50.0, 25.0);

    assert_float_eq(segment.ceiling, 100.0);
    assert_float_eq(segment.floor, 250.0);
    assert_float_eq(segment.x_start, 50.0);
    assert_float_eq(segment.width, 25.0);
    assert_float_eq(segment.x_end(), 75.0);
    assert_float_eq(segment.gap_height(), 150.0);
}

/// Tests that cave generator never violates minimum gap requirement.
#[test]
fn generator_maintains_minimum_gap() {
    let mut cave = Cave::new(12345);

    // Generate many segments and check gap constraint
    for _ in 0..50 {
        cave.generate_next(300.0);
    }

    for segment in cave.segments() {
        let gap = segment.gap_height();
        assert!(
            gap >= CaveConstants::MIN_GAP,
            "Gap {} is less than minimum {}",
            gap,
            CaveConstants::MIN_GAP
        );
    }
}

/// Tests that segments are contiguous (no gaps or overlaps).
#[test]
fn segments_are_contiguous() {
    let mut cave = Cave::new(54321);

    // Generate several segments
    for _ in 0..10 {
        cave.generate_next(300.0);
    }

    let segments: Vec<_> = cave.segments().iter().collect();

    // Check that each segment starts where the previous one ends
    for i in 1..segments.len() {
        let prev_end = segments[i - 1].x_end();
        let current_start = segments[i].x_start;

        assert_float_eq(current_start, prev_end);
    }
}

/// Tests the initial cave setup.
#[test]
fn initial_cave_setup() {
    let cave = Cave::new(999);

    assert_eq!(cave.segments().len(), 1);

    let first_segment = &cave.segments()[0];
    assert_float_eq(first_segment.ceiling, CaveConstants::INITIAL_CEILING);
    assert_float_eq(first_segment.floor, CaveConstants::INITIAL_FLOOR);
    assert_float_eq(first_segment.x_start, 0.0);
    assert_float_eq(first_segment.width, CaveConstants::SEGMENT_WIDTH);
}

/// Tests segments_in_view functionality.
#[test]
fn segments_in_view_generates_as_needed() {
    let mut cave = Cave::new(777);

    // Initially should have only one segment
    let initial_count = cave.segments().len();
    assert_eq!(initial_count, 1);

    // Request view that extends beyond current segments
    let segments = cave.segments_in_view(0.0, 500.0, 300.0);

    // Check after the method completes
    let final_count = cave.segments().len();

    // Should have generated more segments
    assert!(final_count > initial_count);
    assert!(segments.len() > 1);
}

/// Tests that segments_in_view filters correctly.
#[test]
fn segments_in_view_filters_correctly() {
    let mut cave = Cave::new(888);

    // Generate several segments
    for _ in 0..10 {
        cave.generate_next(300.0);
    }

    // Request a specific view range
    let view_start = 100.0;
    let view_end = 200.0;
    let segments = cave.segments_in_view(view_start, view_end, 300.0);

    // All returned segments should intersect with the view range
    for segment in segments {
        let intersects = segment.x_start < view_end && segment.x_end() > view_start;
        assert!(
            intersects,
            "Segment {:?} doesn't intersect view range [{}, {}]",
            segment, view_start, view_end
        );
    }
}

/// Tests height variation stays within reasonable bounds.
#[test]
fn height_variation_is_reasonable() {
    let mut cave = Cave::new(456);

    let initial_ceiling = cave.segments()[0].ceiling;
    let initial_floor = cave.segments()[0].floor;

    // Generate many segments
    for _ in 0..20 {
        cave.generate_next(300.0);
    }

    // Check that heights don't drift too far from initial values
    for segment in cave.segments() {
        let max_total_drift = CaveConstants::MAX_HEIGHT_CHANGE * 20.0; // Conservative estimate

        assert!(
            (segment.ceiling - initial_ceiling).abs() < max_total_drift * 2.0,
            "Ceiling drifted too far: {} vs initial {}",
            segment.ceiling,
            initial_ceiling
        );

        assert!(
            (segment.floor - initial_floor).abs() < max_total_drift * 2.0,
            "Floor drifted too far: {} vs initial {}",
            segment.floor,
            initial_floor
        );
    }
}

/// Tests cave memory management (segment removal).
#[test]
fn cave_limits_segment_count() {
    let mut cave = Cave::new(321);

    // Generate many more segments than the max
    for _ in 0..150 {
        cave.generate_next(300.0);
    }

    // Should not exceed reasonable memory usage
    assert!(
        cave.segments().len() <= 100,
        "Too many segments retained: {}",
        cave.segments().len()
    );
}
