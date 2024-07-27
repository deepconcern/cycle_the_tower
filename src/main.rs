use bevy::{prelude::*, render::{camera::{RenderTarget, ScalingMode}, render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}, texture::BevyDefault, view::RenderLayers}, sprite::Anchor};

const UNIT_SIZE: f32 = 32.0;

const GAME_LAYER: RenderLayers = RenderLayers::layer(1);
const GAME_WIDTH: f32 = UNIT_SIZE * 16.0;
const GAME_HEIGHT: f32 = UNIT_SIZE * 9.0;

// Enemy display
const ENEMY_DISPLAY_TRANSLATION: Vec3 = Vec3::new(0.0, UNIT_SIZE * 3.0, 0.0);
const ENEMY_SPRITE_SIZE: f32 = 64.0;

#[derive(Resource)]
struct GameArea {
    rect: Rect,
}

impl Default for GameArea {
    fn default() -> Self {
        Self {
            rect: Rect::new(0.0, 0.0, GAME_WIDTH, GAME_HEIGHT),
        }
    }
}



fn setup_cameras(asset_server: Res<AssetServer>, mut commands: Commands, mut images: ResMut<Assets<Image>>) {
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

#[derive(Component)]
struct Enemy {
    current_hp: usize,
    max_hp: usize,
    name: String,
}

impl Enemy {
    fn new(name: &str, hp: usize) -> Self {
        Self {
            current_hp: hp,
            max_hp: hp,
            name: name.to_string(),
        }
    }
}

#[derive(Component)]
struct EnemyHealthText;

#[derive(Component)]
struct EnemyNameText;

fn setup_scene(asset_server: Res<AssetServer>, mut commands: Commands) {
    let font_handle = asset_server.load("fonts/PressStart2p-Regular.ttf");

    // Enemy display

    commands.spawn(SpatialBundle {
        transform: Transform {
            translation: ENEMY_DISPLAY_TRANSLATION,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        let enemy_name_y = (ENEMY_SPRITE_SIZE / 2.0) + UNIT_SIZE;
        let enemy_info_y = -((ENEMY_SPRITE_SIZE / 2.0) + UNIT_SIZE);

        parent.spawn((Enemy::new("Rat", 100), SpriteBundle {
            texture: asset_server.load("rat.png"),
            ..default()
        }));

        parent.spawn(Text2dBundle {
            text: Text::from_section("Name", TextStyle {
                font: font_handle.clone(),
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(0.0, enemy_name_y, 0.0),
                ..default()
            },
            ..default()
        });

        parent.spawn(Text2dBundle {
            text: Text::from_section("Info", TextStyle {
                font: font_handle.clone(),
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(0.0, enemy_info_y, 0.0),
                ..default()
            },
            ..default()
        });
    });

    // Player display
}

#[derive(Component)]
struct EnemySpriteTarget;

// fn setup_ui(asset_server: Res<AssetServer>, mut commands: Commands) {
//     let font_handle = asset_server.load("fonts/PressStart2p-Regular.ttf");

//     commands.spawn(NodeBundle {
//         style: Style {
//             display: Display::Flex,
//             flex_direction: FlexDirection::Column,
//             height: Val::Vh(100.0),
//             width: Val::Vw(100.0),
//             ..default()
//         },
//         ..default()
//     }).with_children(|parent| {
//         // Enemy area
//         parent.spawn(NodeBundle {
//             style: Style {
//                 flex_direction: FlexDirection::Column,
//                 flex_grow: 1.0,
//                 ..default()
//             },
//             ..default() // TODO
//         }).with_children(|parent| {
//             // Enemy display
//             parent.spawn(NodeBundle {
//                 style: Style {
//                     justify_content: JustifyContent::Center,
//                     flex_grow: 1.0,
//                     ..default()
//                 },
//                 ..default()
//             }).with_children(|parent| {
//                 parent.spawn((EnemyNameText, TextBundle::from_section("EnemyNameText", TextStyle {
//                     font: font_handle.clone(),
//                     ..default()
//                 })));
//                 parent.spawn((EnemySpriteTarget, NodeBundle::default()));
//             });

//             // Enemy info
//             parent.spawn(NodeBundle {
//                 style: Style {
//                     display: Display::Flex,
//                     justify_content: JustifyContent::SpaceAround,
//                     ..default()
//                 },
//                 ..default()
//             }).with_children(|parent| {
//                 // Health
//                 parent.spawn((
//                     EnemyHealthText,
//                     TextBundle::from_section("EnemyHealthText", TextStyle {
//                         font: font_handle.clone(),
//                         ..default()
//                     }).with_text_justify(JustifyText::Center).with_style(Style {
//                         flex_grow: 1.0,
//                         ..default()
//                     }),
//                 ));

//                 // Actions
//                 parent.spawn(
//                     TextBundle::from_section("Next actions: (TODO)", TextStyle {
//                         font: font_handle.clone(),
//                         ..default()
//                     }).with_text_justify(JustifyText::Center).with_style(Style {
//                         flex_grow: 1.0,
//                         ..default()
//                     }),
//                 );
//             });
//         });

//         // Card target area
//         parent.spawn(NodeBundle {
//             background_color: Color::linear_rgb(0.0, 256.0, 0.0).into(),
//             style: Style {
//                 flex_grow: 1.0,
//                 ..default()
//             },
//             ..default() // TODO
//         });

//         // Player area
//         parent.spawn(NodeBundle {
//             style: Style {
//                 display: Display::Flex,
//                 flex_direction: FlexDirection::Column,
//                 flex_grow: 1.0,
//                 ..default()
//             },
//             ..default()
//         }).with_children(|parent| {

//             // Player cards
//             parent.spawn(NodeBundle {
//                 background_color: Color::linear_rgb(0.0, 0.0, 256.0).into(),
//                 style: Style {
//                     flex_grow: 1.0,
//                     ..default()
//                 },
//                 ..default() // TODO
//             });
//         });
//     });
// }

fn enemy_health_text(mut enemy_health_query: Query<&mut Text, With<EnemyHealthText>>, enemy_query: Query<&Enemy>) {
    let enemy = enemy_query.single();
    let mut text = enemy_health_query.single_mut();

    text.sections[0].value = format!("HP: {}/{}", enemy.current_hp, enemy.max_hp);
}

fn enemy_name_text(mut enemy_name_query: Query<&mut Text, With<EnemyNameText>>, enemy_query: Query<&Enemy>) {
    let enemy = enemy_query.single();
    let mut text = enemy_name_query.single_mut();

    // Update the name

    text.sections[0] = (&enemy.name).to_string().into();
}

fn enemy_sprite(mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<EnemySpriteTarget>)>, enemy_target_query: Query<(&Transform, &GlobalTransform), (With<EnemySpriteTarget>, Without<Enemy>)>) {
    let mut enemy_transform = enemy_query.single_mut();
    let (enemy_target_transform, global_transform) = enemy_target_query.single();

    println!("DEBUG: enemy_transform {:?}", enemy_transform.translation);
    println!("DEBUG: enemy_target_transform {:?}", enemy_target_transform.translation);
    println!("DEBUG: global_transform {:?}", global_transform.translation());

    enemy_transform.translation = enemy_target_transform.translation;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (setup_cameras, setup_scene).chain())
        // .add_systems(Update, ())
        .init_resource::<GameArea>()
        .run();
}
