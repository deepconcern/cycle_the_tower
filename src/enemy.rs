use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub current_hp: isize,
    pub max_hp: isize,
    name: String,
}

impl Enemy {
    pub fn new(name: &str, hp: isize) -> Self {
        Self {
            current_hp: hp,
            max_hp: hp,
            name: name.to_string(),
        }
    }
}

#[derive(Component)]
pub struct EnemyHealthText;

#[derive(Component)]
pub struct EnemyNameText;

fn enemy_health_text(
    mut enemy_health_query: Query<&mut Text, With<EnemyHealthText>>,
    enemy_query: Query<&Enemy>,
) {
    let enemy = enemy_query.single();
    let mut text = enemy_health_query.single_mut();

    text.sections[0].value = format!("HP: {}/{}", enemy.current_hp, enemy.max_hp);
}

fn enemy_name_text(
    mut enemy_name_query: Query<&mut Text, With<EnemyNameText>>,
    enemy_query: Query<&Enemy>,
) {
    let enemy = enemy_query.single();
    let mut text = enemy_name_query.single_mut();

    // Update the name

    text.sections[0].value = enemy.name.to_string();
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (enemy_health_text, enemy_name_text));
    }
}
