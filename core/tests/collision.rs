use core::collision::{aabb_overlap, Aabb};

/// Tests basic AABB creation and properties.
#[test]
fn aabb_properties() {
    let aabb = Aabb::new(10.0, 20.0, 30.0, 40.0);

    assert_eq!(aabb.x, 10.0);
    assert_eq!(aabb.y, 20.0);
    assert_eq!(aabb.width, 30.0);
    assert_eq!(aabb.height, 40.0);

    assert_eq!(aabb.left(), 10.0);
    assert_eq!(aabb.top(), 20.0);
    assert_eq!(aabb.right(), 40.0);
    assert_eq!(aabb.bottom(), 60.0);
}

/// Tests non-overlapping rectangles on x-axis.
#[test]
fn non_overlapping_x_axis() {
    // Rectangle A: (0, 0) to (10, 10)
    // Rectangle B: (20, 0) to (30, 10)
    assert!(!aabb_overlap(
        (0.0, 0.0),
        (10.0, 10.0),
        (20.0, 0.0),
        (10.0, 10.0)
    ));

    // Rectangle A: (20, 0) to (30, 10)
    // Rectangle B: (0, 0) to (10, 10)
    assert!(!aabb_overlap(
        (20.0, 0.0),
        (10.0, 10.0),
        (0.0, 0.0),
        (10.0, 10.0)
    ));
}

/// Tests non-overlapping rectangles on y-axis.
#[test]
fn non_overlapping_y_axis() {
    // Rectangle A: (0, 0) to (10, 10)
    // Rectangle B: (0, 20) to (10, 30)
    assert!(!aabb_overlap(
        (0.0, 0.0),
        (10.0, 10.0),
        (0.0, 20.0),
        (10.0, 10.0)
    ));

    // Rectangle A: (0, 20) to (10, 30)
    // Rectangle B: (0, 0) to (10, 10)
    assert!(!aabb_overlap(
        (0.0, 20.0),
        (10.0, 10.0),
        (0.0, 0.0),
        (10.0, 10.0)
    ));
}

/// Tests rectangles that are touching but not overlapping.
#[test]
fn touching_not_overlapping() {
    // Adjacent on x-axis
    assert!(!aabb_overlap(
        (0.0, 0.0),
        (10.0, 10.0),
        (10.0, 0.0),
        (10.0, 10.0)
    ));

    // Adjacent on y-axis
    assert!(!aabb_overlap(
        (0.0, 0.0),
        (10.0, 10.0),
        (0.0, 10.0),
        (10.0, 10.0)
    ));
}

/// Tests clearly overlapping rectangles.
#[test]
fn overlapping_rectangles() {
    // Partial overlap
    assert!(aabb_overlap(
        (0.0, 0.0),
        (10.0, 10.0),
        (5.0, 5.0),
        (10.0, 10.0)
    ));

    // Complete containment - A contains B
    assert!(aabb_overlap(
        (0.0, 0.0),
        (20.0, 20.0),
        (5.0, 5.0),
        (10.0, 10.0)
    ));

    // Complete containment - B contains A
    assert!(aabb_overlap(
        (5.0, 5.0),
        (10.0, 10.0),
        (0.0, 0.0),
        (20.0, 20.0)
    ));
}

/// Tests overlapping rectangles with different orientations.
#[test]
fn overlapping_different_orientations() {
    // T-shaped overlap
    assert!(aabb_overlap(
        (0.0, 5.0),
        (20.0, 5.0), // Horizontal bar
        (5.0, 0.0),
        (5.0, 15.0) // Vertical bar
    ));

    // Corner overlap
    assert!(aabb_overlap(
        (0.0, 0.0),
        (10.0, 10.0),
        (8.0, 8.0),
        (10.0, 10.0)
    ));
}

/// Tests edge cases with zero-sized rectangles.
#[test]
fn zero_sized_rectangles() {
    // Zero width rectangle
    assert!(!aabb_overlap(
        (0.0, 0.0),
        (0.0, 10.0),
        (1.0, 0.0),
        (10.0, 10.0)
    ));

    // Zero height rectangle
    assert!(!aabb_overlap(
        (0.0, 0.0),
        (10.0, 0.0),
        (0.0, 1.0),
        (10.0, 10.0)
    ));

    // Two zero-sized rectangles at same position
    assert!(!aabb_overlap(
        (5.0, 5.0),
        (0.0, 0.0),
        (5.0, 5.0),
        (0.0, 0.0)
    ));
}

/// Tests rectangles with negative coordinates.
#[test]
fn negative_coordinates() {
    // Both rectangles in negative space
    assert!(aabb_overlap(
        (-20.0, -20.0),
        (15.0, 15.0),
        (-10.0, -10.0),
        (15.0, 15.0)
    ));

    // One in negative, one in positive
    assert!(aabb_overlap(
        (-10.0, -10.0),
        (20.0, 20.0),
        (5.0, 5.0),
        (10.0, 10.0)
    ));

    // Non-overlapping in negative space
    assert!(!aabb_overlap(
        (-30.0, -20.0),
        (10.0, 10.0),
        (-10.0, -20.0),
        (10.0, 10.0)
    ));
}

/// Tests floating point precision cases.
#[test]
fn floating_point_precision() {
    // Very small overlap
    assert!(aabb_overlap(
        (0.0, 0.0),
        (10.0, 10.0),
        (9.999, 0.0),
        (10.0, 10.0)
    ));

    // Very small gap
    assert!(!aabb_overlap(
        (0.0, 0.0),
        (10.0, 10.0),
        (10.001, 0.0),
        (10.0, 10.0)
    ));
}
