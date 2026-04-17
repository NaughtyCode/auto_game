//! Simulation environment that integrates server and client

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use crate::state::ClientCommand;
use crate::network::{NetworkSimulator, Message};
use crate::server::Server;
use crate::client::ShadowFollower;

/// Simulation environment that integrates server, client, and network
pub struct Simulation {
    server: Server,
    client: ShadowFollower,
    network_to_client: Arc<Mutex<NetworkSimulator>>,
    network_to_server: Arc<Mutex<NetworkSimulator>>,
    current_time: f64,
    pending_commands: VecDeque<ClientCommand>,
}

impl Simulation {
    /// Create a new simulation
    pub fn new(
        server: Server,
        client: ShadowFollower,
        network_to_client: Arc<Mutex<NetworkSimulator>>,
        network_to_server: Arc<Mutex<NetworkSimulator>>,
    ) -> Self {
        Self {
            server,
            client,
            network_to_client,
            network_to_server,
            current_time: 0.0,
            pending_commands: VecDeque::new(),
        }
    }

    /// Queue a command from client to server
    pub fn send_command(&mut self, command: ClientCommand) {
        self.pending_commands.push_back(command);
    }

    /// Execute one simulation step
    pub fn step(&mut self, dt: f64) {
        // 1. Send pending commands through network
        let mut network = self.network_to_server.lock().unwrap();
        while let Some(cmd) = self.pending_commands.pop_front() {
            network.send(self.current_time, Message::ClientCommand(cmd));
        }
        drop(network);

        // 2. Server receives and processes commands
        let mut network = self.network_to_server.lock().unwrap();
        let messages = network.receive(self.current_time);
        for msg in messages {
            if let Message::ClientCommand(cmd) = msg {
                self.server.apply_command(cmd);
            }
        }
        drop(network);

        // 3. Update server physics
        self.server.update_physics(dt, self.current_time);

        // 4. Server broadcasts state
        self.server.try_broadcast(self.current_time);

        // 5. Client receives state broadcasts
        let mut network = self.network_to_client.lock().unwrap();
        let messages = network.receive(self.current_time);
        for msg in messages {
            if let Message::StateBroadcast(state) = msg {
                self.client.update_shadow(state);
            }
        }
        drop(network);

        // 6. Client updates entity position
        self.client.update_entity(dt, self.current_time);

        // 7. Advance simulation time
        self.current_time += dt;
    }

    /// Print current status for monitoring
    pub fn print_status(&self) {
        let (entity_x, entity_y) = self.client.get_entity_position();
        let (shadow_x, shadow_y) = self.client.get_shadow_position();
        let server_state = self.server.get_state();

        let diff = ((entity_x - server_state.x).powi(2) + (entity_y - server_state.y).powi(2)).sqrt();

        println!(
            "t={:.3} | Server: ({:.2},{:.2}) | Shadow: ({:.2},{:.2}) | Entity: ({:.2},{:.2}) | Diff: {:.3}",
            self.current_time,
            server_state.x, server_state.y,
            shadow_x, shadow_y,
            entity_x, entity_y,
            diff
        );
    }

    /// Get the current simulation time
    pub fn current_time(&self) -> f64 {
        self.current_time
    }

    /// Get the server instance (mutable)
    pub fn server_mut(&mut self) -> &mut Server {
        &mut self.server
    }

    /// Get the client instance (mutable)
    pub fn client_mut(&mut self) -> &mut ShadowFollower {
        &mut self.client
    }
}