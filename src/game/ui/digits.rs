use bevy::prelude::*;

#[derive(Component)]
pub(crate) enum DigitDisplay {
    Time,
    Mines,
}

#[derive(Component)]
pub(crate) struct Digits {
    pub value: u8,
    digits: [Entity; 3],
}

impl Digits {
    pub fn spawn(cmd: &mut Commands, typ: DigitDisplay) -> Entity {
        cmd.spawn_bundle(TransformBundle::default())
            .insert(typ)
            .push_children(&[])
            .id()
    }
}

pub(crate) fn update_digits_display(
    digits_query: Query<&Digits, Changed<Digits>>,
) {}