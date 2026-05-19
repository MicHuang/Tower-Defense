use bevy::prelude::*;
use crate::resources::game_state::GameState;

#[derive(Component)]
pub struct MenuRoot;

pub fn spawn_menu(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            node: Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        MenuRoot,
    )).with_children(|parent| {
        // Title
        parent.spawn((
            Text::new("TOWER DEFENSE"),
            TextFont {
                font_size: 64.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 1.0)), // WHITE
        ));

        // Play button
        parent.spawn((
            Text::new("[ PLAY ]"),
            TextFont {
                font_size: 36.0,
                ..default()
            },
            TextColor(Color::srgb(0.0, 1.0, 0.0)), // GREEN
            Node {
                margin: UiRect::top(Val::Px(40.0)),
                ..default()
            },
            PlayButton,
        ));
    });
}

#[derive(Component)]
pub struct PlayButton;

pub fn despawn_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuRoot>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn play_button_clicked(
    mut next_state: ResMut<NextState<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}
