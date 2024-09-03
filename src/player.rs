use bevy::prelude::*;

use crate::movement::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene);
        app.add_systems(Update, handle_move_input);
    }
}

#[derive(Debug, Component)]
struct Player;

fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle::default());
    let texture = asset_server.load("Characters/Basic Charakter Spritesheet.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 4, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        Player,
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(2.0)),
            texture: texture.clone(),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 1,
        },
        Velocity::default(),
        PhysicalTranslation::default(),
        PreviousPhysicalTranslation::default(),
    ));
}

fn handle_move_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut TextureAtlas)>,
) {
    const SPEED: f32 = 210.0;
    query.iter_mut().for_each(|(mut velocity, mut sprite)| {
        velocity.0 = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            velocity.y += 1.0;
            sprite.index = 5;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            velocity.y -= 1.0;
            sprite.index = 1;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            velocity.x -= 1.0;
            sprite.index = 9;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            velocity.x += 1.0;
            sprite.index = 13;
        }

        // Need to normalize and scale because otherwise
        // diagonal movement would be faster than horizontal or vertical movement.
        velocity.0 = velocity.normalize_or_zero() * SPEED;
    });
}
