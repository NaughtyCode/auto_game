//! Network simulation for testing the shadow following algorithm

use std::collections::VecDeque;
use crate::state::{RoleState, ClientCommand};

/// Network message types
#[derive(Debug, Clone)]
pub enum Message {
    /// State broadcast from server to client
    StateBroadcast(RoleState),
    /// Command from client to server
    ClientCommand(ClientCommand),
}

/// Network simulator that models delay and packet loss
pub struct NetworkSimulator {
    delay: f64,
    packet_loss_rate: f64,
    message_queue: VecDeque<(f64, Message)>,
}

impl NetworkSimulator {
    /// Create a new network simulator with given delay and packet loss rate
    pub fn new(delay: f64, packet_loss_rate: f64) -> Self {
        Self {
            delay,
            packet_loss_rate,
            message_queue: VecDeque::new(),
        }
    }

    /// Send a message through the simulated network
    pub fn send(&mut self, current_time: f64, message: Message) {
        // Simulate packet loss
        if rand::random::<f64>() < self.packet_loss_rate {
            return;
        }

        // Add to delay queue
        let arrival_time = current_time + self.delay;
        self.message_queue.push_back((arrival_time, message));
    }

    /// Receive all messages that have arrived by the current time
    pub fn receive(&mut self, current_time: f64) -> Vec<Message> {
        let mut messages = Vec::new();

        while let Some((arrival_time, message)) = self.message_queue.front() {
            if *arrival_time <= current_time {
                messages.push(message.clone());
                self.message_queue.pop_front();
            } else {
                break;
            }
        }

        messages
    }

    /// Check if there are pending messages in the queue
    pub fn has_pending_messages(&self) -> bool {
        !self.message_queue.is_empty()
    }

    /// Get the current delay setting
    pub fn delay(&self) -> f64 {
        self.delay
    }

    /// Get the packet loss rate
    pub fn packet_loss_rate(&self) -> f64 {
        self.packet_loss_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_send_receive() {
        let mut network = NetworkSimulator::new(0.1, 0.0); // 100ms delay, no loss
        let message = Message::StateBroadcast(RoleState::default());

        network.send(0.0, message.clone());

        // Should not receive before delay
        let messages = network.receive(0.05);
        assert!(messages.is_empty());

        // Should receive after delay
        let messages = network.receive(0.11);
        assert_eq!(messages.len(), 1);
        match &messages[0] {
            Message::StateBroadcast(_) => (),
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_packet_loss() {
        // Test with 100% packet loss - should never receive
        let mut network = NetworkSimulator::new(0.1, 1.0);
        let message = Message::StateBroadcast(RoleState::default());

        network.send(0.0, message);

        let messages = network.receive(0.2);
        assert!(messages.is_empty());
    }

    #[test]
    fn test_has_pending_messages() {
        let mut network = NetworkSimulator::new(0.1, 0.0);
        let message = Message::StateBroadcast(RoleState::default());

        network.send(0.0, message);
        assert!(network.has_pending_messages());

        let _ = network.receive(0.11);
        assert!(!network.has_pending_messages());
    }
}