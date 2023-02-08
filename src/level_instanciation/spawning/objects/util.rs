use crate::level_instanciation::spawning::{GameObject, PrimedGameObjectSpawner};
use bevy::ecs::system::EntityCommands;
use bevy::gltf::Gltf;
use bevy::prelude::*;

impl<'w, 's, 'a> PrimedGameObjectSpawner<'w, 's, 'a> {
    pub fn spawn_gltf(
        &'a mut self,
        object: GameObject,
        handle: &Handle<Gltf>,
        transform: Transform,
    ) -> EntityCommands<'w, 's, 'a> {
        let gltf = self
            .gltf
            .get(handle)
            .unwrap_or_else(|| panic!("Failed to load scene for {object:?}"));
        self.commands.spawn((
            SceneBundle {
                scene: gltf.scenes[0].clone(),
                transform,
                ..default()
            },
            Name::new(format!("{object:?}")),
        ))
    }
}