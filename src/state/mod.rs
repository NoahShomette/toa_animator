use std::time::Duration;

use bevy::{prelude::Component, reflect::Reflect};

use crate::{
    anim::{Animation, AnimationMode},
    FRAME_DURATION,
};

pub mod anim_state;
pub mod state_machine;

/// Animation state
#[derive(Default, Clone, Component, Reflect)]
pub struct AnimationState {
    animation_frame_index: usize,
    sprite_frame_index: usize,
    elapsed_in_frame: Duration,
    /// Control ping_pong backward frame navigation.
    going_backward: bool,
    is_ended: bool,
}

impl AnimationState {
    /// Create a new state
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset animation state
    ///
    /// The animation will restart from the first frame, like if the animation was freshly spawned.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Returns the current frame index
    #[must_use]
    pub fn frame_index(&self) -> usize {
        self.sprite_frame_index
    }

    /// Returns true if the animation has ended
    #[must_use]
    pub fn is_ended(&self) -> bool {
        self.is_ended
    }

    #[must_use]
    fn frame<'a>(&self, animation: &'a Animation) -> &'a usize {
        match &animation.frames {
            Some(frames) => &frames[self.animation_frame_index % frames.len()],
            None => &0,
        }
    }

    /// Update the animation state
    #[allow(dead_code)]
    pub fn update(&mut self, animation: &Animation, delta: Duration) {
        self.elapsed_in_frame += delta;
        if !animation.has_frames() {
            return;
        }
        let mut frame = self.frame(animation);
        self.sprite_frame_index = *frame;

        let frame_duration = match animation.mode {
            AnimationMode::ActionReady { frame_duration } => frame_duration,
            _ => FRAME_DURATION,
        };

        let ending_frame = animation.frames.as_ref().unwrap().len() - 1;

        while self.elapsed_in_frame >= frame_duration {
            let on_last_frame = self.animation_frame_index >= ending_frame;
            match animation.mode {
                AnimationMode::RepeatFrom(loop_from) => {
                    if on_last_frame {
                        self.animation_frame_index = loop_from;
                    } else {
                        self.animation_frame_index += 1;
                    }
                }
                AnimationMode::Repeat => {
                    if on_last_frame {
                        self.animation_frame_index = 0;
                    } else {
                        self.animation_frame_index += 1;
                    }
                }
                AnimationMode::PingPong => {
                    if self.going_backward {
                        if self.animation_frame_index == 0 {
                            self.going_backward = false;
                            self.animation_frame_index += 1;
                        } else {
                            self.animation_frame_index -= 1;
                        }
                    } else if on_last_frame {
                        self.going_backward = true;
                        self.animation_frame_index -= 1;
                    } else {
                        self.animation_frame_index += 1;
                    }
                }
                AnimationMode::Once | AnimationMode::ActionReady { frame_duration: _ } => {
                    if on_last_frame {
                        self.is_ended = true;
                    } else {
                        self.animation_frame_index += 1;
                    }
                }
                AnimationMode::Action => {}
            }

            self.elapsed_in_frame -= frame_duration;
            frame = self.frame(animation);
            self.sprite_frame_index = *frame;
        }
    }
}
