use core::time::Duration;

use bevy::{
    asset::{AssetServer, Assets},
    prelude::{Component, Deref},
    reflect::{Reflect, TypePath},
    sprite::TextureAtlasLayout,
    utils::HashMap,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{TextureAtlasAsset, TextureAtlasInfo};

/// An animation represented as an asset in a file
#[derive(Debug, Clone, Eq, PartialEq, Component, TypePath)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AnimationAsset {
    /// Frames
    pub frames: Option<Vec<usize>>,
    /// Animation mode
    pub mode: AnimationMode,
    /// The offset from the end of the file to the frame that the animation should end on. None if the animation is packed completely.
    pub ending_offset: Option<usize>,
    pub texture_atlas_asset: TextureAtlasAsset,
}

impl AnimationAsset {
    pub fn load(
        self,
        asset_server: &AssetServer,
        atlases: &mut Assets<TextureAtlasLayout>,
    ) -> Animation {
        Animation {
            frames: self.frames,
            mode: self.mode,
            ending_offset: self.ending_offset,
            texture_atlas_info: self.texture_atlas_asset.load(asset_server, atlases),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect)]
pub struct Animation {
    /// Frames
    pub frames: Option<Vec<usize>>,
    /// Animation mode
    pub mode: AnimationMode,
    /// The offset from the end of the file to the frame that the animation should end on. None if the animation is packed completely.
    pub ending_offset: Option<usize>,
    pub texture_atlas_info: TextureAtlasInfo,
}

impl Animation {
    /// Create a new animation from frames
    #[must_use]
    pub fn from_frames(
        frames: impl IntoIterator<Item = usize>,
        texture_atlas_info: TextureAtlasInfo,
    ) -> Self {
        Self {
            frames: Some(frames.into_iter().collect()),
            mode: AnimationMode::default(),
            ending_offset: None,
            texture_atlas_info,
        }
    }

    /// Runs the animation once and then stop playing
    #[must_use]
    pub fn once(mut self) -> Self {
        self.mode = AnimationMode::Once;
        self
    }

    /// Repeat the animation forever
    #[must_use]
    pub fn repeat(mut self) -> Self {
        self.mode = AnimationMode::RepeatFrom(0);
        self
    }

    /// Repeat the animation forever, from a given frame index (loop back to it at the end of the
    /// animation)
    #[must_use]
    pub fn repeat_from(mut self, frame_index: usize) -> Self {
        self.mode = AnimationMode::RepeatFrom(frame_index);
        self
    }

    /// Repeat the animation forever, going back and forth between the first and last frame.
    #[must_use]
    pub fn ping_pong(mut self) -> Self {
        self.mode = AnimationMode::PingPong;
        self
    }

    pub(crate) fn has_frames(&self) -> bool {
        self.frames.is_some()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Reflect)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AnimationMode {
    /// Plays the animation once with the default frame rate
    Once,
    /// Internal mode signifying that the action animation is setup and ready to be played
    ActionReady { frame_duration: Duration },
    /// Plays the animation once timed to the given action. Converts itself into [`Self::ActionReady`] when it has been setup
    Action,
    /// Repeats the entire animation
    Repeat,
    /// Repeats the animation from the given frame
    RepeatFrom(usize),
    /// Plays the full animation back and forth repeated
    PingPong,
}

impl Extend<usize> for Animation {
    fn extend<T: IntoIterator<Item = usize>>(&mut self, iter: T) {
        match &mut self.frames {
            Some(frames) => frames.extend(iter),
            None => (),
        }
    }
}

impl Default for AnimationMode {
    #[inline]
    fn default() -> Self {
        Self::RepeatFrom(0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, TypePath)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AnimationsAsset {
    /// Default animation used. Must exist
    pub idle: AnimationAsset,
    /// List of animations for specific situations. If one doesnt exst the idle will be returned
    pub animations: HashMap<String, AnimationAsset>,
}

impl AnimationsAsset {
    pub fn load(
        self,
        asset_server: &AssetServer,
        atlases: &mut Assets<TextureAtlasLayout>,
    ) -> Animations {
        Animations {
            idle: self.idle.load(asset_server, atlases),
            animations: self
                .animations
                .iter()
                .map(|item| (item.0.clone(), item.1.clone().load(asset_server, atlases)))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Animations {
    /// Default animation used. Must exist
    pub idle: Animation,
    /// List of animations for specific situations. If one doesnt exst the idle will be returned
    pub animations: HashMap<String, Animation>,
}

impl Animations {
    /// returns the idle animation
    pub fn idle(&self) -> &Animation {
        &self.idle
    }

    /// Returns the requested animation. Returns idle if the animation cant be found
    pub fn animation(&self, label: impl Into<String>) -> &Animation {
        let label = label.into();

        self.animations.get(&label).unwrap_or_else(|| self.idle())
    }
}

// The animation currently active on the entity
#[derive(Component, Deref, Reflect)]
pub struct ActiveAnimation(pub Animation);

impl ActiveAnimation {
    pub fn new(animation: Animation) -> Self {
        Self(animation)
    }
}

#[derive(Component, Serialize, Deserialize)]
/// An action on a thing that is scoped to a specific duration
pub struct ScopedAction {
    pub duration: Duration,
}
