use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowThemeChanged},
};

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup).add_systems(
            Update,
            (
                update,
                // (
                //     apply::<Added<Themed>>,
                //     apply::<()>.run_if(resource_changed::<CurrentTheme>),
                // ),
            )
                .chain(),
        );
    }
}

pub fn startup(
    window: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut events: MessageWriter<WindowThemeChanged>,
) -> Result {
    let (entity, window) = window.single()?;
    if let Some(theme) = window.window_theme {
        events.write(WindowThemeChanged {
            window: entity,
            theme,
        });
    }
    Ok(())
}

pub fn update(mut _commands: Commands, mut events: MessageReader<WindowThemeChanged>) {
    let Some(_event) = events.read().last() else {
        return;
    };

    // let theme = match event.theme {
    //     WindowTheme::Dark => DARK,
    //     WindowTheme::Light => LIGHT,
    // };

    // commands.insert_resource(ClearColor(theme.clear_color));
    // commands.insert_resource(CurrentTheme(theme));
}

// pub fn apply<T: bevy::ecs::query::QueryFilter>(
//     mut commands: Commands,
//     theme: Option<Res<CurrentTheme>>,
//     query: Query<(Entity, &Themed), T>,
// ) {
//     let Some(theme) = theme else {
//         return;
//     };

//     let theme = &theme.0;
//     for (entity, themed) in query.iter() {
//         let Some(mut entity) = commands.get_entity(entity) else {
//             continue;
//         };

//         let components = &themed.0;
//         if components.contains(ThemedComponents::BACKGROUND) {
//             entity.insert(BackgroundColor(theme.background));
//         }
//         if components.contains(ThemedComponents::TEXT) {
//             entity.insert(TextColor(theme.text));
//         }
//     }
// }

// #[derive(Component)]
// pub struct Themed(pub ThemedComponents);

// bitflags::bitflags! {
//     pub struct ThemedComponents: u8 {
//         const TEXT = 1 << 0;
//         const BACKGROUND = 1 << 0;
//     }
// }

// #[derive(Resource)]
// pub struct CurrentTheme(pub Theme);

// pub struct Theme {
//     pub clear_color: Color,
//     pub text: Color,

//     pub background000: Color,
//     pub background100: Color,
//     pub background200: Color,
//     pub background300: Color,
//     pub background400: Color,
//     pub background500: Color,
//     pub background600: Color,
//     pub background700: Color,
//     pub background800: Color,
//     pub background900: Color,
// }

// const DARK: Theme = Theme {
//     clear_color: Color::hsl(0., 0., 0.05),
//     text: Color::hsl(0., 0., 0.75),

//     background000: Color::hsl(0., 0., 0.),
//     background100: Color::hsl(0., 0., 0.05),
//     background200: Color::hsl(0., 0., 0.10),
//     background300: Color::hsl(0., 0., 0.20),
//     background400: Color::hsl(0., 0., 0.30),
//     background500: Color::hsl(0., 0., 0.40),
//     background600: Color::hsl(0., 0., 0.50),
//     background700: Color::hsl(0., 0., 0.60),
//     background800: Color::hsl(0., 0., 0.70),
//     background900: Color::hsl(0., 0., 0.80),
// };

// const LIGHT: Theme = Theme {
//     clear_color: Color::hsl(0., 0., 0.75),
//     text: Color::hsl(0., 0., 0.05),

//     background000: Color::hsl(0., 0., 1.),
//     background100: Color::hsl(0., 0., 0.95),
//     background200: Color::hsl(0., 0., 0.90),
//     background300: Color::hsl(0., 0., 0.80),
//     background400: Color::hsl(0., 0., 0.70),
//     background500: Color::hsl(0., 0., 0.60),
//     background600: Color::hsl(0., 0., 0.50),
//     background700: Color::hsl(0., 0., 0.40),
//     background800: Color::hsl(0., 0., 0.30),
//     background900: Color::hsl(0., 0., 0.20),
// };
