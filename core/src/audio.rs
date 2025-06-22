// core/src/audio.rs
/// Audio event system for the Fuel Drift game.
///
/// Manages audio triggers and events without graphics dependencies,
/// following the Single Responsibility Principle.

/// Audio events that can be triggered during gameplay.
///
/// Each event represents a single audio cue that should be played.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AudioEvent {
    /// Thruster firing sound (loopable)
    ThrusterLoop,
    /// Tractor beam activation sound
    BeamActivation,
    /// Fuel pickup/refill sound
    FuelPickup,
    /// Player death/crash sound
    Death,
    /// UI button click sound
    ButtonClick,
}

/// Audio state tracker for managing looping sounds.
///
/// Tracks which sounds are currently playing to avoid audio overlap.
#[derive(Debug, Clone, Copy, Default)]
pub struct AudioState {
    pub thruster_playing: bool,
}

impl AudioState {
    /// Creates a new audio state with all sounds stopped.
    pub fn new() -> Self {
        Self {
            thruster_playing: false,
        }
    }

    /// Updates thruster sound state based on input.
    ///
    /// Returns true if thruster state changed (for triggering audio).
    pub fn update_thruster(&mut self, should_play: bool) -> bool {
        let changed = self.thruster_playing != should_play;
        self.thruster_playing = should_play;
        changed
    }

    /// Stops all looping sounds.
    pub fn stop_all(&mut self) {
        self.thruster_playing = false;
    }
}

/// Audio event queue for collecting events during a frame.
///
/// Allows multiple systems to queue audio events that are played together.
#[derive(Debug, Default)]
pub struct AudioEventQueue {
    events: Vec<AudioEvent>,
}

impl AudioEventQueue {
    /// Creates a new empty audio event queue.
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Adds an audio event to the queue.
    pub fn push(&mut self, event: AudioEvent) {
        self.events.push(event);
    }

    /// Consumes and returns all queued events.
    pub fn drain(&mut self) -> Vec<AudioEvent> {
        std::mem::take(&mut self.events)
    }

    /// Checks if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Gets the number of queued events.
    pub fn len(&self) -> usize {
        self.events.len()
    }
}
