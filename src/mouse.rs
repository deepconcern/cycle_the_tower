use bevy::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct MousePosition(pub Vec2);

fn mouse_position(camera_query: Query<(&Camera, &GlobalTransform)>, mut mouse_position: ResMut<MousePosition>, window_query: Query<&Window>) {
    let (camera, camera_transform) = camera_query.single();
    let window = window_query.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    mouse_position.0 = ray.origin.truncate();
}

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_position);
        app.init_resource::<MousePosition>();
    }
}