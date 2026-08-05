use bevy::prelude::*;
use shared::{AppConfig, CursorBlink, set_title, spawn_text_field};

use crate::structs::LauncherState;


#[derive(Resource, Default)]
pub struct DirectForm {
    pub address: String,
    pub token: String,
    pub focused: FormField,
}
#[derive(Default, PartialEq)]
pub enum FormField {
    #[default]
    Address,
    Token,
}

#[derive(Component)]
pub struct AddressInput;

#[derive(Component)]
pub struct AddressText;

#[derive(Component)]
pub struct AddressInputFieldCursor;

#[derive(Component)]
pub struct TokenInput;

#[derive(Component)]
pub struct TokenText;

#[derive(Component)]
pub struct TokenInputFieldCursor;
#[derive(Component)]
pub struct ConnectButton;






pub fn setup_direct_match_join_menu_ui(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
) {
    let launcher_title = "Launcher";

    if let Ok(mut window) = windows.single_mut() {
        set_title(&mut window, launcher_title);
        window.resolution.set(480.0, 320.0);
    };

   
    commands.init_resource::<DirectForm>();
    commands.init_resource::<CursorBlink>();

    // Root
    commands
        .spawn((
            DespawnOnExit(LauncherState::DirectMatchJoinMenu),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(12.0),
                ..default()
            },
        ))
        .with_children(|p| {
            spawn_text_field(
                p,
                "Address",
                AddressInput,
                AddressText,
                AddressInputFieldCursor,
                LauncherState::Login,
            );
            spawn_text_field(
                p,
                "Token",
                TokenInput,
                TokenText,
                TokenInputFieldCursor,
                LauncherState::Login,
            );


            

            p.spawn((
                ConnectButton,
                Button,
                Node {
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(10.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.6, 1.0)),
            ))
            .with_child(Text::new("Connect!"));

        });
}


pub fn add_launcher_direct_match_join_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            handle_text_input,
            blink_cursor,
            handle_connect_button,
        )
            .chain()
            .run_if(in_state(LauncherState::DirectMatchJoinMenu)),
    );
}


pub fn handle_text_input(
    mut form: ResMut<DirectForm>,
    mut evr: MessageReader<bevy::input::keyboard::KeyboardInput>,
    address_field_interaction_query: Query<&Interaction, With<AddressInput>>,
    token_field_interaction_query: Query<&Interaction, With<TokenInput>>,
    anywhere_else_interaction_query: Query<
        &Interaction,
        (Without<AddressInput>, Without<TokenInput>),
    >,
    mut address_text: Query<&mut Text, (With<AddressText>, Without<TokenText>)>,
    mut token_text: Query<&mut Text, (With<TokenText>, Without<AddressText>)>,
) {
    let address_pressed = address_field_interaction_query
        .single()
        .is_ok_and(|i| *i == Interaction::Pressed);
    let token_pressed = token_field_interaction_query
        .single()
        .is_ok_and(|i| *i == Interaction::Pressed);
    let anywhere_else_pressed = anywhere_else_interaction_query
        .single()
        .is_ok_and(|i| *i == Interaction::Pressed);

    if address_pressed {
        form.focused = FormField::Address;
        soft_keyboard_input::show_soft_keyboard_if_needed(&form.address, "Address");
    } else if token_pressed {
        form.focused = FormField::Token;
        soft_keyboard_input::show_soft_keyboard_if_needed(&form.token, "Token");
    } else if anywhere_else_pressed {
        soft_keyboard_input::hide_soft_keyboard_if_needed();
    }

    for ev in evr.read() {
        if ev.state != bevy::input::ButtonState::Pressed {
            continue;
        }
        if ev.logical_key == bevy::input::keyboard::Key::Backspace {
            let target = match form.focused {
                FormField::Address => &mut form.address,
                FormField::Token => &mut form.token,
            };
            target.pop();
            continue;
        }

        // do not want to put "enter" into the text fields
        if ev.logical_key == bevy::input::keyboard::Key::Enter {
            continue;
        }

        // Use the `text` field for printable characters
        if let Some(ref s) = ev.text {
            let target = match form.focused {
                FormField::Address => &mut form.address,
                FormField::Token => &mut form.token,
            };
            target.push_str(s);
        }
    }

    // Sync to UI - now these queries will actually find the entities
    if let Ok(mut text) = address_text.single_mut() {
        text.0 = form.address.clone();
    }
    if let Ok(mut text) = token_text.single_mut() {
        text.0 = "*".repeat(form.token.len());
    }
}





pub fn blink_cursor(
    time: Res<Time>,
    mut blink: ResMut<CursorBlink>,
    form: Res<DirectForm>,
    mut address_cursor: Query<
        &mut Visibility,
        (
            With<AddressInputFieldCursor>,
            Without<TokenInputFieldCursor>,
        ),
    >,
    mut token_cursor: Query<
        &mut Visibility,
        (
            With<TokenInputFieldCursor>,
            Without<AddressInputFieldCursor>,
        ),
    >,
) {
    blink.0.tick(time.delta());

    let (mut active, mut inactive) = match form.focused {
        FormField::Address => (
            address_cursor.single_mut().unwrap(),
            token_cursor.single_mut().unwrap(),
        ),
        FormField::Token => (
            token_cursor.single_mut().unwrap(),
            address_cursor.single_mut().unwrap(),
        ),
    };

    // Inactive field cursor always hidden
    *inactive = Visibility::Hidden;

    // Active field cursor blinks
    if blink.0.just_finished() {
        *active = match *active {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}



pub fn handle_connect_button(
    _commands: Commands,
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<ConnectButton>)>,
    form: Res<DirectForm>,
    config: Res<AppConfig>,
) {
    for interaction in &interaction_q {
        let _config = config.clone();
        if *interaction != Interaction::Pressed {
            continue;
        }

        soft_keyboard_input::hide_soft_keyboard_if_needed();

        let (address, token) = (form.address.clone(), form.token.clone());
        debug!("Addres: {address}, token: {token}");
    }
}
