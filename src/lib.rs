use std::time::Duration;

use bevy::{
    app::{Plugin, PostUpdate},
    asset::Assets,
    prelude::{Changed, Children, IntoSystemConfigs, IntoSystemSetConfigs, Query, Res, SystemSet},
    sprite::{TextureAtlas, TextureAtlasLayout},
    time::Time,
};

mod anim;
mod state;

pub use {
    anim::{ActiveAnimation, Animation, AnimationMode, Animations, ScopedAction},
    state::AnimationState,
};

pub struct AnimatorPlugin;

impl Plugin for AnimatorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Animation>()
            .register_type::<AnimationState>()
            .register_type::<ActiveAnimation>();

        app.configure_sets(
            PostUpdate,
            (
                AnimationSets::ChangeAnimations,
                AnimationSets::Setup,
                AnimationSets::Animate,
            )
                .chain(),
        );

        app.add_systems(PostUpdate, animate.in_set(AnimationSets::Animate));
        app.add_systems(
            PostUpdate,
            (setup_frames, setup_action_durations)
                .chain()
                .in_set(AnimationSets::Setup),
        );
    }
}

pub const FRAMES_MS: u64 = 83;
pub const FRAME_DURATION: Duration = Duration::from_millis(FRAMES_MS);

#[derive(SystemSet, Hash, Clone, Eq, Debug, PartialEq)]
pub enum AnimationSets {
    ChangeAnimations,
    Setup,
    Animate,
}

fn animate(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut TextureAtlas, &ActiveAnimation)>,
) {
    for (mut player, mut texture, animation) in query.iter_mut() {
        // Update the state
        player.update(animation, time.delta());
        // Update the texture atlas
        texture.index = player.frame_index();
    }
}

/// Sets up all the frames for all animations. This goes through and uses the texture atlas layout to correctly list all the frames for an animation without requiring it to manually happen
fn setup_frames(
    assets: Res<Assets<TextureAtlasLayout>>,
    mut query: Query<(&TextureAtlas, &mut ActiveAnimation), Changed<ActiveAnimation>>,
) {
    for (texture, mut animation) in query.iter_mut() {
        match animation.mode {
            anim::AnimationMode::ActionReady { frame_duration: _ } => continue,
            _ => {
                let Some(layout) = assets.get(texture.layout.id()) else {
                    continue;
                };

                let ending_offset = animation.0.ending_offset.unwrap_or(0);

                let animation_frames = layout.len() - ending_offset;

                let frames: Vec<usize> = (0..animation_frames).collect();
                animation.0.frames = Some(frames);
            }
        }
    }
}

/// Runs a system that detects inserted scoped action components and inserts the relevant animation
fn setup_action_durations(
    scoped_actions: Query<(&ScopedAction, &Children), Changed<ScopedAction>>,
    mut sprite: Query<(&mut ActiveAnimation, &mut AnimationState)>,
) {
    for (scoped_action, children) in scoped_actions.iter() {
        for child in children.iter() {
            let Ok((mut animation, _animation_state)) = sprite.get_mut(*child) else {
                continue;
            };
            match animation.mode {
                anim::AnimationMode::Action => {
                    let actual_duration = scoped_action
                        .duration
                        .div_f64(animation.frames.as_ref().unwrap_or(&vec![0]).len() as f64);
                    animation.0.mode = anim::AnimationMode::ActionReady {
                        frame_duration: actual_duration,
                    };
                }
                _ => {
                    continue;
                }
            }
        }
    }
}
