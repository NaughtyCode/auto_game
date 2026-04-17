//! Server-side logic for the shadow following algorithm

use std::sync::{Arc, Mutex};
use crate::state::{RoleState, ClientCommand};
use crate::network::{NetworkSimulator, Message};

/// Server that maintains authoritative state and broadcasts to clients
pub struct Server {
    role_state: RoleState,
    broadcast_interval: f64,
    next_broadcast_time: f64,
    outgoing_network: Arc<Mutex<NetworkSimulator>>,
}

impl Server {
    /// Create a new server with initial state
    pub fn new(
        initial_state: RoleState,
        broadcast_interval: f64,
        outgoing_network: Arc<Mutex<NetworkSimulator>>,
    ) -> Self {
        Self {
            role_state: initial_state,
            broadcast_interval,
            next_broadcast_time: initial_state.timestamp + broadcast_interval,
            outgoing_network,
        }
    }

    /// Update physics using Euler integration
    pub fn update_physics(&mut self, dt: f64, current_time: f64) {
        self.role_state.x += self.role_state.vx * dt;
        self.role_state.y += self.role_state.vy * dt;
        self.role_state.timestamp = current_time;
    }

    /// Apply a client command to change the entity's state
    pub fn apply_command(&mut self, command: ClientCommand) {
        match command {
            ClientCommand::SetVelocity { vx, vy } => {
                self.role_state.vx = vx;
                self.role_state.vy = vy;
            }
            ClientCommand::SetAngle { angle } => {
                self.role_state.angle = angle;
            }
        }
    }

    /// Broadcast current state if it's time to do so
    pub fn try_broadcast(&mut self, current_time: f64) {
        if current_time >= self.next_broadcast_time {
            let mut network = self.outgoing_network.lock().unwrap();
            network.send(current_time, Message::StateBroadcast(self.role_state));
            self.next_broadcast_time += self.broadcast_interval;
        }
    }

    /// Get the current authoritative state
    pub fn get_state(&self) -> RoleState {
        self.role_state
    }

    /// Get the broadcast interval
    pub fn broadcast_interval(&self) -> f64 {
        self.broadcast_interval
    }

    /// Get the next broadcast time
    pub fn next_broadcast_time(&self) -> f64 {
        self.next_broadcast_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_server_update_physics() {
        let network = Arc::new(Mutex::new(NetworkSimulator::new(0.1, 0.0)));
        let mut server = Server::new(RoleState::default(), 0.5, network.clone());

        // Apply velocity command
        server.apply_command(ClientCommand::SetVelocity { vx: 2.0, vy: 1.0 });
        server.update_physics(1.0, 1.0);

        let state = server.get_state();
        assert_eq!(state.x, 2.0);
        assert_eq!(state.y, 1.0);
        assert_eq!(state.vx, 2.0);
        assert_eq!(state.vy, 1.0);
        assert_eq!(state.timestamp, 1.0);
    }

    #[test]
    fn test_server_apply_commands() {
        let network = Arc::new(Mutex::new(NetworkSimulator::new(0.1, 0.0)));
        let mut server = Server::new(RoleState::default(), 0.5, network.clone());

        // Test velocity command
        server.apply_command(ClientCommand::SetVelocity { vx: 3.0, vy: 4.0 });
        let state = server.get_state();
        assert_eq!(state.vx, 3.0);
        assert_eq!(state.vy, 4.0);

        // Test angle command
        server.apply_command(ClientCommand::SetAngle { angle: 1.57 });
        let state = server.get_state();
        assert_eq!(state.angle, 1.57);
    }

    #[test]
    fn test_server_broadcast_timing() {
        let network = Arc::new(Mutex::new(NetworkSimulator::new(0.1, 0.0)));
        let mut server = Server::new(
            RoleState::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            0.2, // broadcast every 200ms
            network.clone(),
        );

        assert_eq!(server.next_broadcast_time(), 0.2);

        // Should broadcast at 0.2s
        server.try_broadcast(0.2);
        let mut net = network.lock().unwrap();
        // Message sent at 0.2s with 0.1s delay arrives at 0.3s
        let messages = net.receive(0.31);
        assert_eq!(messages.len(), 1);
        assert_eq!(server.next_broadcast_time(), 0.4);
    }
}