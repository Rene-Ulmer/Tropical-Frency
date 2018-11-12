use {HEIGHT, INPUT_STATE, WIDTH};

#[no_mangle]
pub extern "C" fn mm(dx: isize, dy: isize) {
    unsafe {
        INPUT_STATE.mouse_pos.0 += dx;
        if INPUT_STATE.mouse_pos.0 < 0 {
            INPUT_STATE.mouse_pos.0 = 0;
        } else if INPUT_STATE.mouse_pos.0 >= WIDTH as isize {
            INPUT_STATE.mouse_pos.0 = WIDTH as isize - 1;
        }

        INPUT_STATE.mouse_pos.1 += dy;
        if INPUT_STATE.mouse_pos.1 < 0 {
            INPUT_STATE.mouse_pos.1 = 0;
        } else if INPUT_STATE.mouse_pos.1 >= HEIGHT as isize {
            INPUT_STATE.mouse_pos.1 = HEIGHT as isize - 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn md(btn: i32) {
    unsafe {
        if btn >= 0 && btn < 3 {
            INPUT_STATE.mouse_down[btn as usize] = true;
        }
    }
}

#[no_mangle]
pub extern "C" fn mu(btn: i32) {
    unsafe {
        if btn >= 0 && btn < 3 {
            INPUT_STATE.mouse_down[btn as usize] = false;
        }
    }
}

#[no_mangle]
pub extern "C" fn kd(key: i32) {
    unsafe {
        if key >= 0 && key < 256 {
            INPUT_STATE.key_down[key as usize] = true;
        }
    }
}

#[no_mangle]
pub extern "C" fn ku(key: i32) {
    unsafe {
        if key >= 0 && key < 256 {
            INPUT_STATE.key_down[key as usize] = false;
        }
    }
}
