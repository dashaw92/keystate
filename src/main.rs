use std::fmt;
use x11::xlib::{self, XKeyboardState, Display};

fn main() {
    match display()
        .and_then(kbd_state)
        .and_then(KeyboardState::try_from) {
        Ok(state) => println!("{}", state),
        Err(e) => eprintln!("{}", e),
    }
}

struct KeyboardState {
    caps: bool,
    num: bool,
}

impl fmt::Display for KeyboardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "capslock={};numlock={}", self.caps, self.num)
    }
}

impl TryFrom<*mut XKeyboardState> for KeyboardState {
    type Error = Error;

    fn try_from(kbd: *mut XKeyboardState) -> Result<Self, Self::Error> {
        #[cfg(debug_assertions)]
        eprintln!("KeyboardState::try_from({:?})", kbd);
        if kbd.is_null() {
            return Err(Error::KeyboardNull);
        }

        let led = unsafe {(*kbd).led_mask};
        let caps = led & 1 == 1;
        let num = led & 2 == 2;

        Ok(Self {
            caps,
            num,
        })
    }
}

fn kbd_state(disp: *mut Display) -> Result<*mut XKeyboardState, Error> {
    #[cfg(debug_assertions)]
    eprintln!("kbd_state({:?})", disp);
    let kbd_state = std::mem::MaybeUninit::zeroed().as_mut_ptr();
    unsafe { xlib::XGetKeyboardControl(disp, kbd_state) };
    if kbd_state.is_null() {
        return Err(Error::KeyboardError);
    }

    Ok(kbd_state)
}

fn display() -> Result<*mut Display, Error> {
    #[cfg(debug_assertions)]
    eprintln!("display()");

    let disp = unsafe {
        let ptr = xlib::XOpenDisplay(std::ptr::null());
        if ptr.is_null() {
            return Err(Error::DisplayOpenFailed);
        }
        
        ptr
    };

    Ok(disp)
}

enum Error {
    DisplayOpenFailed,
    KeyboardError,
    KeyboardNull,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Error::DisplayOpenFailed => "Failed to open display",
            Error::KeyboardError => "Failed to grab the keyboard from X11 (xlib::XGetKeyboardControl)",
            Error::KeyboardNull => "Diligence: Keyboard pointer was null, should never happen",
        })
    }
}
