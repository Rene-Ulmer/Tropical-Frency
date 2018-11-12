use INPUT_STATE;
use LAST_INPUT_STATE;

pub fn key_just_pressed(key: u8) -> bool {
    unsafe { INPUT_STATE.key_down[key as usize] && !LAST_INPUT_STATE.key_down[key as usize] }
}

pub fn key_just_released(key: u8) -> bool {
    unsafe { !INPUT_STATE.key_down[key as usize] && LAST_INPUT_STATE.key_down[key as usize] }
}

pub fn mouse_just_pressed(key: u8) -> bool {
    unsafe { INPUT_STATE.mouse_down[key as usize] && !LAST_INPUT_STATE.mouse_down[key as usize] }
}

pub fn mouse_just_released(key: u8) -> bool {
    unsafe { !INPUT_STATE.mouse_down[key as usize] && LAST_INPUT_STATE.mouse_down[key as usize] }
}
