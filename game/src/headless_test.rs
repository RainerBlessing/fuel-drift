// game/src/headless_test.rs
use core::audio::{AudioEventQueue, AudioState};
use core::cave::Cave;
use core::distance::DistanceTracker;
use core::fuel::Fuel;
/// Headless test runner for CI smoke testing.
///
/// Runs core game logic without graphics for 5 seconds to verify
/// basic functionality and catch runtime panics.
use core::game_state::{GameEvent, StateMachine};
use core::player::{Player, PlayerInput, Vec2};
use core::tractor::{BeamDir, TractorBeam};

/// Headless game world for testing core logic.
struct HeadlessGameWorld {
    state_machine: StateMachine,
    player: Player,
    fuel: Fuel,
    cave: Cave,
    tractor_beam: TractorBeam,
    distance_tracker: DistanceTracker,
    audio_queue: AudioEventQueue,
    audio_state: AudioState,
}

impl HeadlessGameWorld {
    fn new() -> Self {
        Self {
            state_machine: StateMachine::new(),
            player: Player::new(Vec2::new(100.0, 300.0)),
            fuel: Fuel::new(100.0, 20.0),
            cave: Cave::new(42),
            tractor_beam: TractorBeam::new(),
            distance_tracker: DistanceTracker::new(),
            audio_queue: AudioEventQueue::new(),
            audio_state: AudioState::new(),
        }
    }

    fn reset(&mut self) {
        self.player = Player::new(Vec2::new(100.0, 300.0));
        self.fuel = Fuel::new(100.0, 20.0);
        self.tractor_beam = TractorBeam::new();
        self.distance_tracker.reset();
        self.audio_state.stop_all();
    }

    fn update(&mut self, dt: f32) {
        // Start game if in menu
        if self.state_machine.current() == core::game_state::GameState::Menu {
            self.state_machine.handle_event(GameEvent::Start);
            self.reset();
        }

        match self.state_machine.current() {
            core::game_state::GameState::Playing => {
                // Update distance
                self.distance_tracker.update(120.0, dt);

                // Simulate some input
                let input = PlayerInput {
                    up: (dt * 1000.0) as i32 % 120 < 20, // Thrust every 2 seconds
                    down: false,
                    left: (dt * 1000.0) as i32 % 240 < 40, // Move left occasionally
                    right: false,
                    tractor_up: (dt * 1000.0) as i32 % 180 < 10, // Beam occasionally
                    tractor_down: false,
                };

                // Update tractor beam
                if input.tractor_up {
                    self.tractor_beam.activate(BeamDir::Up);
                }
                self.tractor_beam.tick(dt);

                // Update fuel
                let consuming = input.up || input.down || input.left || input.right;
                let fuel_became_empty = self.fuel.burn(dt, consuming);

                if fuel_became_empty {
                    self.state_machine.handle_event(GameEvent::Dead);
                    return;
                }

                // Update player physics
                if !self.fuel.is_empty() {
                    self.player.tick(dt, input, 120.0, 0.0); // Using SCROLL_SPEED constant value, camera_offset = 0
                }

                // Generate cave segments
                let view_end = self.distance_tracker.distance + 800.0;
                self.cave.segments_in_view(0.0, view_end);

                // Update audio state
                if self.audio_state.update_thruster(consuming) {
                    if consuming {
                        self.audio_queue.push(core::audio::AudioEvent::ThrusterLoop);
                    }
                }

                // Clear audio events
                self.audio_queue.drain();
            }
            core::game_state::GameState::GameOver => {
                // Restart after a moment
                self.state_machine.handle_event(GameEvent::Start);
                self.reset();
            }
            _ => {}
        }
    }
}

/// Runs headless test for the specified duration.
pub fn run_headless_test(duration_seconds: f32) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Starting headless test for {:.1} seconds...",
        duration_seconds
    );

    let mut world = HeadlessGameWorld::new();
    let dt = 1.0 / 60.0; // 60 FPS simulation
    let total_frames = (duration_seconds / dt) as u32;

    let start_time = std::time::Instant::now();

    for frame in 0..total_frames {
        world.update(dt);

        // Print progress every second
        if frame % 60 == 0 {
            let elapsed = start_time.elapsed().as_secs_f32();
            let distance = world.distance_tracker.distance_as_int();
            let fuel_ratio = world.fuel.ratio();
            let state = world.state_machine.current();

            println!(
                "Frame {}: {:.1}s elapsed, Distance: {}m, Fuel: {:.0}%, State: {:?}",
                frame,
                elapsed,
                distance,
                fuel_ratio * 100.0,
                state
            );
        }
    }

    let elapsed = start_time.elapsed();
    let final_distance = world.distance_tracker.distance_as_int();

    println!("Headless test completed successfully!");
    println!("Elapsed time: {:.2}s", elapsed.as_secs_f32());
    println!("Final distance: {}m", final_distance);
    println!("Final fuel: {:.1}%", world.fuel.ratio() * 100.0);

    Ok(())
}
