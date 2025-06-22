/// 2D axis-aligned bounding box for collision detection.
///
/// Simple rectangle representation following the principle of least surprise.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Aabb {
    /// Creates a new AABB.
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Gets the right edge x-coordinate.
    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    /// Gets the bottom edge y-coordinate.
    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    /// Gets the left edge x-coordinate.
    pub fn left(&self) -> f32 {
        self.x
    }

    /// Gets the top edge y-coordinate.
    pub fn top(&self) -> f32 {
        self.y
    }
}

/// Checks if two axis-aligned bounding boxes overlap.
///
/// Uses the separating axis theorem for AABB collision detection.
/// Returns true if the rectangles overlap, false otherwise.
///
/// # Arguments
/// * `a_pos` - Position (x, y) of the first rectangle
/// * `a_size` - Size (width, height) of the first rectangle  
/// * `b_pos` - Position (x, y) of the second rectangle
/// * `b_size` - Size (width, height) of the second rectangle
///
/// # Examples
/// ```
/// use core::collision::aabb_overlap;
///
/// // Non-overlapping rectangles
/// assert!(!aabb_overlap((0.0, 0.0), (10.0, 10.0), (20.0, 0.0), (10.0, 10.0)));
///
/// // Overlapping rectangles
/// assert!(aabb_overlap((0.0, 0.0), (10.0, 10.0), (5.0, 5.0), (10.0, 10.0)));
/// ```
pub fn aabb_overlap(
    a_pos: (f32, f32),
    a_size: (f32, f32),
    b_pos: (f32, f32),
    b_size: (f32, f32),
) -> bool {
    let a = Aabb::new(a_pos.0, a_pos.1, a_size.0, a_size.1);
    let b = Aabb::new(b_pos.0, b_pos.1, b_size.0, b_size.1);

    check_aabb_overlap(&a, &b)
}

/// Checks if two AABB structs overlap.
///
/// Internal helper function with low cyclomatic complexity.
/// Uses early return for non-overlapping cases.
fn check_aabb_overlap(a: &Aabb, b: &Aabb) -> bool {
    // Check for separation on x-axis
    if a.right() <= b.left() || b.right() <= a.left() {
        return false;
    }

    // Check for separation on y-axis
    if a.bottom() <= b.top() || b.bottom() <= a.top() {
        return false;
    }

    // No separation found, boxes must overlap
    true
}
