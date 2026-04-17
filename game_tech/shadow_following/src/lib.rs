//! Shadow Following Algorithm Implementation
//!
//! A modular implementation of the shadow following algorithm for game synchronization.
//! This library provides the core components for simulating server-authoritative state
//! synchronization with client-side smooth interpolation.

pub mod state;
pub mod network;
pub mod server;
pub mod client;
pub mod simulation;

// Re-export main types for convenience
pub use state::{RoleState, ClientCommand};
pub use network::NetworkSimulator;
pub use server::Server;
pub use client::ShadowFollower;
pub use simulation::Simulation;