mod enemy;
mod mouse;

use std::time::Duration;

use bevy::{asset::AssetMetaCheck, prelude::*, sprite::Anchor, text::Text2dBounds};
use bevy_prng::WyRand;
use bevy_rand::plugin::EntropyPlugin;
use enemy::{Enemy, EnemyHealthText, EnemyNameText, EnemyPlugin};
use mouse::MousePlugin;

const SCALE_FACTOR: f32 = 2.0;
const SCALE_VEC3: Vec3 = Vec3::splat(SCALE_FACTOR);
const UNIT_SIZE: f32 = 32.0;

const GAME_WIDTH: f32 = UNIT_SIZE * 16.0;
const GAME_HEIGHT: f32 = UNIT_SIZE * 9.0;

// Enemy display
const ENEMY_DISPLAY_TRANSLATION: Vec3 = Vec3::new(0.0, GAME_HEIGHT * SCALE_FACTOR * 0.25, 0.0);
const ENEMY_SPRITE_SIZE: f32 = 64.0;

// Player display
const HERO_COL_WIDTH: f32 = (GAME_WIDTH * SCALE_FACTOR) / 6.0;
const HERO_ONE_OFFSET: f32 = -HERO_COL_WIDTH;
const HERO_TWO_OFFSET: f32 = 0.0;
const HERO_THREE_OFFSET: f32 = -HERO_ONE_OFFSET;
const HERO_SPRITE_SIZE: f32 = 32.0;
const PLAYER_DISPLAY_TRANSLATION: Vec3 = Vec3::new(0.0, -(GAME_HEIGHT * SCALE_FACTOR * 0.25), 0.0);

// Menu
const MENU_BLOCK: f32 = UNIT_SIZE * SCALE_FACTOR * 0.75;
const MENU_START_OFFSET: f32 = 1.5 * MENU_BLOCK;
const MENU_OPTIONS: [[&str; 4]; 3] = [
    ["Attack", "Reckless Attack", "Block", "Cycle Hero"],
    [
        "Magic Missle",
        "Shield Warrior",
        "Shield Priest",
        "Cycle Hero",
    ],
    ["Heal Warrior", "Heal Mage", "Heal Self", "Cycle Hero"],
];

// Battle info
const BATTLE_INFO_START_OFFSET: f32 = MENU_BLOCK * 2.0;

// Attack values
const ENEMY_ATTACK_DAMAGE: u32 = 5;
const MAGIC_MISSLE_DAMAGE: u32 = 2;
const ATTACK_DAMAGE: u32 = 1;
const RECKLESS_ATTACK_DAMAGE: u32 = 3;
const HEAL_AMOUNT: u32 = 3;
// const BLOCK_AMOUNT: u32 = 1;

#[derive(Resource)]
struct Player {
    current_hps: [isize; 3],
    current_hero: usize,
    is_warrior_blocking: bool,
    max_hps: [isize; 3],
    shielded: [usize; 3],
    sleep_state: [bool; 3],
}

impl Default for Player {
    fn default() -> Self {
        Self {
            current_hps: [10, 20, 10],
            current_hero: 0,
            is_warrior_blocking: false,
            max_hps: [10, 20, 10],
            shielded: [0, 0, 0],
            sleep_state: [false, false, false],
        }
    }
}

#[derive(Default, Resource)]
struct MenuSelection(usize);

#[derive(Component)]
struct MenuOption(usize);

#[derive(Component)]
struct MenuArrow;

#[derive(Debug)]
enum MageAction {
    Missle,
    ShieldPriest,
    ShieldWarrior,
}

#[derive(Debug)]
enum PriestAction {
    HealMage,
    HealSelf,
    HealWarrior,
}

#[derive(Debug)]
enum WarriorAction {
    Attack,
    Reckless,
    Block,
}

#[derive(Debug)]
enum PlayerAction {
    Mage(MageAction),
    Priest(PriestAction),
    Warrior(WarriorAction),
}

#[derive(Debug, Event)]
enum ActionEvent {
    Enemy,
    Player(PlayerAction),
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
enum InfoPanelState {
    Battle,
    #[default]
    Menu,
}

#[derive(Component)]
struct InfoPanelNode;

#[derive(Component)]
struct InfoPanelTarget;

#[derive(Component)]
struct BattleInfoText;

#[derive(Default, Resource)]
struct BattleInfoTimer(Option<Timer>);

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
enum BattleState {
    Enemy,
    Lose,
    #[default]
    Player,
    Win,
}

#[derive(Component)]
struct Hero(usize);

#[derive(Component)]
struct HeroHealthText(usize);

#[derive(Component)]
struct HeroStatusText(usize);

#[derive(Component)]
struct HeroArrow;

fn setup_cameras(
    // asset_server: Res<AssetServer>,
    mut commands: Commands,
    // mut images: ResMut<Assets<Image>>,
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

fn setup_scene(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Font handle

    let font_handle = asset_server.load("fonts/press_start_2p.ttf");

    // Textures

    let basic_background_texture_handle = asset_server.load("basic_background.png");
    let heroes_texture_handle = asset_server.load("heroes.png");
    let heroes_layout_handle = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(HERO_SPRITE_SIZE as u32),
        2,
        3,
        None,
        None,
    ));

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
                    scale: SCALE_VEC3,
                    translation: Vec3::new(0.0, 0.0, -1.0),
                    ..default()
                },
                ..default()
            });

            // Enemy sprite display

            parent.spawn((
                Enemy::new("Rat", 10),
                SpriteBundle {
                    texture: asset_server.load("rat.png"),
                    transform: Transform {
                        scale: SCALE_VEC3,
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

            // Enemy health

            parent.spawn((
                EnemyHealthText,
                Text2dBundle {
                    transform: Transform {
                        translation: Vec3::new(0.0, enemy_info_y, 0.0),
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
            // Background sprite info
            let scale_mode = ImageScaleMode::Sliced(TextureSlicer {
                border: BorderRect::square(4.0),
                center_scale_mode: SliceScaleMode::Stretch,
                sides_scale_mode: SliceScaleMode::Stretch,
                max_corner_scale: 2.0,
            });
            let size = Some(Vec2::new(GAME_WIDTH * 0.5, GAME_HEIGHT * 0.5));

            // Hero pane

            parent
                .spawn(SpatialBundle {
                    transform: Transform {
                        translation: Vec3::new(-(GAME_WIDTH * SCALE_FACTOR * 0.25), 0.0, 0.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Background

                    parent.spawn((
                        scale_mode.clone(),
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: size,
                                ..default()
                            },
                            texture: basic_background_texture_handle.clone(),
                            transform: Transform {
                                scale: SCALE_VEC3,
                                translation: Vec3::new(0.0, 0.0, -1.0),
                                ..default()
                            },
                            ..default()
                        },
                    ));

                    // Heroes
                    for i in 0..3 {
                        let x_offset = if i == 0 {
                            HERO_ONE_OFFSET
                        } else if i == 1 {
                            HERO_TWO_OFFSET
                        } else {
                            HERO_THREE_OFFSET
                        };

                        parent
                            .spawn((
                                Hero(i),
                                SpriteBundle {
                                    texture: heroes_texture_handle.clone(),
                                    transform: Transform {
                                        scale: SCALE_VEC3,
                                        translation: Vec3::new(x_offset, 0.0, 0.0),
                                        ..default()
                                    },
                                    ..default()
                                },
                                TextureAtlas {
                                    index: i * 2,
                                    layout: heroes_layout_handle.clone(),
                                },
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    HeroStatusText(i),
                                    Text2dBundle {
                                        text: Text::from_section(
                                            "HeroStatusText",
                                            TextStyle {
                                                color: Color::NONE,
                                                font: font_handle.clone(),
                                                font_size: 10.0,
                                                ..default()
                                            },
                                        ),
                                        transform: Transform {
                                            scale: Vec3::splat(0.5), // Undo parent scaling.. oops..
                                            translation: Vec3::ZERO,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                ));

                                parent.spawn((
                                    HeroHealthText(i),
                                    Text2dBundle {
                                        text: Text::from_section(
                                            "HeroHealthText",
                                            TextStyle {
                                                font: font_handle.clone(),
                                                font_size: 10.0,
                                                ..default()
                                            },
                                        ),
                                        transform: Transform {
                                            scale: Vec3::splat(0.5), // Undo parent scaling.. oops..
                                            translation: Vec3::new(
                                                0.0,
                                                -(HERO_SPRITE_SIZE * SCALE_FACTOR * 0.5),
                                                0.0,
                                            ),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                ));
                            });
                    }

                    // Hero arrow
                    parent.spawn((
                        HeroArrow,
                        SpriteBundle {
                            texture: asset_server.load("arrow_down.png"),
                            transform: Transform {
                                scale: SCALE_VEC3,
                                translation: Vec3::new(
                                    0.0,
                                    HERO_SPRITE_SIZE * SCALE_FACTOR * 0.75,
                                    0.0,
                                ),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });

            // Command/menu pane

            parent
                .spawn((
                    InfoPanelTarget,
                    SpatialBundle {
                        transform: Transform {
                            translation: Vec3::new(GAME_WIDTH * SCALE_FACTOR * 0.25, 0.0, 0.0),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    // Background

                    parent.spawn((
                        scale_mode.clone(),
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: size,
                                ..default()
                            },
                            texture: basic_background_texture_handle.clone(),
                            transform: Transform {
                                scale: SCALE_VEC3,
                                translation: Vec3::new(0.0, 0.0, -1.0),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });
        });
}

fn hero_arrow(mut hero_arrow_query: Query<&mut Transform, With<HeroArrow>>, player: Res<Player>) {
    let mut transform = hero_arrow_query.single_mut();

    transform.translation.x = if player.current_hero == 0 {
        HERO_ONE_OFFSET
    } else if player.current_hero == 1 {
        HERO_TWO_OFFSET
    } else {
        HERO_THREE_OFFSET
    };
}

fn hero_health_status(
    hero_query: Query<&Hero>,
    player: Res<Player>,
    mut text_query: Query<(&mut HeroHealthText, &mut Text)>,
) {
    for hero in hero_query.iter() {
        for (marker, mut hero_health_text) in text_query.iter_mut() {
            if hero.0 != marker.0 {
                continue;
            }

            hero_health_text.sections[0].value = format!(
                "HP: {}/{}",
                player.current_hps[hero.0], player.max_hps[hero.0]
            );
        }
    }
}
fn hero_animation(mut hero_query: Query<(&mut Hero, &mut TextureAtlas)>, player: Res<Player>) {
    for (hero, mut texture_atlas) in hero_query.iter_mut() {
        let is_dead = player.current_hps[hero.0] <= 0;
        let is_sleeping = player.sleep_state[hero.0];

        texture_atlas.index = (hero.0 * 2) + (if is_dead || is_sleeping { 1 } else { 0 });
    }
}

fn hero_status(player: Res<Player>, mut text_query: Query<(&mut HeroStatusText, &mut Text)>) {
    for (hero_status_text, mut text) in text_query.iter_mut() {
        if player.current_hps[hero_status_text.0] <= 0 {
            text.sections[0].value = "DEAD".to_string();
            text.sections[0].style.color = Color::default();
        } else if player.sleep_state[hero_status_text.0] {
            text.sections[0].value = "SLEEP".to_string();
            text.sections[0].style.color = Color::default();
        } else {
            text.sections[0].style.color = Color::NONE;
        }
    }
}

fn cleanup_info_panel(mut commands: Commands, node_query: Query<Entity, With<InfoPanelNode>>) {
    for entity_id in node_query.iter() {
        commands.entity(entity_id).despawn_recursive();
    }
}

fn setup_menu(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    target_query: Query<Entity, With<InfoPanelTarget>>,
) {
    let font_handle = asset_server.load("fonts/press_start_2p.ttf");

    let target_entity_id = match target_query.get_single() {
        Ok(e) => e,
        Err(_) => return,
    };

    commands
        .get_entity(target_entity_id)
        .unwrap()
        .with_children(|parent| {
            parent
                .spawn((InfoPanelNode, SpatialBundle::default()))
                .with_children(|parent| {
                    // Options

                    for i in 0..4 {
                        parent.spawn((
                            MenuOption(i),
                            Text2dBundle {
                                text: Text::from_section(
                                    "MenuOption",
                                    TextStyle {
                                        font: font_handle.clone(),
                                        font_size: 20.0,
                                        ..default()
                                    },
                                ),
                                text_anchor: Anchor::CenterLeft,
                                transform: Transform {
                                    translation: Vec3::new(
                                        -(UNIT_SIZE * SCALE_FACTOR * 2.0),
                                        MENU_START_OFFSET - (i as f32 * MENU_BLOCK),
                                        0.0,
                                    ),
                                    ..default()
                                },
                                ..default()
                            },
                        ));
                    }

                    // Cursor
                    parent.spawn((
                        MenuArrow,
                        SpriteBundle {
                            texture: asset_server.load("arrow_right.png"),
                            transform: Transform {
                                scale: SCALE_VEC3,
                                translation: Vec3::new(-(UNIT_SIZE * SCALE_FACTOR * 3.0), 0.0, 0.0),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });
        });
}

fn setup_info(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    target_query: Query<Entity, With<InfoPanelTarget>>,
) {
    let font_handle = asset_server.load("fonts/press_start_2p.ttf");

    let target_entity_id = match target_query.get_single() {
        Ok(e) => e,
        Err(_) => return,
    };

    commands
        .get_entity(target_entity_id)
        .unwrap()
        .with_children(|parent| {
            parent
                .spawn((InfoPanelNode, SpatialBundle::default()))
                .with_children(|parent| {
                    // Info text

                    parent.spawn((
                        BattleInfoText,
                        Text2dBundle {
                            text: Text {
                                linebreak_behavior: bevy::text::BreakLineOn::WordBoundary,
                                justify: JustifyText::Left,
                                sections: vec![TextSection::new(
                                    "",
                                    TextStyle {
                                        font: font_handle.clone(),
                                        font_size: 20.0,
                                        ..default()
                                    },
                                )],
                            },
                            text_2d_bounds: Text2dBounds {
                                size: Vec2::new(
                                    UNIT_SIZE * SCALE_FACTOR * 6.0,
                                    UNIT_SIZE * SCALE_FACTOR * 9.0,
                                ),
                            },
                            text_anchor: Anchor::TopLeft,
                            transform: Transform {
                                translation: Vec3::new(
                                    -(UNIT_SIZE * SCALE_FACTOR * 3.0),
                                    BATTLE_INFO_START_OFFSET,
                                    0.0,
                                ),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });
        });
}

fn menu_cursor(
    mut arrow_query: Query<&mut Transform, With<MenuArrow>>,
    menu_selection: Res<MenuSelection>,
) {
    for mut transform in arrow_query.iter_mut() {
        transform.translation.y = MENU_START_OFFSET - (menu_selection.0 as f32 * MENU_BLOCK);
    }
}

fn menu_cursor_change(keys: Res<ButtonInput<KeyCode>>, mut menu_selection: ResMut<MenuSelection>) {
    if keys.just_pressed(KeyCode::ArrowDown) {
        if menu_selection.0 == 3 {
            menu_selection.0 = 0;
        } else {
            menu_selection.0 += 1;
        }
    }

    if keys.just_pressed(KeyCode::ArrowUp) {
        if menu_selection.0 == 0 {
            menu_selection.0 = 3;
        } else {
            menu_selection.0 -= 1;
        }
    }
}

fn menu_options(mut menu_option_query: Query<(&mut MenuOption, &mut Text)>, player: Res<Player>) {
    for (menu_option, mut text) in menu_option_query.iter_mut() {
        text.sections[0].value = MENU_OPTIONS[player.current_hero][menu_option.0].to_string();
    }
}

fn cycle_hero(player: &mut Player) {
    let mut max_iteration = 100;
    loop {
        max_iteration -= 1;

        if max_iteration == 0 {
            break;
        }

        if player.current_hero == 2 {
            player.current_hero = 0;
        } else {
            player.current_hero += 1;
        }

        if player.current_hps[player.current_hero] <= 0 {
            continue;
        }

        if player.sleep_state[player.current_hero] {
            continue;
        }

        break;
    }
}

fn menu_select(
    mut action_event_writer: EventWriter<ActionEvent>,
    keys: Res<ButtonInput<KeyCode>>,
    menu_selection: Res<MenuSelection>,
    mut next_state: ResMut<NextState<InfoPanelState>>,
    mut player: ResMut<Player>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        // All heroes have cycle
        if menu_selection.0 == 3 {
            cycle_hero(&mut player);
            return;
        }

        let event = ActionEvent::Player(if player.current_hero == 0 {
            // Warrior
            PlayerAction::Warrior(if menu_selection.0 == 0 {
                WarriorAction::Attack
            } else if menu_selection.0 == 1 {
                WarriorAction::Reckless
            } else {
                WarriorAction::Block
            })
        } else if player.current_hero == 1 {
            // Mage
            PlayerAction::Mage(if menu_selection.0 == 0 {
                MageAction::Missle
            } else if menu_selection.0 == 1 {
                MageAction::ShieldWarrior
            } else {
                MageAction::ShieldPriest
            })
        } else {
            // Priest
            PlayerAction::Priest(if menu_selection.0 == 0 {
                PriestAction::HealWarrior
            } else if menu_selection.0 == 1 {
                PriestAction::HealMage
            } else {
                PriestAction::HealSelf
            })
        });

        next_state.set(InfoPanelState::Battle);
        action_event_writer.send(event);
    }
}

fn heal_action(hero: usize, hero_name: &str, player: &mut Player) -> String {
    if player.current_hps[hero] <= 0 {
        return "Priest cannot raise the dead".to_string();
    }

    player.current_hps[hero] += HEAL_AMOUNT as isize;

    if player.current_hps[hero] > player.max_hps[hero] {
        player.current_hps[hero] = player.max_hps[hero];
    }

    format!("Priest heals {} for {}!", hero_name, HEAL_AMOUNT)
}

fn handle_event(
    mut action_event_reader: EventReader<ActionEvent>,
    mut battle_info_timer: ResMut<BattleInfoTimer>,
    mut enemy_query: Query<&mut Enemy>,
    mut next_state: ResMut<NextState<BattleState>>,
    mut player: ResMut<Player>,
    mut info_text_query: Query<&mut Text, With<BattleInfoText>>,
) {
    for event in action_event_reader.read() {
        let mut enemy = enemy_query.single_mut();

        for mut text in info_text_query.iter_mut() {
            text.sections[0].value = match event {
                ActionEvent::Player(player_action) => match player_action {
                    PlayerAction::Mage(mage_action) => match mage_action {
                        MageAction::Missle => {
                            enemy.current_hp -= MAGIC_MISSLE_DAMAGE as isize;

                            format!("Mage cast Magic Missle for {} damage!", MAGIC_MISSLE_DAMAGE)
                        }
                        MageAction::ShieldPriest => {
                            player.shielded[2] = 2;

                            "Mage cast shield on Priest!".to_string()
                        }
                        MageAction::ShieldWarrior => {
                            player.shielded[0] = 2;

                            "Mage cast shield on Warrior!".to_string()
                        }
                    },
                    PlayerAction::Priest(priest_action) => match priest_action {
                        PriestAction::HealMage => heal_action(1, "Mage", &mut player),
                        PriestAction::HealSelf => heal_action(2, "self", &mut player),
                        PriestAction::HealWarrior => heal_action(0, "Warrior", &mut player),
                    },
                    PlayerAction::Warrior(warrior_action) => match warrior_action {
                        WarriorAction::Attack => {
                            enemy.current_hp -= ATTACK_DAMAGE as isize;

                            format!("Warrior attacks for {} damage!", ATTACK_DAMAGE)
                        }
                        WarriorAction::Block => {
                            player.is_warrior_blocking = true;

                            "Warrior blocks!".to_string()
                        }
                        WarriorAction::Reckless => {
                            enemy.current_hp -= RECKLESS_ATTACK_DAMAGE as isize;

                            player.current_hps[0] -= RECKLESS_ATTACK_DAMAGE as isize;

                            format!("Warrior recklessly attacks for {} damage! Also receives the same damage!", RECKLESS_ATTACK_DAMAGE)
                        }
                    },
                },
                ActionEvent::Enemy => {
                    let current_hero = player.current_hero;

                    let mut damage = ENEMY_ATTACK_DAMAGE as isize;

                    if current_hero == 0 && player.is_warrior_blocking {
                        damage -= 2;
                    }

                    if player.shielded[current_hero] > 0 {
                        damage -= 2;
                    }

                    player.current_hps[current_hero] -= damage as isize;

                    format!("Rat attacks for {} damage!", damage)
                }
            };
        }

        if enemy.current_hp <= 0 {
            next_state.set(BattleState::Win);
        } else if player.current_hps[0] <= 0
            && player.current_hps[1] <= 0
            && player.current_hps[2] <= 0
        {
            next_state.set(BattleState::Lose);
        }

        battle_info_timer.0 = Some(Timer::new(Duration::from_secs(3), TimerMode::Once));
    }
}

fn tick_battle_info_timer(
    mut action_event_writer: EventWriter<ActionEvent>,
    mut battle_info_timer: ResMut<BattleInfoTimer>,
    mut info_text_query: Query<&mut Text, With<BattleInfoText>>,
    mut next_battle_state: ResMut<NextState<BattleState>>,
    mut next_info_state: ResMut<NextState<InfoPanelState>>,
    mut player: ResMut<Player>,
    state: Res<State<BattleState>>,
    time: Res<Time>,
) {
    let Some(timer) = battle_info_timer.0.as_mut() else {
        return;
    };

    timer.tick(time.delta());

    if timer.just_finished() {
        match state.into_inner().get() {
            BattleState::Enemy => {
                let current_hero = player.current_hero;

                // Reset things

                if current_hero == 0 {
                    player.is_warrior_blocking = false;
                }

                if player.shielded[current_hero] > 0 {
                    player.shielded[current_hero] -= 1;
                }

                // Sleep, or awaken if all asleep

                player.sleep_state[current_hero] = true;

                if player.sleep_state[0] && player.sleep_state[1] && player.sleep_state[2] {
                    player.sleep_state = [false; 3];
                }

                // Cycle hero
                cycle_hero(&mut player);

                // Next state
                next_battle_state.set(BattleState::Player);
                next_info_state.set(InfoPanelState::Menu);
            }
            BattleState::Player => {
                next_battle_state.set(BattleState::Enemy);
                action_event_writer.send(ActionEvent::Enemy);
            }
            BattleState::Lose => {
                for mut text in info_text_query.iter_mut() {
                    text.sections[0].value = "You lose!".to_string();
                }
            }
            BattleState::Win => {
                for mut text in info_text_query.iter_mut() {
                    text.sections[0].value = "You win!".to_string();
                }
            }
        }
    }
}

fn main() {
    App::new()
        .add_event::<ActionEvent>()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (GAME_WIDTH * SCALE_FACTOR, GAME_HEIGHT * SCALE_FACTOR).into(),
                        ..default()
                    }),
                    ..default()
                }),
            EnemyPlugin,
            EntropyPlugin::<WyRand>::default(),
            MousePlugin,
        ))
        .add_systems(Startup, (setup_cameras, setup_scene, setup_menu).chain())
        .add_systems(OnEnter(InfoPanelState::Battle), setup_info)
        .add_systems(OnEnter(InfoPanelState::Menu), setup_menu)
        .add_systems(OnExit(InfoPanelState::Battle), cleanup_info_panel)
        .add_systems(OnExit(InfoPanelState::Menu), cleanup_info_panel)
        .add_systems(
            Update,
            (
                hero_arrow,
                hero_animation,
                hero_health_status,
                hero_status,
                (handle_event, tick_battle_info_timer).run_if(in_state(InfoPanelState::Battle)),
                (menu_cursor, menu_cursor_change, menu_options, menu_select)
                    .run_if(in_state(InfoPanelState::Menu)),
            ),
        )
        .init_resource::<BattleInfoTimer>()
        .init_resource::<MenuSelection>()
        .init_resource::<Player>()
        .init_state::<BattleState>()
        .init_state::<InfoPanelState>()
        .run();
}
