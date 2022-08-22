use bevy::{
    input::mouse::{MouseButton, MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
};

use crate::cut::*;
use crate::io::{QuickLoad, SaveMeshEvent};
use crate::poly::{MakingPolygon, MakingSegment};
use crate::util::*;

use lyon::tessellation::math::Point;

// pub struct StartMakingCutSegment {
//     pub start: Vec2,
// }

// pub struct EndCutSegment {
//     pub end: Vec2,
// }

// pub struct StartMakingPolygon {
//     pub pos: Point,
// }
// pub struct StartMakingSegment {
//     pub pos: Point,
// }
// pub struct EndSegment {
//     pub pos: Point,
// }

pub enum Action {
    StartMakingPolygon { pos: Point },
    EndMakingPolygon,
    StartMakingSegment { pos: Point },
    EndSegment { pos: Point },
    StartMakingCutSegment { start: Vec2 },
    EndCutSegment { end: Vec2 },
    Delete,
}

#[derive(Clone, Copy, Debug)]
pub struct Cursor {
    pub position: Vec2,
    pub pos_relative_to_click: Vec2,
    pub last_click_position: Vec2,
    pub last_right_click_position: Vec2,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            position: Vec2::ZERO,
            pos_relative_to_click: Vec2::ZERO,
            last_click_position: Vec2::ZERO,
            last_right_click_position: Vec2::ZERO,
        }
    }
}

impl Cursor {
    pub fn within_rect(&self, position: Vec2, size: Vec2) -> bool {
        if self.position.x < position.x + size.x / 2.0
            && self.position.x > position.x - size.x / 2.0
            && self.position.y < position.y + size.y / 2.0
            && self.position.y > position.y - size.y / 2.0
        {
            return true;
        }
        return false;
    }
}

impl Into<Point> for Cursor {
    fn into(self) -> Point {
        Point::new(self.position.x, self.position.y)
    }
}

pub fn record_mouse_events_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut cursor_res: ResMut<Cursor>,
    mut windows: ResMut<Windows>,
    cam_transform_query: Query<&Transform, With<OrthographicProjection>>,
    // cam_ortho_query: Query<&OrthographicProjection>,
    // globals: Res<Globals>,
) {
    for event in cursor_moved_events.iter() {
        let cursor_in_pixels = event.position; // lower left is origin
        let window_size = Vec2::new(
            windows.get_primary_mut().unwrap().width(),
            windows.get_primary_mut().unwrap().height(),
        );

        let screen_position = cursor_in_pixels - window_size / 2.0;

        let cam_transform = cam_transform_query.iter().next().unwrap();

        // this variable currently has no effect
        let scale = 1.0;

        // for ortho in cam_ortho_query.iter() {
        //     scale = ortho.scale;
        // }

        let cursor_vec4: Vec4 = cam_transform.compute_matrix()
            * screen_position.extend(0.0).extend(1.0 / (scale))
            * scale;

        let cursor_pos = Vec2::new(cursor_vec4.x, cursor_vec4.y);
        cursor_res.position = cursor_pos;
        cursor_res.pos_relative_to_click = cursor_res.position - cursor_res.last_click_position;
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        cursor_res.last_click_position = cursor_res.position;
        cursor_res.pos_relative_to_click = Vec2::ZERO;
    }

    if mouse_button_input.just_pressed(MouseButton::Right) {
        cursor_res.last_right_click_position = cursor_res.position;
    }
}

pub fn direct_make_polygon_action(
    mut commands: Commands,
    // mut action_event_writer: EventWriter<Action>,
    making_poly_query: Query<&MakingPolygon>,
    making_cut_query: Query<(Entity, &MakingCutSegment)>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    // mut mouse_wheel_events: EventReader<MouseWheel>,
    mut quickload_event_writer: EventWriter<QuickLoad>,

    // mut start_polygon: EventWriter<StartMakingPolygon>,
    // mut start_segment: EventWriter<EndSegment>,
    // mut start_cut_segment: EventWriter<StartMakingCutSegment>,
    // mut end_polygon: EventWriter<EndMakingPolygon>,
    // mut delete_event: EventWriter<DeleteEvent>,
    mut action_event: EventWriter<Action>,
    mut quicksave_event_writer: EventWriter<SaveMeshEvent>,
    // mut end_cut_segment: EventWriter<EndCutSegment>,
    cursor: Res<Cursor>,
) {
    let mouse_pressed = mouse_button_input.pressed(MouseButton::Left);

    let mouse_just_pressed = mouse_button_input.just_pressed(MouseButton::Left);
    let mouse_right_just_pressed = mouse_button_input.just_pressed(MouseButton::Right);

    // let mut mouse_wheel_up = false;
    // let mut mouse_wheel_down = false;
    // if let Some(mouse_wheel) = mouse_wheel_events.iter().next() {
    //     if mouse_wheel.y > 0.5 {
    //         mouse_wheel_up = true;
    //     }
    //     if mouse_wheel.y < -0.5 {
    //         mouse_wheel_down = true;
    //     }
    // }

    // only used for pattern matching
    let _pressed_g = keyboard_input.just_pressed(KeyCode::G);
    let _pressed_h = keyboard_input.just_pressed(KeyCode::H);
    let pressed_s = keyboard_input.just_pressed(KeyCode::S);
    let pressed_l = keyboard_input.just_pressed(KeyCode::L);
    let _pressed_z = keyboard_input.just_pressed(KeyCode::Z);
    let _pressed_t = keyboard_input.just_pressed(KeyCode::T);
    let pressed_delete = keyboard_input.just_pressed(KeyCode::Delete);
    let pressed_enter = keyboard_input.just_pressed(KeyCode::Return);
    let pressed_escape = keyboard_input.just_pressed(KeyCode::Escape);
    let pressed_space = keyboard_input.just_pressed(KeyCode::Space);

    // match keys / mouse buttons / mouse wheel combination and send event to corresponding action
    match (
        keyboard_input.pressed(KeyCode::LShift),
        keyboard_input.pressed(KeyCode::LControl),
        keyboard_input.pressed(KeyCode::Space),
    ) {
        //
        //
        //
        ////////////// if currently making either a polygon or a cut segment  /////////////////////////////
        //
        //
        //
        (false, false, _)
            if ((pressed_enter || mouse_right_just_pressed || pressed_space)
                && making_poly_query.iter().count() == 1) =>
        {
            action_event.send(Action::EndMakingPolygon);
        }

        // a click ends the current segment
        (false, false, false) if (mouse_just_pressed && making_poly_query.iter().count() == 1) => {
            action_event.send(Action::EndSegment {
                pos: cursor.clone().into(),
            });
        }

        (false, false, false) if mouse_just_pressed && making_cut_query.iter().count() == 1 => {
            action_event.send(Action::EndCutSegment {
                end: cursor.position,
            });
        }
        //
        //
        //
        ////////////// if currently making either a polygon or a cut segment  /////////////////////////////
        //
        //
        //
        //
        //
        //
        //
        (false, true, false) if pressed_s => quicksave_event_writer.send(SaveMeshEvent),
        (false, true, false) if pressed_l => quickload_event_writer.send(QuickLoad),
        (false, false, false) if _pressed_g => {}

        (false, false, false) if pressed_escape && making_cut_query.iter().count() == 1 => {
            // delete cut segment
            let (entity, _) = making_cut_query.single();
            commands.entity(entity).despawn();
        }
        (true, true, false) if _pressed_g => {}
        (false, true, false) if _pressed_h => {}
        (true, true, false) if _pressed_h => {}
        (false, true, false) if _pressed_z => {}
        (true, true, false) if _pressed_z => {}
        // (false, true, false) if mouse_wheel_up => {}
        // (false, true, false) if mouse_wheel_down => {}
        (true, false, false) if _pressed_t => {}

        (false, false, false) if pressed_delete || pressed_escape => {
            action_event.send(Action::Delete);
        }

        // cannot start a polygon if one is already being made
        (true, false, false) if (mouse_just_pressed && making_poly_query.iter().count() == 0) => {
            action_event.send(Action::StartMakingPolygon {
                pos: cursor.clone().into(),
            })
        }

        // cannot start a cut segment if one is already being made
        (false, true, false) if mouse_just_pressed && making_cut_query.iter().count() == 0 => {
            action_event.send(Action::StartMakingCutSegment {
                start: cursor.position,
            });
        }

        _ => {}
    }
}

pub fn direct_release_action(
    mut commands: Commands,
    segment_query: Query<Entity, With<MakingSegment>>,
    mouse_button_input: Res<Input<MouseButton>>,
    // mut start_polygon: EventWriter<StartMakingPolygon>,
    // mut action_event_writer: EventWriter<Action>,
    // keyboard_input: Res<Input<KeyCode>>,
    // mut mouse_wheel_events: EventReader<MouseWheel>,
    // cursor: Res<Cursor>,
) {
    if mouse_button_input.just_released(MouseButton::Left) {
        // delete MakingSegment if it exists
        for entity in segment_query.iter() {
            commands.entity(entity).remove::<MakingSegment>();
        }
    }
}
