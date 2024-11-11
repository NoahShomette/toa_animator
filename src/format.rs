use bevy::{
    asset::{AssetServer, Assets, Handle},
    math::UVec2,
    prelude::Image,
    reflect::Reflect,
    sprite::TextureAtlasLayout,
};
use serde::{Deserialize, Serialize};

/// Final representation of a loaded texture
#[derive(Debug, PartialEq, Eq, Clone, Reflect)]
pub struct TextureAtlasInfo {
    pub name: String,
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl TextureAtlasInfo {
    pub fn texture_handle(&self) -> &Handle<Image> {
        &self.texture
    }
    pub fn texture_name(&self) -> &str {
        &self.name
    }
}

/// Representation of a texture in a file
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextureAtlasAsset {
    pub name: String,
    pub file_path: String,
    pub rows: u32,
    pub columns: u32,
    pub tile_size_x: u32,
    pub tile_size_y: u32,
}

impl TextureAtlasAsset {
    pub fn load(
        &self,
        asset_server: &AssetServer,
        atlases: &mut Assets<TextureAtlasLayout>,
    ) -> TextureAtlasInfo {
        let building_texture = asset_server.load(self.file_path.clone());
        let atlas = TextureAtlasLayout::from_grid(
            UVec2::new(self.tile_size_x, self.tile_size_y),
            self.columns,
            self.rows,
            None,
            None,
        );

        TextureAtlasInfo {
            name: self.name.clone(),
            texture: building_texture,
            layout: atlases.add(atlas).clone(),
        }
    }
}
