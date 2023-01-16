use crate::GameState;
use bevy::prelude::*;
use bevy::utils::HashSet;
use serde::{Deserialize, Serialize};

pub struct ConditionPlugin;

impl Plugin for ConditionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveConditions>()
            .add_event::<ConditionAddEvent>()
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(add_conditions));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct ActiveConditions(pub HashSet<ConditionId>);

#[derive(Debug, Clone, Eq, PartialEq, Default, Reflect, Hash, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct ConditionId(pub String);

#[derive(Debug, Clone, Eq, PartialEq, Default, Reflect, Hash, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct ConditionAddEvent(pub ConditionId);

fn add_conditions(
    mut conditions: ResMut<ActiveConditions>,
    mut incoming_conditions: EventReader<ConditionAddEvent>,
) {
    for incoming_condition in incoming_conditions.iter() {
        conditions.0.insert(incoming_condition.0.clone());
    }
}