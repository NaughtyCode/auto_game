//! Simple example of the shadow following algorithm

use shadow_following::{RoleState, ShadowFollower};

fn main() {
    println!("=== Simple Shadow Follower Example ===\n");

    // Create initial state
    let initial_state = RoleState::new(0.0, 0.0, 2.0, 1.0, 0.0, 0.0);

    // Create a shadow follower
    let mut follower = ShadowFollower::new(initial_state, 0.3, 10.0);

    println!("Initial state:");
    println!("  Shadow position: ({:.1}, {:.1})", initial_state.x, initial_state.y);
    println!("  Entity position: ({:.1}, {:.1})\n", initial_state.x, initial_state.y);

    // Simulate receiving updates with some delay
    println!("Simulating updates with 100ms intervals:");

    for i in 1..=10 {
        let time = i as f64 * 0.1; // 100ms steps

        // Create a new shadow state (simulating server update)
        let new_shadow = RoleState::new(
            time * 2.0,      // x = 2 * t
            time * 1.0,      // y = 1 * t
            2.0, 1.0, 0.0,   // Same velocity and angle
            time,
        );

        // Update the shadow
        follower.update_shadow(new_shadow);

        // Update entity to chase shadow
        let dt = 0.016; // 60 FPS
        follower.update_entity(dt, time);

        let (entity_x, entity_y) = follower.get_entity_position();
        let (shadow_x, shadow_y) = follower.get_shadow_position();

        println!("t={:.1}s: Shadow=({:.2},{:.2}), Entity=({:.2},{:.2}), Distance={:.3}",
                 time, shadow_x, shadow_y, entity_x, entity_y,
                 ((entity_x - shadow_x).powi(2) + (entity_y - shadow_y).powi(2)).sqrt());
    }

    println!("\n=== Example Complete ===");
    println!("Notice how the Entity position:");
    println!("1. Smoothly follows the Shadow position");
    println!("2. Never jumps abruptly");
    println!("3. Gradually reduces the distance over time");
}