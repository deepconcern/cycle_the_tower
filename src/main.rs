mod drag;
mod enemy;
mod mouse;

use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    render::view::RenderLayers,
};
use bevy_prng::WyRand;
use bevy_rand::{
    plugin::EntropyPlugin,
    prelude::{ForkableRng, GlobalEntropy},
};
use drag::{DragEvent, DragPlugin, DragState, Draggable};
use enemy::{Enemy, EnemyHealthText, EnemyNameText, EnemyPlugin};
use mouse::{MousePlugin, MousePosition};
use rand::seq::IteratorRandom;

const SCALE_FACTOR: f32 = 2.0;
const UNIT_SIZE: f32 = 32.0;

const GAME_LAYER: RenderLayers = RenderLayers::layer(1);
const GAME_WIDTH: f32 = UNIT_SIZE * 16.0;
const GAME_HEIGHT: f32 = UNIT_SIZE * 9.0;

// Enemy display
const ENEMY_DISPLAY_TRANSLATION: Vec3 = Vec3::new(0.0, UNIT_SIZE * 6.0, 0.0);
const ENEMY_SPRITE_SIZE: f32 = 64.0;

// Player display
const CARD_DROP_TARGET_SIZE: f32 = 8.0;
const CARD_SPRITE_SIZE: f32 = 32.0;
const PLAYER_DISPLAY_TRANSLATION: Vec3 = Vec3::new(0.0, -(UNIT_SIZE * 6.0), 0.0);

#[derive(Resource)]
struct Player {
    current_hp: usize,
    max_hp: usize,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            current_hp: 100,
            max_hp: 100,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum CardType {
    Attack,
    Guard,
    None,
}

#[derive(Component)]
struct CardDropTarget;

#[derive(Component)]
struct Card(CardType);

#[derive(Component)]
struct HandPosition(usize);

#[derive(Component)]
struct CommandPosition(usize);

#[derive(Component)]
struct CardPosition {
    position_type: CardPositionType,
}

enum CardPositionType {
    Command(usize),
    Hand(usize),
}

fn setup_cameras(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    // let size = Extent3d {
    //     height: GAME_HEIGHT as u32,
    //     width: GAME_WIDTH as u32,
    //     ..default()
    // };

    // let data = vec![0; (GAME_WIDTH * GAME_HEIGHT * 4.0) as usize];

    // let mut render_target = Image::new(size, TextureDimension::D2, data, TextureFormat::bevy_default(), RenderAssetUsages::all());

    // render_target.texture_descriptor.usage = TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING;

    // let render_target_handle = images.add(render_target);

    // commands.spawn(Camera2dBundle {
    //     camera: Camera {
    //         order: -1,
    //         target: RenderTarget::Image(render_target_handle.clone()),
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(GAME_WIDTH / 2.0, GAME_HEIGHT / 2.0, 1.0),
    //     ..default()
    // });

    // commands.spawn(SpriteBundle {
    //     texture: render_target_handle,
    //     ..default()
    // });

    // commands.spawn(Camera2dBundle {
    //     projection: OrthographicProjection {
    //         scaling_mode: ScalingMode::AutoMin { min_width: GAME_WIDTH, min_height: GAME_HEIGHT },
    //         ..default()
    //     },
    //     ..default()
    // });
    commands.spawn(Camera2dBundle::default());
}

fn setup_scene(asset_server: Res<AssetServer>, mut commands: Commands) {
    // Font handle

    let font_handle = asset_server.load("fonts/press_start_2p.ttf");

    // Textures

    let card_target_texture_handle = asset_server.load("card_target.png");

    // Enemy display

    commands
        .spawn(SpatialBundle {
            transform: Transform {
                translation: ENEMY_DISPLAY_TRANSLATION,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            let enemy_name_y = (ENEMY_SPRITE_SIZE) + UNIT_SIZE;
            let enemy_info_y = -((ENEMY_SPRITE_SIZE) + UNIT_SIZE);

            // Background

            parent.spawn(SpriteBundle {
                texture: asset_server.load("enemy_background.png"),
                transform: Transform {
                    scale: Vec3::splat(SCALE_FACTOR),
                    ..default()
                },
                ..default()
            });

            // Enemy sprite display

            parent.spawn((
                Enemy::new("Rat", 100),
                SpriteBundle {
                    texture: asset_server.load("rat.png"),
                    transform: Transform {
                        scale: Vec3::splat(SCALE_FACTOR),
                        ..default()
                    },
                    ..default()
                },
            ));

            // Enemy name

            parent.spawn((
                EnemyNameText,
                Text2dBundle {
                    text: Text::from_section(
                        "EnemyNameText",
                        TextStyle {
                            font: font_handle.clone(),
                            ..default()
                        },
                    ),
                    transform: Transform {
                        translation: Vec3::new(0.0, enemy_name_y, 0.0),
                        ..default()
                    },
                    ..default()
                },
            ));

            // Info area

            parent
                .spawn(SpatialBundle {
                    transform: Transform {
                        translation: Vec3::new(0.0, enemy_info_y, 0.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Enemy health

                    parent.spawn((
                        EnemyHealthText,
                        Text2dBundle {
                            transform: Transform {
                                translation: Vec3::new(-(UNIT_SIZE * 7.0), 0.0, 0.0),
                                ..default()
                            },
                            text: Text::from_section(
                                "EnemyHealthText",
                                TextStyle {
                                    font: font_handle.clone(),
                                    ..default()
                                },
                            ),
                            ..default()
                        },
                    ));

                    //Enemy actions

                    parent.spawn(Text2dBundle {
                        transform: Transform {
                            translation: Vec3::new(UNIT_SIZE * 7.0, 0.0, 0.0),
                            ..default()
                        },
                        text: Text::from_section(
                            "Next actions: ?, ?, ?",
                            TextStyle {
                                font: font_handle.clone(),
                                ..default()
                            },
                        ),
                        ..default()
                    });
                });
        });

    // Input area

    commands
        .spawn(SpatialBundle::default())
        .with_children(|parent| {
            for i in 0..3 {
                let translation =
                    Vec3::new((i as f32) * (2.0 * UNIT_SIZE * SCALE_FACTOR), 0.0, 0.0);

                parent.spawn((
                    CardDropTarget,
                    CommandPosition(i),
                    SpriteBundle {
                        texture: card_target_texture_handle.clone(),
                        transform: Transform {
                            scale: Vec3::splat(SCALE_FACTOR),
                            translation,
                            ..default()
                        },
                        ..default()
                    },
                ));
            }
        });

    // Player display

    commands
        .spawn(SpatialBundle {
            transform: Transform {
                translation: PLAYER_DISPLAY_TRANSLATION,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Hand

            for i in 0..3 {
                let translation =
                    Vec3::new((i as f32) * (2.0 * UNIT_SIZE * SCALE_FACTOR), 0.0, 0.0);

                parent.spawn((
                    CardDropTarget,
                    HandPosition(i),
                    SpriteBundle {
                        texture: card_target_texture_handle.clone(),
                        transform: Transform {
                            scale: Vec3::splat(SCALE_FACTOR),
                            translation,
                            ..default()
                        },
                        ..default()
                    },
                ));
            }
        });
}

fn card_display(mut card_query: Query<(&mut Card, &mut TextureAtlas)>) {
    for (card, mut texture_atlas) in card_query.iter_mut() {
        match card.0 {
            CardType::Attack => {
                texture_atlas.index = 0;
            }
            CardType::Guard => {
                texture_atlas.index = 1;
            }
            CardType::None => {
                texture_atlas.index = 3;
            }
        }
    }
}

fn initial_hand(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut global_rng: ResMut<GlobalEntropy<WyRand>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Card texture and layout

    let card_texture_handle = asset_server.load("cards.png");
    let card_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 2, 2, None, None);
    let card_layout_handle = texture_atlas_layouts.add(card_layout);

    // Fork RNG

    let mut rng = global_rng.fork_rng();

    // Shuffle new hand

    let choices = [CardType::Attack, CardType::Guard];

    for i in 0..3 {
        commands.spawn((
            Card(choices.iter().choose(&mut rng).unwrap().clone()),
            CardPosition {
                position_type: CardPositionType::Hand(i),
            },
            Draggable::Aabb(Aabb2d::new(
                Vec2::ZERO,
                Vec2::splat(CARD_SPRITE_SIZE * SCALE_FACTOR * 0.5),
            )),
            SpriteBundle {
                texture: card_texture_handle.clone(),
                transform: Transform {
                    scale: Vec3::splat(SCALE_FACTOR),
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                index: 0,
                layout: card_layout_handle.clone(),
            },
        ));
    }
}

fn card_aabb(mut card_query: Query<(&mut Draggable, &mut GlobalTransform), With<Card>>) {
    for (mut draggable, global_transform) in card_query.iter_mut() {
        draggable.set_center(global_transform.translation().truncate());
    }
}

fn card_drop(
    mut card_query: Query<&mut CardPosition>,
    command_drop_target_query: Query<(&GlobalTransform, &CommandPosition), With<CardDropTarget>>,
    mut drag_event_reader: EventReader<DragEvent>,
    hand_drop_target_query: Query<(&GlobalTransform, &HandPosition), With<CardDropTarget>>,
) {
    for event in drag_event_reader.read() {
        println!("DEBUG: event = {:?}", event);

        let card_aabb = Aabb2d::new(
            event.draggable_position,
            Vec2::splat(CARD_SPRITE_SIZE * SCALE_FACTOR * 0.5),
        );
        let mut position = card_query.get_mut(event.draggable_id).unwrap();
        let half_size = Vec2::splat(CARD_DROP_TARGET_SIZE * SCALE_FACTOR * 0.5);

        // Check hand

        for (global_transform, hand_position) in hand_drop_target_query.iter() {
            let target_aabb = Aabb2d::new(global_transform.translation().truncate(), half_size);

            if card_aabb.intersects(&target_aabb) {
                position.position_type = CardPositionType::Hand(hand_position.0);

                return;
            }
        }

        // Check command

        for (global_transform, command_position) in command_drop_target_query.iter() {
            let target_aabb = Aabb2d::new(global_transform.translation().truncate(), half_size);

            if card_aabb.intersects(&target_aabb) {
                position.position_type = CardPositionType::Command(command_position.0);

                return;
            }
        }
    }
}

fn card_position(
    mut card_query: Query<(&mut CardPosition, Entity, &mut Transform), With<Card>>,
    command_position_query: Query<(&GlobalTransform, &CommandPosition)>,
    drag_state: Res<DragState>,
    hand_position: Query<(&GlobalTransform, &HandPosition)>,
) {
    for (card_position, entity, mut transform) in card_query.iter_mut() {
        // Don't interfere with dragging
        if Some(entity) == drag_state.current_entity {
            continue;
        }

        match card_position.position_type {
            CardPositionType::Command(index) => {
                for (global_transform, command_position) in command_position_query.iter() {
                    if command_position.0 == index.clone() {
                        transform.translation = global_transform.translation();
                    }
                }
            }
            CardPositionType::Hand(index) => {
                for (global_transform, hand_position) in hand_position.iter() {
                    if hand_position.0 == index.clone() {
                        transform.translation = global_transform.translation();
                    }
                }
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (GAME_WIDTH * SCALE_FACTOR, GAME_HEIGHT * SCALE_FACTOR).into(),
                    ..default()
                }),
                ..default()
            }),
            DragPlugin,
            EnemyPlugin,
            EntropyPlugin::<WyRand>::default(),
            MousePlugin,
        ))
        .add_systems(Startup, (setup_cameras, setup_scene, initial_hand).chain())
        .add_systems(Update, (card_aabb, card_display, card_drop, card_position))
        .init_resource::<Player>()
        .run();
}
