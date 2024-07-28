use bevy::{
    input::common_conditions::{input_just_pressed, input_just_released},
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

use crate::mouse::MousePosition;

#[derive(Component, Debug)]
pub enum Draggable {
    Aabb(Aabb2d),
    BoundingCircle(BoundingCircle),
}

impl Draggable {
    pub fn set_center(&mut self, center: Vec2) {
        match self {
            Draggable::Aabb(aabb) => {
                let half_size = aabb.half_size();

                aabb.max = center + half_size;
                aabb.min = center - half_size;
            },
            Draggable::BoundingCircle(bounding_circle) => {
                bounding_circle.center = center;
            },
        }
    }
}

impl IntersectsVolume<Aabb2d> for Draggable {
    fn intersects(&self, volume: &Aabb2d) -> bool {
        match self {
            Self::Aabb(aabb) => aabb.intersects(volume),
            Self::BoundingCircle(bouding_circle) => bouding_circle.intersects(volume),
        }
    }
}

impl IntersectsVolume<BoundingCircle> for Draggable {
    fn intersects(&self, volume: &BoundingCircle) -> bool {
        match self {
            Self::Aabb(aabb) => aabb.intersects(volume),
            Self::BoundingCircle(bouding_circle) => bouding_circle.intersects(volume),
        }
    }
}

#[derive(Debug, Event)]
pub struct DragEvent {
    pub draggable_id: Entity,
    pub draggable_position: Vec2,
}

#[derive(Resource)]
pub struct DragState {
    pub current_entity: Option<Entity>,
}

impl Default for DragState {
    fn default() -> Self {
        Self {
            current_entity: None,
        }
    }
}

fn drag(drag_state: Res<DragState>, mut draggable_query: Query<&mut Transform>, mouse_position: Res<MousePosition>) {
    let Some(dragging_entity_id) = drag_state.current_entity else {
        return;
    };

    let mut transform = draggable_query.get_mut(dragging_entity_id).unwrap();

    transform.translation = mouse_position.0.extend(0.0);
}

fn handle_draggable_press(
    mut drag_state: ResMut<DragState>,
    draggable_query: Query<(&Draggable, Entity)>,
    mouse_position: Res<MousePosition>,
) {
    for (draggable, entity_id) in draggable_query.iter() {
        if !draggable.intersects(&BoundingCircle::new(mouse_position.0, 2.0)) {
            continue;
        }

        drag_state.current_entity = Some(entity_id);
    }
}

fn handle_draggable_release(
    mut drag_event_writer: EventWriter<DragEvent>,
    mut drag_state: ResMut<DragState>,
    mouse_position: Res<MousePosition>,
) {
    let Some(entity_id) = drag_state.current_entity else {
        return;
    };

    drag_state.current_entity = None;

    drag_event_writer.send(DragEvent {
        draggable_id: entity_id,
        draggable_position: mouse_position.0.clone(),
    });
}

pub struct DragPlugin;

impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DragEvent>();
        app.add_systems(
            Update,
            (
                drag,
                handle_draggable_press.run_if(input_just_pressed(MouseButton::Left)),
                handle_draggable_release.run_if(input_just_released(MouseButton::Left)),
            ),
        );
        app.init_resource::<DragState>();
    }
}
