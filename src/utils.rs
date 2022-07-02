use bevy::prelude::IVec2;
use winit::dpi::PhysicalPosition;

pub fn center_window(window: &winit::window::Window) -> Option<IVec2> {
    let monitor = window.current_monitor()?;
    let monitor_size = monitor.size();
    let monitor_position = monitor.position();
    let window_size = window.outer_size();

    let position = IVec2::new(
        monitor_position.x + ((monitor_size.width - window_size.width) as i32 / 2),
        monitor_position.y + ((monitor_size.height - window_size.height) as i32 / 2),
    );
    window.set_outer_position(PhysicalPosition::new(position.x, position.y));

    return Some(position);
}
