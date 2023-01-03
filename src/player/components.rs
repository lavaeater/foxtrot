use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerSensor;

#[derive(Component)]
pub struct PlayerModel;

#[derive(Debug, Component, Default, Clone)]
pub struct CharacterVelocity(pub Vect);

#[derive(Component, Default)]
pub struct Grounded {
    pub time_since_last_grounded: Timer,
}

#[derive(Component, Default)]
pub struct Jump {
    pub time_since_start: Timer,
    pub state: JumpState,
}

#[derive(Debug)]
pub enum JumpState {
    InProgress,
    Done,
}
impl Default for JumpState {
    fn default() -> Self {
        Self::Done
    }
}
impl Jump {
    pub fn speed_fraction(&self) -> f32 {
        let t: f32 = self.time_since_start.into();
        // shifted and scaled sigmoid
        let suggestion = 1. / (1. + (40. * (t - 0.1)).exp());
        if suggestion > 0.001 {
            suggestion
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Timer {
    elapsed_time: f32,
}
impl Default for Timer {
    fn default() -> Self {
        Self {
            elapsed_time: f32::MAX,
        }
    }
}

impl From<Timer> for f32 {
    fn from(timer: Timer) -> Self {
        timer.elapsed_time
    }
}

impl Timer {
    pub fn start(&mut self) {
        self.elapsed_time = 0.0
    }
    pub fn update(&mut self, dt: f32) {
        self.elapsed_time = if self.elapsed_time < f32::MAX - dt - 0.1 {
            self.elapsed_time + dt
        } else {
            f32::MAX
        }
    }
}