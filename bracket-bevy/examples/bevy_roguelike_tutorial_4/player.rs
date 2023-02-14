use bevy::prelude::*;

pub fn player_input(keyboard: &Input<KeyCode>) -> (i32, i32) {
    if keyboard.pressed(KeyCode::Left)
        || keyboard.pressed(KeyCode::Numpad4)
        || keyboard.pressed(KeyCode::H)
    {
        (-1, 0)
    } else if keyboard.pressed(KeyCode::Right)
        || keyboard.pressed(KeyCode::Numpad6)
        || keyboard.pressed(KeyCode::L)
    {
        (1, 0)
    } else if keyboard.pressed(KeyCode::Up)
        || keyboard.pressed(KeyCode::Numpad8)
        || keyboard.pressed(KeyCode::K)
    {
        (0, -1)
    } else if keyboard.pressed(KeyCode::Down)
        || keyboard.pressed(KeyCode::Numpad2)
        || keyboard.pressed(KeyCode::J)
    {
        (0, 1)
    } else {
        (0, 0)
    }
}
