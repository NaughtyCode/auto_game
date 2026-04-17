//! Client-side shadow following logic

use std::collections::VecDeque;
use crate::state::RoleState;

/// Client that follows server state using shadow following algorithm
pub struct ShadowFollower {
    shadow_state: RoleState,
    entity_state: RoleState,
    state_history: VecDeque<RoleState>,
    chase_speed: f64,
    max_chase_distance: f64,
}

impl ShadowFollower {
    /// Create a new shadow follower
    pub fn new(initial_state: RoleState, chase_speed: f64, max_chase_distance: f64) -> Self {
        Self {
            shadow_state: initial_state,
            entity_state: initial_state,
            state_history: VecDeque::with_capacity(4),
            chase_speed,
            max_chase_distance,
        }
    }

    /// Update the shadow state with new authoritative data from server
    pub fn update_shadow(&mut self, new_shadow: RoleState) {
        // Insert state in chronological order
        let pos = self.state_history.iter()
            .position(|state| state.timestamp > new_shadow.timestamp)
            .unwrap_or(self.state_history.len());
        self.state_history.insert(pos, new_shadow);
        self.shadow_state = new_shadow;

        // Keep history size manageable
        if self.state_history.len() > 4 {
            self.state_history.pop_front();
        }
    }

    /// Get the interpolated/extrapolated shadow state for the current time
    pub fn get_current_shadow(&self, current_time: f64) -> RoleState {
        if self.state_history.len() < 2 {
            // Not enough data for interpolation, use extrapolation
            let dt = current_time - self.shadow_state.timestamp;
            if dt > 0.0 {
                return self.shadow_state.extrapolate(dt);
            }
            return self.shadow_state;
        }

        // Find states bracketing the current time for interpolation
        let mut prev = None;
        let mut next = None;

        for state in &self.state_history {
            if state.timestamp <= current_time {
                prev = Some(state);
            } else if next.is_none() {
                next = Some(state);
            }
        }

        match (prev, next) {
            (Some(p), Some(n)) => {
                // Linear interpolation between the two states
                if (n.timestamp - p.timestamp).abs() < 1e-10 {
                    // Timestamps are effectively equal, return the newer state
                    RoleState {
                        x: n.x,
                        y: n.y,
                        vx: n.vx,
                        vy: n.vy,
                        angle: n.angle,
                        timestamp: current_time,
                    }
                } else {
                    let t = (current_time - p.timestamp) / (n.timestamp - p.timestamp);
                    let t = t.clamp(0.0, 1.0);
                    RoleState {
                        x: p.x + (n.x - p.x) * t,
                        y: p.y + (n.y - p.y) * t,
                        vx: p.vx + (n.vx - p.vx) * t,
                        vy: p.vy + (n.vy - p.vy) * t,
                        angle: p.angle + (n.angle - p.angle) * t,
                        timestamp: current_time,
                    }
                }
            }
            (Some(p), None) => {
                // Only historical data, extrapolate
                p.extrapolate(current_time - p.timestamp)
            }
            _ => self.shadow_state,
        }
    }

    /// Update the entity position to chase the shadow
    pub fn update_entity(&mut self, dt: f64, current_time: f64) {
        let target = self.get_current_shadow(current_time);

        // Calculate chase vector
        let dx = target.x - self.entity_state.x;
        let dy = target.y - self.entity_state.y;
        let distance = (dx * dx + dy * dy).sqrt();

        // If already very close, just snap to target
        if distance < 0.001 {
            self.entity_state = target;
            return;
        }

        // Proportional control chase with limits
        let max_step = self.max_chase_distance * dt;
        let step_distance = (self.chase_speed * distance).min(max_step).min(distance);
        let ratio = step_distance / distance;

        // Update entity position
        self.entity_state.x += dx * ratio;
        self.entity_state.y += dy * ratio;

        // Smooth velocity and angle transitions
        self.entity_state.vx = self.entity_state.vx * 0.9 + target.vx * 0.1;
        self.entity_state.vy = self.entity_state.vy * 0.9 + target.vy * 0.1;
        self.entity_state.angle = self.entity_state.angle * 0.9 + target.angle * 0.1;
        self.entity_state.timestamp = current_time;
    }

    /// Get the current entity (rendered) position
    pub fn get_entity_position(&self) -> (f64, f64) {
        (self.entity_state.x, self.entity_state.y)
    }

    /// Get the current shadow (authoritative) position
    pub fn get_shadow_position(&self) -> (f64, f64) {
        (self.shadow_state.x, self.shadow_state.y)
    }

    /// Get the entity state for debugging
    pub fn get_entity_state(&self) -> RoleState {
        self.entity_state
    }

    /// Get the shadow state for debugging
    pub fn get_shadow_state(&self) -> RoleState {
        self.shadow_state
    }

    /// Get the chase speed parameter
    pub fn chase_speed(&self) -> f64 {
        self.chase_speed
    }

    /// Get the max chase distance parameter
    pub fn max_chase_distance(&self) -> f64 {
        self.max_chase_distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadow_follower_update() {
        let initial = RoleState::new(0.0, 0.0, 2.0, 1.0, 0.0, 0.0);
        let mut follower = ShadowFollower::new(initial, 0.3, 10.0);

        // Update with new shadow state
        let new_shadow = RoleState::new(10.0, 5.0, 2.0, 1.0, 0.5, 0.5);
        follower.update_shadow(new_shadow);

        assert_eq!(follower.get_shadow_state().x, 10.0);
        assert_eq!(follower.get_shadow_state().y, 5.0);
        assert_eq!(follower.get_shadow_state().timestamp, 0.5);
    }

    #[test]
    fn test_shadow_interpolation() {
        let initial = RoleState::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let mut follower = ShadowFollower::new(initial, 0.3, 10.0);

        // Add two states for interpolation
        follower.update_shadow(RoleState::new(0.0, 0.0, 2.0, 0.0, 0.0, 0.0));
        follower.update_shadow(RoleState::new(2.0, 0.0, 2.0, 0.0, 0.0, 1.0));

        // Get interpolated state at 0.5 seconds
        let interpolated = follower.get_current_shadow(0.5);
        assert_eq!(interpolated.x, 1.0); // Halfway between 0 and 2
        assert_eq!(interpolated.timestamp, 0.5);
    }

    #[test]
    fn test_entity_chasing() {
        let initial = RoleState::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let mut follower = ShadowFollower::new(initial, 1.0, 10.0); // chase_speed = 1.0

        // Set shadow far away
        follower.update_shadow(RoleState::new(10.0, 0.0, 0.0, 0.0, 0.0, 1.0));

        // Update entity for 0.1 seconds
        follower.update_entity(0.1, 1.1);

        // Entity should move toward shadow
        let (entity_x, _) = follower.get_entity_position();
        assert!(entity_x > 0.0 && entity_x < 10.0);
    }

    #[test]
    fn test_history_ordering() {
        let initial = RoleState::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let mut follower = ShadowFollower::new(initial, 0.3, 10.0);

        // Add states out of order
        follower.update_shadow(RoleState::new(3.0, 0.0, 0.0, 0.0, 0.0, 0.3));
        follower.update_shadow(RoleState::new(1.0, 0.0, 0.0, 0.0, 0.0, 0.1));
        follower.update_shadow(RoleState::new(2.0, 0.0, 0.0, 0.0, 0.0, 0.2));

        // History should be in chronological order
        let shadow = follower.get_current_shadow(0.15);
        assert_eq!(shadow.x, 1.5); // Interpolated between 1.0 and 2.0
    }
}