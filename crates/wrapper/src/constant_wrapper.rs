use pyo3::prelude::*;

pub fn add_module_constants(m: &PyModule) -> PyResult<()> {
    macro_rules! add_constant {
        ($name: ident) => {
            m.add(stringify!($name), pyxel::$name)
        };
    }

    // Settings
    add_constant!(PYXEL_VERSION)?;

    add_constant!(APPLICATION_FILE_EXTENSION)?;
    add_constant!(RESOURCE_FILE_EXTENSION)?;
    add_constant!(RESOURCE_ARCHIVE_DIRNAME)?;

    add_constant!(NUM_COLORS)?;
    add_constant!(NUM_IMAGES)?;
    add_constant!(IMAGE_SIZE)?;
    add_constant!(NUM_TILEMAPS)?;
    add_constant!(TILEMAP_SIZE)?;
    add_constant!(TILE_SIZE)?;
    add_constant!(COLOR_BLACK)?;
    add_constant!(COLOR_NAVY)?;
    add_constant!(COLOR_PURPLE)?;
    add_constant!(COLOR_GREEN)?;
    add_constant!(COLOR_BROWN)?;
    add_constant!(COLOR_DARK_BLUE)?;
    add_constant!(COLOR_LIGHT_BLUE)?;
    add_constant!(COLOR_WHITE)?;
    add_constant!(COLOR_RED)?;
    add_constant!(COLOR_ORANGE)?;
    add_constant!(COLOR_YELLOW)?;
    add_constant!(COLOR_LIME)?;
    add_constant!(COLOR_CYAN)?;
    add_constant!(COLOR_GRAY)?;
    add_constant!(COLOR_PINK)?;
    add_constant!(COLOR_PEACH)?;
    add_constant!(FONT_WIDTH)?;
    add_constant!(FONT_HEIGHT)?;

    add_constant!(NUM_CHANNELS)?;
    add_constant!(NUM_SOUNDS)?;
    add_constant!(NUM_MUSICS)?;
    add_constant!(TONE_TRIANGLE)?;
    add_constant!(TONE_SQUARE)?;
    add_constant!(TONE_PULSE)?;
    add_constant!(TONE_NOISE)?;
    add_constant!(EFFECT_NONE)?;
    add_constant!(EFFECT_SLIDE)?;
    add_constant!(EFFECT_VIBRATO)?;
    add_constant!(EFFECT_FADEOUT)?;

    // Key
    add_constant!(KEY_NONE)?;
    add_constant!(KEY_A)?;
    add_constant!(KEY_B)?;
    add_constant!(KEY_C)?;
    add_constant!(KEY_D)?;
    add_constant!(KEY_E)?;
    add_constant!(KEY_F)?;
    add_constant!(KEY_G)?;
    add_constant!(KEY_H)?;
    add_constant!(KEY_I)?;
    add_constant!(KEY_J)?;
    add_constant!(KEY_K)?;
    add_constant!(KEY_L)?;
    add_constant!(KEY_M)?;
    add_constant!(KEY_N)?;
    add_constant!(KEY_O)?;
    add_constant!(KEY_P)?;
    add_constant!(KEY_Q)?;
    add_constant!(KEY_R)?;
    add_constant!(KEY_S)?;
    add_constant!(KEY_T)?;
    add_constant!(KEY_U)?;
    add_constant!(KEY_V)?;
    add_constant!(KEY_W)?;
    add_constant!(KEY_X)?;
    add_constant!(KEY_Y)?;
    add_constant!(KEY_Z)?;
    add_constant!(KEY_1)?;
    add_constant!(KEY_2)?;
    add_constant!(KEY_3)?;
    add_constant!(KEY_4)?;
    add_constant!(KEY_5)?;
    add_constant!(KEY_6)?;
    add_constant!(KEY_7)?;
    add_constant!(KEY_8)?;
    add_constant!(KEY_9)?;
    add_constant!(KEY_0)?;
    add_constant!(KEY_RETURN)?;
    add_constant!(KEY_ESCAPE)?;
    add_constant!(KEY_BACKSPACE)?;
    add_constant!(KEY_TAB)?;
    add_constant!(KEY_SPACE)?;
    add_constant!(KEY_MINUS)?;
    add_constant!(KEY_EQUALS)?;
    add_constant!(KEY_LEFTBRACKET)?;
    add_constant!(KEY_RIGHTBRACKET)?;
    add_constant!(KEY_BACKSLASH)?;
    add_constant!(KEY_NONUSHASH)?;
    add_constant!(KEY_SEMICOLON)?;
    add_constant!(KEY_APOSTROPHE)?;
    add_constant!(KEY_GRAVE)?;
    add_constant!(KEY_COMMA)?;
    add_constant!(KEY_PERIOD)?;
    add_constant!(KEY_SLASH)?;
    add_constant!(KEY_CAPSLOCK)?;
    add_constant!(KEY_F1)?;
    add_constant!(KEY_F2)?;
    add_constant!(KEY_F3)?;
    add_constant!(KEY_F4)?;
    add_constant!(KEY_F5)?;
    add_constant!(KEY_F6)?;
    add_constant!(KEY_F7)?;
    add_constant!(KEY_F8)?;
    add_constant!(KEY_F9)?;
    add_constant!(KEY_F10)?;
    add_constant!(KEY_F11)?;
    add_constant!(KEY_F12)?;
    add_constant!(KEY_PRINTSCREEN)?;
    add_constant!(KEY_SCROLLLOCK)?;
    add_constant!(KEY_PAUSE)?;
    add_constant!(KEY_INSERT)?;
    add_constant!(KEY_HOME)?;
    add_constant!(KEY_PAGEUP)?;
    add_constant!(KEY_DELETE)?;
    add_constant!(KEY_END)?;
    add_constant!(KEY_PAGEDOWN)?;
    add_constant!(KEY_RIGHT)?;
    add_constant!(KEY_LEFT)?;
    add_constant!(KEY_DOWN)?;
    add_constant!(KEY_UP)?;
    add_constant!(KEY_NUMLOCKCLEAR)?;
    add_constant!(KEY_KP_DIVIDE)?;
    add_constant!(KEY_KP_MULTIPLY)?;
    add_constant!(KEY_KP_MINUS)?;
    add_constant!(KEY_KP_PLUS)?;
    add_constant!(KEY_KP_ENTER)?;
    add_constant!(KEY_KP_1)?;
    add_constant!(KEY_KP_2)?;
    add_constant!(KEY_KP_3)?;
    add_constant!(KEY_KP_4)?;
    add_constant!(KEY_KP_5)?;
    add_constant!(KEY_KP_6)?;
    add_constant!(KEY_KP_7)?;
    add_constant!(KEY_KP_8)?;
    add_constant!(KEY_KP_9)?;
    add_constant!(KEY_KP_0)?;
    add_constant!(KEY_KP_PERIOD)?;
    add_constant!(KEY_NONUSBACKSLASH)?;
    add_constant!(KEY_APPLICATION)?;
    add_constant!(KEY_POWER)?;
    add_constant!(KEY_KP_EQUALS)?;
    add_constant!(KEY_F13)?;
    add_constant!(KEY_F14)?;
    add_constant!(KEY_F15)?;
    add_constant!(KEY_F16)?;
    add_constant!(KEY_F17)?;
    add_constant!(KEY_F18)?;
    add_constant!(KEY_F19)?;
    add_constant!(KEY_F20)?;
    add_constant!(KEY_F21)?;
    add_constant!(KEY_F22)?;
    add_constant!(KEY_F23)?;
    add_constant!(KEY_F24)?;
    add_constant!(KEY_EXECUTE)?;
    add_constant!(KEY_HELP)?;
    add_constant!(KEY_MENU)?;
    add_constant!(KEY_SELECT)?;
    add_constant!(KEY_STOP)?;
    add_constant!(KEY_AGAIN)?;
    add_constant!(KEY_UNDO)?;
    add_constant!(KEY_CUT)?;
    add_constant!(KEY_COPY)?;
    add_constant!(KEY_PASTE)?;
    add_constant!(KEY_FIND)?;
    add_constant!(KEY_MUTE)?;
    add_constant!(KEY_VOLUMEUP)?;
    add_constant!(KEY_VOLUMEDOWN)?;
    add_constant!(KEY_KP_COMMA)?;
    add_constant!(KEY_KP_EQUALSAS400)?;
    add_constant!(KEY_INTERNATIONAL1)?;
    add_constant!(KEY_INTERNATIONAL2)?;
    add_constant!(KEY_INTERNATIONAL3)?;
    add_constant!(KEY_INTERNATIONAL4)?;
    add_constant!(KEY_INTERNATIONAL5)?;
    add_constant!(KEY_INTERNATIONAL6)?;
    add_constant!(KEY_INTERNATIONAL7)?;
    add_constant!(KEY_INTERNATIONAL8)?;
    add_constant!(KEY_INTERNATIONAL9)?;
    add_constant!(KEY_LANG1)?;
    add_constant!(KEY_LANG2)?;
    add_constant!(KEY_LANG3)?;
    add_constant!(KEY_LANG4)?;
    add_constant!(KEY_LANG5)?;
    add_constant!(KEY_LANG6)?;
    add_constant!(KEY_LANG7)?;
    add_constant!(KEY_LANG8)?;
    add_constant!(KEY_LANG9)?;
    add_constant!(KEY_ALTERASE)?;
    add_constant!(KEY_SYSREQ)?;
    add_constant!(KEY_CANCEL)?;
    add_constant!(KEY_CLEAR)?;
    add_constant!(KEY_PRIOR)?;
    add_constant!(KEY_RETURN2)?;
    add_constant!(KEY_SEPARATOR)?;
    add_constant!(KEY_OUT)?;
    add_constant!(KEY_OPER)?;
    add_constant!(KEY_CLEARAGAIN)?;
    add_constant!(KEY_CRSEL)?;
    add_constant!(KEY_EXSEL)?;
    add_constant!(KEY_KP_00)?;
    add_constant!(KEY_KP_000)?;
    add_constant!(KEY_THOUSANDSSEPARATOR)?;
    add_constant!(KEY_DECIMALSEPARATOR)?;
    add_constant!(KEY_CURRENCYUNIT)?;
    add_constant!(KEY_CURRENCYSUBUNIT)?;
    add_constant!(KEY_KP_LEFTPAREN)?;
    add_constant!(KEY_KP_RIGHTPAREN)?;
    add_constant!(KEY_KP_LEFTBRACE)?;
    add_constant!(KEY_KP_RIGHTBRACE)?;
    add_constant!(KEY_KP_TAB)?;
    add_constant!(KEY_KP_BACKSPACE)?;
    add_constant!(KEY_KP_A)?;
    add_constant!(KEY_KP_B)?;
    add_constant!(KEY_KP_C)?;
    add_constant!(KEY_KP_D)?;
    add_constant!(KEY_KP_E)?;
    add_constant!(KEY_KP_F)?;
    add_constant!(KEY_KP_XOR)?;
    add_constant!(KEY_KP_POWER)?;
    add_constant!(KEY_KP_PERCENT)?;
    add_constant!(KEY_KP_LESS)?;
    add_constant!(KEY_KP_GREATER)?;
    add_constant!(KEY_KP_AMPERSAND)?;
    add_constant!(KEY_KP_DBLAMPERSAND)?;
    add_constant!(KEY_KP_VERTICALBAR)?;
    add_constant!(KEY_KP_DBLVERTICALBAR)?;
    add_constant!(KEY_KP_COLON)?;
    add_constant!(KEY_KP_HASH)?;
    add_constant!(KEY_KP_SPACE)?;
    add_constant!(KEY_KP_AT)?;
    add_constant!(KEY_KP_EXCLAM)?;
    add_constant!(KEY_KP_MEMSTORE)?;
    add_constant!(KEY_KP_MEMRECALL)?;
    add_constant!(KEY_KP_MEMCLEAR)?;
    add_constant!(KEY_KP_MEMADD)?;
    add_constant!(KEY_KP_MEMSUBTRACT)?;
    add_constant!(KEY_KP_MEMMULTIPLY)?;
    add_constant!(KEY_KP_MEMDIVIDE)?;
    add_constant!(KEY_KP_PLUSMINUS)?;
    add_constant!(KEY_KP_CLEAR)?;
    add_constant!(KEY_KP_CLEARENTRY)?;
    add_constant!(KEY_KP_BINARY)?;
    add_constant!(KEY_KP_OCTAL)?;
    add_constant!(KEY_KP_DECIMAL)?;
    add_constant!(KEY_KP_HEXADECIMAL)?;
    add_constant!(KEY_LCTRL)?;
    add_constant!(KEY_LSHIFT)?;
    add_constant!(KEY_LALT)?;
    add_constant!(KEY_LGUI)?;
    add_constant!(KEY_RCTRL)?;
    add_constant!(KEY_RSHIFT)?;
    add_constant!(KEY_RALT)?;
    add_constant!(KEY_RGUI)?;
    add_constant!(KEY_MODE)?;
    add_constant!(KEY_AUDIONEXT)?;
    add_constant!(KEY_AUDIOPREV)?;
    add_constant!(KEY_AUDIOSTOP)?;
    add_constant!(KEY_AUDIOPLAY)?;
    add_constant!(KEY_AUDIOMUTE)?;
    add_constant!(KEY_MEDIASELECT)?;
    add_constant!(KEY_WWW)?;
    add_constant!(KEY_MAIL)?;
    add_constant!(KEY_CALCULATOR)?;
    add_constant!(KEY_COMPUTER)?;
    add_constant!(KEY_AC_SEARCH)?;
    add_constant!(KEY_AC_HOME)?;
    add_constant!(KEY_AC_BACK)?;
    add_constant!(KEY_AC_FORWARD)?;
    add_constant!(KEY_AC_STOP)?;
    add_constant!(KEY_AC_REFRESH)?;
    add_constant!(KEY_AC_BOOKMARKS)?;
    add_constant!(KEY_BRIGHTNESSDOWN)?;
    add_constant!(KEY_BRIGHTNESSUP)?;
    add_constant!(KEY_DISPLAYSWITCH)?;
    add_constant!(KEY_KBDILLUMTOGGLE)?;
    add_constant!(KEY_KBDILLUMDOWN)?;
    add_constant!(KEY_KBDILLUMUP)?;
    add_constant!(KEY_EJECT)?;
    add_constant!(KEY_SLEEP)?;
    add_constant!(KEY_APP1)?;
    add_constant!(KEY_APP2)?;
    add_constant!(KEY_AUDIOREWIND)?;
    add_constant!(KEY_AUDIOFASTFORWARD)?;
    add_constant!(KEY_SHIFT)?;
    add_constant!(KEY_CTRL)?;
    add_constant!(KEY_ALT)?;
    add_constant!(KEY_GUI)?;

    add_constant!(MOUSE_POS_X)?;
    add_constant!(MOUSE_POS_Y)?;
    add_constant!(MOUSE_WHEEL_X)?;
    add_constant!(MOUSE_WHEEL_Y)?;
    add_constant!(MOUSE_BUTTON_LEFT)?;
    add_constant!(MOUSE_BUTTON_MIDDLE)?;
    add_constant!(MOUSE_BUTTON_RIGHT)?;
    add_constant!(MOUSE_BUTTON_X1)?;
    add_constant!(MOUSE_BUTTON_X2)?;
    add_constant!(MOUSE_BUTTON_UNKOWN)?;

    add_constant!(GAMEPAD1_AXIS_LEFTX)?;
    add_constant!(GAMEPAD1_AXIS_LEFTY)?;
    add_constant!(GAMEPAD1_AXIS_RIGHTX)?;
    add_constant!(GAMEPAD1_AXIS_RIGHTY)?;
    add_constant!(GAMEPAD1_AXIS_TRIGGERLEFT)?;
    add_constant!(GAMEPAD1_AXIS_TRIGGERRIGHT)?;
    add_constant!(GAMEPAD1_BUTTON_A)?;
    add_constant!(GAMEPAD1_BUTTON_B)?;
    add_constant!(GAMEPAD1_BUTTON_X)?;
    add_constant!(GAMEPAD1_BUTTON_Y)?;
    add_constant!(GAMEPAD1_BUTTON_BACK)?;
    add_constant!(GAMEPAD1_BUTTON_GUIDE)?;
    add_constant!(GAMEPAD1_BUTTON_START)?;
    add_constant!(GAMEPAD1_BUTTON_LEFTSTICK)?;
    add_constant!(GAMEPAD1_BUTTON_RIGHTSTICK)?;
    add_constant!(GAMEPAD1_BUTTON_LEFTSHOULDER)?;
    add_constant!(GAMEPAD1_BUTTON_RIGHTSHOULDER)?;
    add_constant!(GAMEPAD1_BUTTON_DPAD_UP)?;
    add_constant!(GAMEPAD1_BUTTON_DPAD_DOWN)?;
    add_constant!(GAMEPAD1_BUTTON_DPAD_LEFT)?;
    add_constant!(GAMEPAD1_BUTTON_DPAD_RIGHT)?;

    add_constant!(GAMEPAD2_AXIS_LEFTX)?;
    add_constant!(GAMEPAD2_AXIS_LEFTY)?;
    add_constant!(GAMEPAD2_AXIS_RIGHTX)?;
    add_constant!(GAMEPAD2_AXIS_RIGHTY)?;
    add_constant!(GAMEPAD2_AXIS_TRIGGERLEFT)?;
    add_constant!(GAMEPAD2_AXIS_TRIGGERRIGHT)?;
    add_constant!(GAMEPAD2_BUTTON_A)?;
    add_constant!(GAMEPAD2_BUTTON_B)?;
    add_constant!(GAMEPAD2_BUTTON_X)?;
    add_constant!(GAMEPAD2_BUTTON_Y)?;
    add_constant!(GAMEPAD2_BUTTON_BACK)?;
    add_constant!(GAMEPAD2_BUTTON_GUIDE)?;
    add_constant!(GAMEPAD2_BUTTON_START)?;
    add_constant!(GAMEPAD2_BUTTON_LEFTSTICK)?;
    add_constant!(GAMEPAD2_BUTTON_RIGHTSTICK)?;
    add_constant!(GAMEPAD2_BUTTON_LEFTSHOULDER)?;
    add_constant!(GAMEPAD2_BUTTON_RIGHTSHOULDER)?;
    add_constant!(GAMEPAD2_BUTTON_DPAD_UP)?;
    add_constant!(GAMEPAD2_BUTTON_DPAD_DOWN)?;
    add_constant!(GAMEPAD2_BUTTON_DPAD_LEFT)?;
    add_constant!(GAMEPAD2_BUTTON_DPAD_RIGHT)?;

    Ok(())
}
