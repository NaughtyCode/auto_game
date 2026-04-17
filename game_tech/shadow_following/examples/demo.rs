//! Demonstration of the shadow following algorithm

use shadow_following::{Simulation, Server, ShadowFollower, NetworkSimulator, RoleState, ClientCommand};
use std::sync::{Arc, Mutex};

fn main() {
    println!("=== Shadow Following Algorithm Demo ===");
    println!("This demonstrates how the client smoothly follows server state");
    println!("even with network delay and intermittent updates.\n");

    // Create network simulators
    let network_to_client = Arc::new(Mutex::new(NetworkSimulator::new(0.08, 0.0))); // 80ms delay
    let network_to_server = Arc::new(Mutex::new(NetworkSimulator::new(0.02, 0.0))); // 20ms delay

    // Initial state
    let initial_state = RoleState {
        x: 0.0,
        y: 0.0,
        vx: 0.0,
        vy: 0.0,
        angle: 0.0,
        timestamp: 0.0,
    };

    // Create server and client
    let server = Server::new(
        initial_state,
        0.1, // Broadcast every 100ms
        network_to_client.clone(),
    );

    let client = ShadowFollower::new(
        initial_state,
        0.25, // Chase speed coefficient
        5.0,  // Max chase distance per second
    );

    // Create simulation
    let mut sim = Simulation::new(
        server,
        client,
        network_to_client,
        network_to_server,
    );

    // Simulation parameters
    let total_time = 5.0;
    let step_dt = 0.016; // ~60 FPS
    let steps = (total_time / step_dt) as usize;

    println!("Simulation Parameters:");
    println!("  - Server broadcast interval: 100ms");
    println!("  - Network delay (server→client): 80ms");
    println!("  - Network delay (client→server): 20ms");
    println!("  - Chase speed coefficient: 0.25");
    println!("  - Max chase distance: 5.0 units/sec");
    println!("  - Simulation time: {} seconds", total_time);
    println!("  - Step size: {:.3} seconds (~{} FPS)\n", step_dt, (1.0 / step_dt) as u32);

    println!("Key:");
    println!("  Server:   Authoritative position (ground truth)");
    println!("  Shadow:   Client's view of server position (delayed)");
    println!("  Entity:   Rendered position (smoothed via shadow following)");
    println!("  Diff:     Distance between Entity and Server\n");

    // Send initial command to start moving
    sim.send_command(ClientCommand::SetVelocity { vx: 2.0, vy: 1.0 });

    // Track when to send additional commands
    let mut command_sent_at_0_5s = false;
    let mut command_sent_at_1_5s = false;

    // Main simulation loop
    for i in 0..steps {
        let t = i as f64 * step_dt;

        // Send commands at specific times
        if t >= 0.5 && !command_sent_at_0_5s {
            sim.send_command(ClientCommand::SetVelocity { vx: 0.5, vy: -1.2 });
            command_sent_at_0_5s = true;
            println!("\n[Time {:.1}s] Changing velocity to (0.5, -1.2)", t);
        }

        if t >= 1.5 && !command_sent_at_1_5s {
            sim.send_command(ClientCommand::SetVelocity { vx: 0.0, vy: 0.0 });
            command_sent_at_1_5s = true;
            println!("\n[Time {:.1}s] Stopping entity", t);
        }

        // Execute simulation step
        sim.step(step_dt);

        // Print status every 0.2 seconds
        if i % (0.2 / step_dt) as usize == 0 {
            sim.print_status();
        }
    }

    println!("\n=== Simulation Complete ===");
    println!("Observations:");
    println!("1. Entity position smoothly follows Shadow position");
    println!("2. Shadow position lags behind Server due to network delay");
    println!("3. Entity never jumps abruptly, even when Server velocity changes");
    println!("4. When Server stops, Entity gradually catches up");
    println!("\nThe shadow following algorithm successfully provides:");
    println!("  - Smooth visual experience for players");
    println!("  - Tolerance to network latency");
    println!("  - No visual teleportation or snapping");
}