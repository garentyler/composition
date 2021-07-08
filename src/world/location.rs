/// Used to store a point in a world.
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f32,
    pub yaw: f32,
    // TODO: Store a reference to the world this location is in.
}
impl Location {
    /// Create a new `Location`.
    pub fn new(x: f64, y: f64, z: f64, pitch: f32, yaw: f32) -> Location {
        Location {
            x,
            y,
            z,
            pitch,
            yaw,
        }
    }

    /// Create a new `Location` with no rotation.
    pub fn position(x: f64, y: f64, z: f64) -> Location {
        Location {
            x,
            y,
            z,
            pitch: 0.0,
            yaw: 0.0,
        }
    }

    /// Create a new `Location` with no position.
    pub fn rotation(pitch: f32, yaw: f32) -> Location {
        Location {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            pitch,
            yaw,
        }
    }

    /// Create a new `Location` with no rotation or position.
    pub fn zero() -> Location {
        Location {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}
