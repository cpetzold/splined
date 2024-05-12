use bevy::{input::InputSystem, prelude::*, window::PrimaryWindow};
use leafwing_input_manager::{
    axislike::DualAxisData, plugin::InputManagerSystem, prelude::*, systems::run_if_enabled,
};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<EditorState>()
            .register_type::<EditorState>()
            .register_type::<EditCursorStart>()
            .add_plugins(InputManagerPlugin::<EditorAction>::default())
            .add_plugins(InputManagerPlugin::<SelectAction>::default())
            .init_resource::<ActionState<EditorAction>>()
            .init_resource::<ActionState<SelectAction>>()
            .insert_resource(EditorAction::default_input_map())
            .insert_resource(SelectAction::default_input_map())
            .insert_resource(EditCursorStart(None))
            .add_systems(
                Update,
                update_cursor_state_from_window
                    .run_if(run_if_enabled::<EditorAction>)
                    .in_set(InputManagerSystem::ManualControl)
                    .before(InputManagerSystem::ReleaseOnDisable)
                    .after(InputManagerSystem::Tick)
                    .after(InputManagerSystem::Update)
                    .after(InputSystem),
            )
            .add_systems(
                Update,
                (
                    update_edit_cursor_start,
                    (
                        update_select.run_if(in_state(EditorState::Select)),
                        move_selected.run_if(in_state(EditorState::Move)),
                    ),
                )
                    .chain(),
            );
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States, Reflect)]
enum EditorState {
    #[default]
    Select,
    Move,
    Rotate,
    Scale,
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
enum EditorAction {
    MousePosition,
}

impl EditorAction {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map
    }
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
enum SelectAction {
    Deselect,
    Move,
    Rotate,
    Scale,
}

impl SelectAction {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert(Self::Deselect, KeyCode::Escape);
        input_map.insert(Self::Move, KeyCode::KeyG);
        input_map.insert(Self::Rotate, KeyCode::KeyR);
        input_map.insert(Self::Scale, KeyCode::KeyS);

        input_map
    }
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq, Hash, Reflect)]
enum MoveAction {
    Revert,
    Commit,
    ConstrainToX,
    ConstrainToY,
}

fn update_select(
    action_state: Res<ActionState<SelectAction>>,
    mut next_edit_state: ResMut<NextState<EditorState>>,
) {
    if action_state.just_pressed(&SelectAction::Move) {
        next_edit_state.set(EditorState::Move);
    }
}

fn move_selected(
    mut selected: Query<(Entity, &mut Transform, Option<&Moving>), With<Selected>>,
    action_state: Res<ActionState<EditorAction>>,
    cursor_start: Res<EditCursorStart>,
    mut commands: Commands,
) {
    let cursor_start = cursor_start.0.unwrap_or_default();
    let cursor_pos = action_state
        .axis_pair(&EditorAction::MousePosition)
        .map(|d| d.xy())
        .unwrap_or_default();
    let delta = cursor_pos - cursor_start;
    for (entity, mut transform, maybe_moving) in selected.iter_mut() {
        let start = match maybe_moving {
            Some(Moving { start_pos }) => *start_pos,
            None => {
                let start_pos = transform.translation.xy();
                commands.entity(entity).insert(Moving { start_pos });
                start_pos
            }
        };
        let new_pos = start + delta;
        transform.translation.x = new_pos.x;
        transform.translation.y = new_pos.y;
    }
}

fn update_edit_cursor_start(
    events: EventReader<StateTransitionEvent<EditorState>>,
    mut edit_cursor_start: ResMut<EditCursorStart>,
    action_state: Res<ActionState<EditorAction>>,
) {
    if !events.is_empty() {
        edit_cursor_start.0 = action_state
            .axis_pair(&EditorAction::MousePosition)
            .map(|d| d.xy());
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct EditCursorStart(Option<Vec2>);

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Moving {
    start_pos: Vec2,
}

fn update_cursor_state_from_window(
    mut action_state: ResMut<ActionState<EditorAction>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let Some((camera, camera_transform)) = camera.iter().find(|c| c.0.is_active) else {
        return;
    };

    for window in window_query.iter() {
        if let Some(val) = window
            .cursor_position()
            .and_then(|p| camera.viewport_to_world_2d(camera_transform, p))
        {
            action_state
                .action_data_mut_or_default(&EditorAction::MousePosition)
                .axis_pair = Some(DualAxisData::from_xy(val));
        }
    }
}
