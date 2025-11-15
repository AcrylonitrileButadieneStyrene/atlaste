use bevy::prelude::*;

#[derive(Resource)]
pub struct UnitRectangle(pub Handle<Mesh>);

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(UnitRectangle(meshes.add(Rectangle::new(1., 1.))));
}
