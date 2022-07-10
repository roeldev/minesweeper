use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum LoadState {
    Loading,
    Loaded,
    Failed,
}

pub type LoadAssets = Vec<HandleUntyped>;

pub struct LoadAssetsPlugin;

impl Plugin for LoadAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(LoadState::Loading);
        app.insert_resource(LoadAssets::new());
        app.add_system_set(
            SystemSet::on_update(LoadState::Loading)
                .with_system(check_load_state)
        );
        app.add_system_set(
            SystemSet::on_exit(LoadState::Loaded)
                .with_system(cleanup)
        );
    }
}

fn check_load_state(
    mut state: ResMut<State<LoadState>>,
    server: Res<AssetServer>,
    assets: Res<LoadAssets>,
) {
    use bevy::asset::LoadState::*;
    match server.get_group_load_state(assets.iter().map(|h| h.id)) {
        Failed => {
            println!("err");
            let _ = state.set(LoadState::Failed);
        }
        Loaded => {
            let _ = state.set(LoadState::Loaded);
        }
        _ => {}
    }
}

fn cleanup(mut cmd: Commands) {
    cmd.remove_resource::<LoadAssets>();
}