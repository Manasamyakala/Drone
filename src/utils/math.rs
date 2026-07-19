use std::f32::consts::PI;

pub struct MathUtils;

impl MathUtils {
    /// Euclidean distance in 2D
    pub fn distance_2d(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    ) -> f32 {
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }

    /// Euclidean distance in 3D
    pub fn distance_3d(
        x1: f32,
        y1: f32,
        z1: f32,
        x2: f32,
        y2: f32,
        z2: f32,
    ) -> f32 {
        ((x2 - x1).powi(2)
            + (y2 - y1).powi(2)
            + (z2 - z1).powi(2))
            .sqrt()
    }

    /// Calculate speed
    pub fn speed(
        distance: f32,
        time: f32,
    ) -> f32 {
        if time <= 0.0 {
            return 0.0;
        }

        distance / time
    }

    /// Calculate velocity components
    pub fn velocity(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        dt: f32,
    ) -> (f32, f32) {

        if dt <= 0.0 {
            return (0.0, 0.0);
        }

        (
            (x2 - x1) / dt,
            (y2 - y1) / dt,
        )
    }

    /// Magnitude of velocity vector
    pub fn magnitude(
        vx: f32,
        vy: f32,
    ) -> f32 {

        (vx.powi(2) + vy.powi(2)).sqrt()
    }

    /// Heading angle (degrees)
    pub fn heading(
        vx: f32,
        vy: f32,
    ) -> f32 {

        vy.atan2(vx).to_degrees()
    }

    /// Convert degrees to radians
    pub fn deg_to_rad(
        degrees: f32,
    ) -> f32 {

        degrees * PI / 180.0
    }

    /// Convert radians to degrees
    pub fn rad_to_deg(
        radians: f32,
    ) -> f32 {

        radians * 180.0 / PI
    }

    /// Clamp value
    pub fn clamp(
        value: f32,
        min: f32,
        max: f32,
    ) -> f32 {

        value.max(min).min(max)
    }

    /// Normalize value
    pub fn normalize(
        value: f32,
        min: f32,
        max: f32,
    ) -> f32 {

        if (max - min).abs() < f32::EPSILON {
            return 0.0;
        }

        (value - min) / (max - min)
    }

    /// Linear interpolation
    pub fn lerp(
        start: f32,
        end: f32,
        t: f32,
    ) -> f32 {

        start + (end - start) * t
    }

    /// Dot product
    pub fn dot(
        ax: f32,
        ay: f32,
        bx: f32,
        by: f32,
    ) -> f32 {

        ax * bx + ay * by
    }

    /// Angle between vectors (degrees)
    pub fn angle_between(
        ax: f32,
        ay: f32,
        bx: f32,
        by: f32,
    ) -> f32 {

        let dot = Self::dot(ax, ay, bx, by);

        let mag_a = Self::magnitude(ax, ay);

        let mag_b = Self::magnitude(bx, by);

        if mag_a == 0.0 || mag_b == 0.0 {
            return 0.0;
        }

        let cos_theta =
            (dot / (mag_a * mag_b))
                .clamp(-1.0, 1.0);

        cos_theta.acos().to_degrees()
    }

    /// Calculate midpoint
    pub fn midpoint(
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
    ) -> (f32, f32) {

        (
            (x1 + x2) / 2.0,
            (y1 + y2) / 2.0,
        )
    }

    /// Check whether a point is inside a circle
    pub fn inside_radius(
        px: f32,
        py: f32,
        cx: f32,
        cy: f32,
        radius: f32,
    ) -> bool {

        Self::distance_2d(px, py, cx, cy) <= radius
    }
}