//! State definitions for the shadow following algorithm

/// Client command types sent to the server
#[derive(Debug, Clone, Copy)]
pub enum ClientCommand {
    /// Set the velocity of the entity
    SetVelocity { vx: f64, vy: f64 },
    /// Set the angle (facing direction) of the entity
    SetAngle { angle: f64 },
}

/// Role state containing position, velocity, and timing information
#[derive(Debug, Clone, Copy)]
pub struct RoleState {
    /// X position
    pub x: f64,
    /// Y position
    pub y: f64,
    /// X velocity
    pub vx: f64,
    /// Y velocity
    pub vy: f64,
    /// Facing angle in radians
    pub angle: f64,
    /// Timestamp when this state was created (in seconds)
    pub timestamp: f64,
}

impl RoleState {
    /// Create a new RoleState
    pub fn new(x: f64, y: f64, vx: f64, vy: f64, angle: f64, timestamp: f64) -> Self {
        Self {
            x,
            y,
            vx,
            vy,
            angle,
            timestamp,
        }
    }

    /// Extrapolate position based on velocity and elapsed time (dead reckoning)
    pub fn extrapolate(&self, dt: f64) -> Self {
        Self {
            x: self.x + self.vx * dt,
            y: self.y + self.vy * dt,
            vx: self.vx,
            vy: self.vy,
            angle: self.angle,
            timestamp: self.timestamp + dt,
        }
    }

    /// Calculate distance to another state
    pub fn distance_to(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    /// Calculate squared distance (faster than distance_to for comparisons)
    pub fn distance_squared_to(&self, other: &Self) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
}

impl Default for RoleState {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            angle: 0.0,
            timestamp: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_state_extrapolate() {
        let state = RoleState::new(0.0, 0.0, 2.0, 1.0, 0.5, 0.0);
        let extrapolated = state.extrapolate(1.0);
        assert_eq!(extrapolated.x, 2.0);
        assert_eq!(extrapolated.y, 1.0);
        assert_eq!(extrapolated.timestamp, 1.0);
        assert_eq!(extrapolated.vx, 2.0);
        assert_eq!(extrapolated.vy, 1.0);
        assert_eq!(extrapolated.angle, 0.5);
    }

    #[test]
    fn test_distance_calculation() {
        let state1 = RoleState::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let state2 = RoleState::new(3.0, 4.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(state1.distance_to(&state2), 5.0);
        assert_eq!(state1.distance_squared_to(&state2), 25.0);
    }

    #[test]
    fn test_default_state() {
        let default = RoleState::default();
        assert_eq!(default.x, 0.0);
        assert_eq!(default.y, 0.0);
        assert_eq!(default.vx, 0.0);
        assert_eq!(default.vy, 0.0);
        assert_eq!(default.angle, 0.0);
        assert_eq!(default.timestamp, 0.0);
    }
}