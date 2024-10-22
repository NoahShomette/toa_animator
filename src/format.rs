use std::fmt::Display;

use bevy::{
    asset::{Asset, AssetLoader, AssetServer, Assets, AsyncReadExt, Handle},
    math::UVec2,
    prelude::{Image, ResMut},
    reflect::TypePath,
    sprite::TextureAtlasLayout,
    utils::HashMap,
};
use serde::{Deserialize, Serialize};

use crate::Animations;

#[derive(Asset, TypePath, Debug, PartialEq, Eq)]
pub struct ArtCollection {
    pub animations: Animations,
    pub textures: HashMap<String, TextureInfo>,
}

/// Final representation of a loaded texture
#[derive(TypePath, Debug, PartialEq, Eq)]
pub enum TextureInfo {
    Atlas {
        name: String,
        texture: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
    },
    Sprite {
        name: String,
        texture: Handle<Image>,
    },
}

impl TextureInfo {
    pub fn texture_handle(&self) -> &Handle<Image> {
        match self {
            TextureInfo::Atlas {
                name: _,
                texture,
                layout: _,
            } => texture,
            TextureInfo::Sprite { name: _, texture } => texture,
        }
    }
    pub fn texture_name(&self) -> &str {
        match self {
            TextureInfo::Atlas {
                name,
                texture: _,
                layout: _,
            } => name,
            TextureInfo::Sprite { name, texture: _ } => name,
        }
    }
}

/// Representation of a texture in a file
#[derive(Serialize, Deserialize)]
pub enum TextureAsset {
    Atlas {
        name: String,
        file_path: String,
        rows: u32,
        columns: u32,
        tile_size_x: u32,
        tile_size_y: u32,
    },
    Sprite {
        name: String,
        file_path: String,
    },
}

impl TextureAsset {
    pub fn load(
        &self,
        asset_server: &ResMut<AssetServer>,
        atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> TextureInfo {
        match self {
            TextureAsset::Atlas {
                name,
                file_path,
                rows,
                columns,
                tile_size_x,
                tile_size_y,
            } => {
                let building_texture = asset_server.load(file_path.clone());
                let atlas = TextureAtlasLayout::from_grid(
                    UVec2::new(*tile_size_x, *tile_size_y),
                    *columns,
                    *rows,
                    None,
                    None,
                );

                TextureInfo::Atlas {
                    name: name.clone(),
                    texture: building_texture,
                    layout: atlases.add(atlas).clone(),
                }
            }
            TextureAsset::Sprite { name, file_path } => {
                let building_texture = asset_server.load(file_path.clone());
                TextureInfo::Sprite {
                    name: name.clone(),
                    texture: building_texture,
                }
            }
        }
    }
}
