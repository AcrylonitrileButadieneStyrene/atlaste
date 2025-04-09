use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component, Default)]
pub struct Popup(pub f32, pub f32);

#[derive(Component)]
pub struct PopupHeader;

#[derive(Event)]
pub struct PopupClosed;

// TODO: decide if the window's position should be reset to its initial position when reopening or not.
pub fn spawn(
    commands: &mut Commands,
    title: &str,
    font: Handle<Font>,
    x: f32,
    y: f32,
) -> (Entity, Entity) {
    let header = spawn_header(commands, title, font);
    let body = commands
        .spawn((
            Node {
                flex_grow: 1.0,
                min_height: Val::ZERO,
                ..Default::default()
            },
            BackgroundColor(Color::hsl(0., 0., 0.1)),
        ))
        .id();

    (
        commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    width: Val::Px(324.0),
                    height: Val::Px(260.0),
                    top: Val::Px(y),
                    left: Val::Px(x),
                    padding: UiRect::all(Val::Px(1.)),
                    ..Default::default()
                },
                Visibility::Hidden,
                ZIndex(1),
                BackgroundColor(Color::hsl(0., 0., 0.4)),
                Popup(x, y),
            ))
            .insert_children(0, &[header, body])
            .id(),
        body,
    )
}

pub fn spawn_header(commands: &mut Commands, title: &str, font: Handle<Font>) -> Entity {
    commands
        .spawn((
            Node {
                height: Val::Px(16.0),
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            BackgroundColor(Color::hsl(0., 0., 0.05)),
            PopupHeader,
        ))
        .with_children(|children| {
            children.spawn((
                Text::new(title),
                TextFont::from_font(font.clone()).with_font_size(16.0),
                PickingBehavior::IGNORE,
            ));
            children.spawn((
                Node {
                    flex_grow: 1.0,
                    ..Default::default()
                },
                PickingBehavior::IGNORE,
            ));
            children
                .spawn((
                    Node {
                        width: Val::Px(16.0),
                        height: Val::Px(16.0),
                        ..Default::default()
                    },
                    Button,
                    BackgroundColor(Color::srgb_u8(0xFF, 0, 0)),
                ))
                .observe(close);
        })
        .observe(promote)
        .observe(drag)
        .id()
}

pub fn promote(
    trigger: Trigger<Pointer<Click>>,
    mut query: Query<(Entity, &mut ZIndex), With<Popup>>,
) {
    let target = trigger.entity();
    query
        .iter_mut()
        .for_each(|(entity, mut index)| index.0 = if target == entity { 2 } else { 1 });
}

pub fn drag(trigger: Trigger<Pointer<Drag>>, parent: Query<&Parent>, mut popup: Query<&mut Popup>) {
    let Ok(mut popup) = parent
        .get(trigger.entity())
        .and_then(|parent| popup.get_mut(parent.get()))
    else {
        return;
    };

    popup.0 = popup.0 + trigger.delta.x;
    popup.1 = popup.1 + trigger.delta.y;
}

pub fn propagate(
    mut query: Query<(&mut Node, &Popup), Changed<Popup>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.get_single() else {
        log::error!("Primary windows != 1");
        return;
    };

    query.par_iter_mut().for_each(|(mut node, popup)| {
        let convert = |x| if let Val::Px(x) = x { x } else { 0.0 };
        node.left = Val::Px(popup.0.clamp(0., window.width() - convert(node.width)));
        node.top = Val::Px(popup.1.clamp(0., window.height() - convert(node.height)));
    });
}

pub fn close(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    parent: Query<&Parent>,
    mut visibility: Query<&mut Visibility>,
) {
    if let Ok((entity, mut visibility)) = parent
        .get(trigger.entity())
        .and_then(|ent| parent.get(ent.get()))
        .and_then(|ent| {
            visibility
                .get_mut(ent.get())
                .map(|visibility| (ent.get(), visibility))
        })
    {
        commands.trigger_targets(PopupClosed, entity);
        *visibility = Visibility::Hidden;
    }
}
