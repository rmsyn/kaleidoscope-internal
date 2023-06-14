#![allow(non_upper_case_globals)]
#![allow(unused)]

use usbd_hid::descriptor::{KeyboardUsage as KBU, MediaKey, SystemControlKey};

use super::*;

pub const fn is_printable(key: u8) -> bool {
    key <= KBU::KeypadHexadecimal as u8
}

pub const fn is_modifier(key: u8) -> bool {
    key >= KBU::KeyboardLeftControl as u8 && key <= KBU::KeyboardRightGUI as u8
}

pub fn is_media(key: u8) -> bool {
    MediaKey::from(key) != MediaKey::Reserved
}

pub fn is_system_control(key: u8) -> bool {
    SystemControlKey::from(key) != SystemControlKey::Reserved
}

pub(crate) const fn key_to_index(key: u8) -> usize {
    (key / 8) as usize
}

pub(crate) const fn key_to_printable_bitfield(key: u8) -> u8 {
    1 << (key % 8)
}

pub(crate) const fn key_to_modifier_bitfield(key: u8) -> u8 {
    1 << (key - KBU::KeyboardLeftControl as u8)
}

pub const Key_A: Key = Key::new(KBU::KeyboardAa as u8, KeyFlags::default());
pub const Key_B: Key = Key::new(KBU::KeyboardBb as u8, KeyFlags::default());
pub const Key_C: Key = Key::new(KBU::KeyboardCc as u8, KeyFlags::default());
pub const Key_D: Key = Key::new(KBU::KeyboardDd as u8, KeyFlags::default());
pub const Key_E: Key = Key::new(KBU::KeyboardEe as u8, KeyFlags::default());
pub const Key_F: Key = Key::new(KBU::KeyboardFf as u8, KeyFlags::default());
pub const Key_G: Key = Key::new(KBU::KeyboardGg as u8, KeyFlags::default());
pub const Key_H: Key = Key::new(KBU::KeyboardHh as u8, KeyFlags::default());
pub const Key_I: Key = Key::new(KBU::KeyboardIi as u8, KeyFlags::default());
pub const Key_J: Key = Key::new(KBU::KeyboardJj as u8, KeyFlags::default());
pub const Key_K: Key = Key::new(KBU::KeyboardKk as u8, KeyFlags::default());
pub const Key_L: Key = Key::new(KBU::KeyboardLl as u8, KeyFlags::default());
pub const Key_M: Key = Key::new(KBU::KeyboardMm as u8, KeyFlags::default());
pub const Key_N: Key = Key::new(KBU::KeyboardNn as u8, KeyFlags::default());
pub const Key_O: Key = Key::new(KBU::KeyboardOo as u8, KeyFlags::default());
pub const Key_P: Key = Key::new(KBU::KeyboardPp as u8, KeyFlags::default());
pub const Key_Q: Key = Key::new(KBU::KeyboardQq as u8, KeyFlags::default());
pub const Key_R: Key = Key::new(KBU::KeyboardRr as u8, KeyFlags::default());
pub const Key_S: Key = Key::new(KBU::KeyboardSs as u8, KeyFlags::default());
pub const Key_T: Key = Key::new(KBU::KeyboardTt as u8, KeyFlags::default());
pub const Key_U: Key = Key::new(KBU::KeyboardUu as u8, KeyFlags::default());
pub const Key_V: Key = Key::new(KBU::KeyboardVv as u8, KeyFlags::default());
pub const Key_W: Key = Key::new(KBU::KeyboardWw as u8, KeyFlags::default());
pub const Key_X: Key = Key::new(KBU::KeyboardXx as u8, KeyFlags::default());
pub const Key_Y: Key = Key::new(KBU::KeyboardYy as u8, KeyFlags::default());
pub const Key_Z: Key = Key::new(KBU::KeyboardZz as u8, KeyFlags::default());
pub const Key_1: Key = Key::new(KBU::Keyboard1Exclamation as u8, KeyFlags::default());
pub const Key_2: Key = Key::new(KBU::Keyboard2At as u8, KeyFlags::default());
pub const Key_3: Key = Key::new(KBU::Keyboard3Hash as u8, KeyFlags::default());
pub const Key_4: Key = Key::new(KBU::Keyboard4Dollar as u8, KeyFlags::default());
pub const Key_5: Key = Key::new(KBU::Keyboard5Percent as u8, KeyFlags::default());
pub const Key_6: Key = Key::new(KBU::Keyboard6Caret as u8, KeyFlags::default());
pub const Key_7: Key = Key::new(KBU::Keyboard7Ampersand as u8, KeyFlags::default());
pub const Key_8: Key = Key::new(KBU::Keyboard8Asterisk as u8, KeyFlags::default());
pub const Key_9: Key = Key::new(KBU::Keyboard9OpenParens as u8, KeyFlags::default());
pub const Key_0: Key = Key::new(KBU::Keyboard0CloseParens as u8, KeyFlags::default());
pub const Key_Enter: Key = Key::new(KBU::KeyboardEnter as u8, KeyFlags::default());
pub const Key_Escape: Key = Key::new(KBU::KeyboardEscape as u8, KeyFlags::default());
pub const Key_Backspace: Key = Key::new(KBU::KeyboardBackspace as u8, KeyFlags::default());
pub const Key_Tab: Key = Key::new(KBU::KeyboardTab as u8, KeyFlags::default());
pub const Key_Spacebar: Key = Key::new(KBU::KeyboardSpacebar as u8, KeyFlags::default());
pub const Key_Minus: Key = Key::new(KBU::KeyboardDashUnderscore as u8, KeyFlags::default());
pub const Key_Equals: Key = Key::new(KBU::KeyboardEqualPlus as u8, KeyFlags::default());
pub const Key_LeftBracket: Key = Key::new(KBU::KeyboardOpenBracketBrace as u8, KeyFlags::default());
pub const Key_RightBracket: Key =
    Key::new(KBU::KeyboardCloseBracketBrace as u8, KeyFlags::default());
pub const Key_Backslash: Key = Key::new(KBU::KeyboardBackslashBar as u8, KeyFlags::default());
pub const Key_NonUsPound: Key = Key::new(KBU::KeyboardNonUSHash as u8, KeyFlags::default());
pub const Key_Semicolon: Key = Key::new(KBU::KeyboardSemiColon as u8, KeyFlags::default());
pub const Key_Quote: Key = Key::new(KBU::KeyboardSingleDoubleQuote as u8, KeyFlags::default());
pub const Key_Backtick: Key = Key::new(KBU::KeyboardBacktickTilde as u8, KeyFlags::default());
pub const Key_Comma: Key = Key::new(KBU::KeyboardCommaLess as u8, KeyFlags::default());
pub const Key_Period: Key = Key::new(KBU::KeyboardPeriodGreater as u8, KeyFlags::default());
pub const Key_Slash: Key = Key::new(KBU::KeyboardSlashQuestion as u8, KeyFlags::default());
pub const Key_CapsLock: Key = Key::new(KBU::KeyboardCapsLock as u8, KeyFlags::default());
pub const Key_F1: Key = Key::new(KBU::KeyboardF1 as u8, KeyFlags::default());
pub const Key_F2: Key = Key::new(KBU::KeyboardF2 as u8, KeyFlags::default());
pub const Key_F3: Key = Key::new(KBU::KeyboardF3 as u8, KeyFlags::default());
pub const Key_F4: Key = Key::new(KBU::KeyboardF4 as u8, KeyFlags::default());
pub const Key_F5: Key = Key::new(KBU::KeyboardF5 as u8, KeyFlags::default());
pub const Key_F6: Key = Key::new(KBU::KeyboardF6 as u8, KeyFlags::default());
pub const Key_F7: Key = Key::new(KBU::KeyboardF7 as u8, KeyFlags::default());
pub const Key_F8: Key = Key::new(KBU::KeyboardF8 as u8, KeyFlags::default());
pub const Key_F9: Key = Key::new(KBU::KeyboardF9 as u8, KeyFlags::default());
pub const Key_F10: Key = Key::new(KBU::KeyboardF10 as u8, KeyFlags::default());
pub const Key_F11: Key = Key::new(KBU::KeyboardF11 as u8, KeyFlags::default());
pub const Key_F12: Key = Key::new(KBU::KeyboardF12 as u8, KeyFlags::default());
pub const Key_PrintScreen: Key = Key::new(KBU::KeyboardPrintScreen as u8, KeyFlags::default());
pub const Key_ScrollLock: Key = Key::new(KBU::KeyboardScrollLock as u8, KeyFlags::default());
pub const Key_Pause: Key = Key::new(KBU::KeyboardPause as u8, KeyFlags::default());
pub const Key_Insert: Key = Key::new(KBU::KeyboardInsert as u8, KeyFlags::default());
pub const Key_Home: Key = Key::new(KBU::KeyboardHome as u8, KeyFlags::default());
pub const Key_PageUp: Key = Key::new(KBU::KeyboardPageUp as u8, KeyFlags::default());
pub const Key_Delete: Key = Key::new(KBU::KeyboardDelete as u8, KeyFlags::default());
pub const Key_End: Key = Key::new(KBU::KeyboardEnd as u8, KeyFlags::default());
pub const Key_PageDown: Key = Key::new(KBU::KeyboardPageDown as u8, KeyFlags::default());
pub const Key_RightArrow: Key = Key::new(KBU::KeyboardRightArrow as u8, KeyFlags::default());
pub const Key_LeftArrow: Key = Key::new(KBU::KeyboardLeftArrow as u8, KeyFlags::default());
pub const Key_DownArrow: Key = Key::new(KBU::KeyboardDownArrow as u8, KeyFlags::default());
pub const Key_UpArrow: Key = Key::new(KBU::KeyboardUpArrow as u8, KeyFlags::default());
pub const Key_KeypadNumLock: Key = Key::new(KBU::KeypadNumLock as u8, KeyFlags::default());
pub const Key_KeypadDivide: Key = Key::new(KBU::KeypadDivide as u8, KeyFlags::default());
pub const Key_KeypadMultiply: Key = Key::new(KBU::KeypadMultiply as u8, KeyFlags::default());
pub const Key_KeypadSubtract: Key = Key::new(KBU::KeypadMinus as u8, KeyFlags::default());
pub const Key_KeypadAdd: Key = Key::new(KBU::KeypadPlus as u8, KeyFlags::default());
pub const Key_KeypadEnter: Key = Key::new(KBU::KeypadEnter as u8, KeyFlags::default());
pub const Key_Keypad1: Key = Key::new(KBU::Keypad1End as u8, KeyFlags::default());
pub const Key_Keypad2: Key = Key::new(KBU::Keypad2DownArrow as u8, KeyFlags::default());
pub const Key_Keypad3: Key = Key::new(KBU::Keypad3PageDown as u8, KeyFlags::default());
pub const Key_Keypad4: Key = Key::new(KBU::Keypad4LeftArrow as u8, KeyFlags::default());
pub const Key_Keypad5: Key = Key::new(KBU::Keypad5 as u8, KeyFlags::default());
pub const Key_Keypad6: Key = Key::new(KBU::Keypad6RightArrow as u8, KeyFlags::default());
pub const Key_Keypad7: Key = Key::new(KBU::Keypad7Home as u8, KeyFlags::default());
pub const Key_Keypad8: Key = Key::new(KBU::Keypad8UpArrow as u8, KeyFlags::default());
pub const Key_Keypad9: Key = Key::new(KBU::Keypad9PageUp as u8, KeyFlags::default());
pub const Key_Keypad0: Key = Key::new(KBU::Keypad0Insert as u8, KeyFlags::default());
pub const Key_KeypadDot: Key = Key::new(KBU::KeypadPeriodDelete as u8, KeyFlags::default());
pub const Key_NonUsBackslashAndPipe: Key =
    Key::new(KBU::KeyboardNonUSSlash as u8, KeyFlags::default());
pub const Key_PcApplication: Key = Key::new(KBU::KeyboardApplication as u8, KeyFlags::default());
pub const Key_Power: Key = Key::new(KBU::KeyboardPower as u8, KeyFlags::default());
pub const Key_KeypadEquals: Key = Key::new(KBU::KeypadEqual as u8, KeyFlags::default());
pub const Key_F13: Key = Key::new(KBU::KeyboardF13 as u8, KeyFlags::default());
pub const Key_F14: Key = Key::new(KBU::KeyboardF14 as u8, KeyFlags::default());
pub const Key_F15: Key = Key::new(KBU::KeyboardF15 as u8, KeyFlags::default());
pub const Key_F16: Key = Key::new(KBU::KeyboardF16 as u8, KeyFlags::default());
pub const Key_F17: Key = Key::new(KBU::KeyboardF17 as u8, KeyFlags::default());
pub const Key_F18: Key = Key::new(KBU::KeyboardF18 as u8, KeyFlags::default());
pub const Key_F19: Key = Key::new(KBU::KeyboardF19 as u8, KeyFlags::default());
pub const Key_F20: Key = Key::new(KBU::KeyboardF20 as u8, KeyFlags::default());
pub const Key_F21: Key = Key::new(KBU::KeyboardF21 as u8, KeyFlags::default());
pub const Key_F22: Key = Key::new(KBU::KeyboardF22 as u8, KeyFlags::default());
pub const Key_F23: Key = Key::new(KBU::KeyboardF23 as u8, KeyFlags::default());
pub const Key_F24: Key = Key::new(KBU::KeyboardF24 as u8, KeyFlags::default());
pub const Key_Execute: Key = Key::new(KBU::KeyboardExecute as u8, KeyFlags::default());
pub const Key_Help: Key = Key::new(KBU::KeyboardHelp as u8, KeyFlags::default());
pub const Key_Menu: Key = Key::new(KBU::KeyboardMenu as u8, KeyFlags::default());
pub const Key_Select: Key = Key::new(KBU::KeyboardSelect as u8, KeyFlags::default());
pub const Key_Stop: Key = Key::new(KBU::KeyboardStop as u8, KeyFlags::default());
pub const Key_Again: Key = Key::new(KBU::KeyboardAgain as u8, KeyFlags::default());
pub const Key_Undo: Key = Key::new(KBU::KeyboardUndo as u8, KeyFlags::default());
pub const Key_Cut: Key = Key::new(KBU::KeyboardCut as u8, KeyFlags::default());
pub const Key_Copy: Key = Key::new(KBU::KeyboardCopy as u8, KeyFlags::default());
pub const Key_Paste: Key = Key::new(KBU::KeyboardPaste as u8, KeyFlags::default());
pub const Key_Find: Key = Key::new(KBU::KeyboardFind as u8, KeyFlags::default());
pub const Key_Mute: Key = Key::new(KBU::KeyboardMute as u8, KeyFlags::default());
pub const Key_VolumeUp: Key = Key::new(KBU::KeyboardVolumeUp as u8, KeyFlags::default());
pub const Key_VolumeDown: Key = Key::new(KBU::KeyboardVolumeDown as u8, KeyFlags::default());
pub const Key_LockingCapsLock: Key =
    Key::new(KBU::KeyboardLockingCapsLock as u8, KeyFlags::default());
pub const Key_LockingNumLock: Key =
    Key::new(KBU::KeyboardLockingNumLock as u8, KeyFlags::default());
pub const Key_LockingScrollLock: Key =
    Key::new(KBU::KeyboardLockingScrollLock as u8, KeyFlags::default());
pub const Key_KeypadComma: Key = Key::new(KBU::KeypadComma as u8, KeyFlags::default());
pub const Key_KeypadEqualSign: Key = Key::new(KBU::KeypadEqualSign as u8, KeyFlags::default());
pub const Key_International1: Key =
    Key::new(KBU::KeyboardInternational1 as u8, KeyFlags::default());
pub const Key_International2: Key =
    Key::new(KBU::KeyboardInternational2 as u8, KeyFlags::default());
pub const Key_International3: Key =
    Key::new(KBU::KeyboardInternational3 as u8, KeyFlags::default());
pub const Key_International4: Key =
    Key::new(KBU::KeyboardInternational4 as u8, KeyFlags::default());
pub const Key_International5: Key =
    Key::new(KBU::KeyboardInternational5 as u8, KeyFlags::default());
pub const Key_International6: Key =
    Key::new(KBU::KeyboardInternational6 as u8, KeyFlags::default());
pub const Key_International7: Key =
    Key::new(KBU::KeyboardInternational7 as u8, KeyFlags::default());
pub const Key_International8: Key =
    Key::new(KBU::KeyboardInternational8 as u8, KeyFlags::default());
pub const Key_International9: Key =
    Key::new(KBU::KeyboardInternational9 as u8, KeyFlags::default());
pub const Key_Lang1: Key = Key::new(KBU::KeyboardLANG1 as u8, KeyFlags::default());
pub const Key_Lang2: Key = Key::new(KBU::KeyboardLANG2 as u8, KeyFlags::default());
pub const Key_Lang3: Key = Key::new(KBU::KeyboardLANG3 as u8, KeyFlags::default());
pub const Key_Lang4: Key = Key::new(KBU::KeyboardLANG4 as u8, KeyFlags::default());
pub const Key_Lang5: Key = Key::new(KBU::KeyboardLANG5 as u8, KeyFlags::default());
pub const Key_Lang6: Key = Key::new(KBU::KeyboardLANG6 as u8, KeyFlags::default());
pub const Key_Lang7: Key = Key::new(KBU::KeyboardLANG7 as u8, KeyFlags::default());
pub const Key_Lang8: Key = Key::new(KBU::KeyboardLANG8 as u8, KeyFlags::default());
pub const Key_Lang9: Key = Key::new(KBU::KeyboardLANG9 as u8, KeyFlags::default());
pub const Key_AlternateErase: Key =
    Key::new(KBU::KeyboardAlternateErase as u8, KeyFlags::default());
pub const Key_SysReq: Key = Key::new(KBU::KeyboardSysReqAttention as u8, KeyFlags::default());
pub const Key_Cancel: Key = Key::new(KBU::KeyboardCancel as u8, KeyFlags::default());
pub const Key_Clear: Key = Key::new(KBU::KeyboardClear as u8, KeyFlags::default());
pub const Key_Prior: Key = Key::new(KBU::KeyboardPrior as u8, KeyFlags::default());
pub const Key_Return: Key = Key::new(KBU::KeyboardReturn as u8, KeyFlags::default());
pub const Key_Separator: Key = Key::new(KBU::KeyboardSeparator as u8, KeyFlags::default());
pub const Key_Out: Key = Key::new(KBU::KeyboardOut as u8, KeyFlags::default());
pub const Key_Oper: Key = Key::new(KBU::KeyboardOper as u8, KeyFlags::default());
pub const Key_ClearSlashAgain: Key = Key::new(KBU::KeyboardClearAgain as u8, KeyFlags::default());
pub const Key_CrSelSlashProps: Key = Key::new(KBU::KeyboardCrSelProps as u8, KeyFlags::default());
pub const Key_Exsel: Key = Key::new(KBU::KeyboardExSel as u8, KeyFlags::default());
pub const Key_Keypad00: Key = Key::new(KBU::Keypad00 as u8, KeyFlags::default());
pub const Key_Keypad000: Key = Key::new(KBU::Keypad000 as u8, KeyFlags::default());
pub const Key_ThousandsSeparator: Key =
    Key::new(KBU::ThousandsSeparator as u8, KeyFlags::default());
pub const Key_DecimalSeparator: Key = Key::new(KBU::DecimalSeparator as u8, KeyFlags::default());
pub const Key_CurrencyUnit: Key = Key::new(KBU::CurrencyUnit as u8, KeyFlags::default());
pub const Key_CurrencySubunit: Key = Key::new(KBU::CurrencyUnit as u8, KeyFlags::default());
pub const Key_KeypadLeftParen: Key = Key::new(KBU::KeypadOpenParens as u8, KeyFlags::default());
pub const Key_KeypadRightParen: Key = Key::new(KBU::KeypadCloseParens as u8, KeyFlags::default());
pub const Key_LeftCurlyBracket: Key = Key::new(KBU::KeypadOpenBrace as u8, KeyFlags::default());
pub const Key_RightCurlyBracket: Key = Key::new(KBU::KeypadCloseBrace as u8, KeyFlags::default());
pub const Key_KeypadTab: Key = Key::new(KBU::KeypadTab as u8, KeyFlags::default());
pub const Key_KeypadBackspace: Key = Key::new(KBU::KeypadBackspace as u8, KeyFlags::default());
pub const Key_KeypadA: Key = Key::new(KBU::KeypadA as u8, KeyFlags::default());
pub const Key_KeypadB: Key = Key::new(KBU::KeypadB as u8, KeyFlags::default());
pub const Key_KeypadC: Key = Key::new(KBU::KeypadC as u8, KeyFlags::default());
pub const Key_KeypadD: Key = Key::new(KBU::KeypadD as u8, KeyFlags::default());
pub const Key_KeypadE: Key = Key::new(KBU::KeypadE as u8, KeyFlags::default());
pub const Key_KeypadF: Key = Key::new(KBU::KeypadF as u8, KeyFlags::default());
pub const Key_KeypadXor: Key = Key::new(KBU::KeypadBitwiseXor as u8, KeyFlags::default());
pub const Key_KeypadCarat: Key = Key::new(KBU::KeypadLogicalXor as u8, KeyFlags::default());
pub const Key_KeypadPercent: Key = Key::new(KBU::KeypadModulo as u8, KeyFlags::default());
pub const Key_KeypadLessThan: Key = Key::new(KBU::KeypadLeftShift as u8, KeyFlags::default());
pub const Key_KeypadGreaterThan: Key = Key::new(KBU::KeypadRightShift as u8, KeyFlags::default());
pub const Key_KeypadAmpersand: Key = Key::new(KBU::KeypadBitwiseAnd as u8, KeyFlags::default());
pub const Key_KeypadDoubleAmpersand: Key =
    Key::new(KBU::KeypadLogicalAnd as u8, KeyFlags::default());
pub const Key_KeypadPipe: Key = Key::new(KBU::KeypadBitwiseOr as u8, KeyFlags::default());
pub const Key_KeypadDoublePipe: Key = Key::new(KBU::KeypadLogicalOr as u8, KeyFlags::default());
pub const Key_KeypadColon: Key = Key::new(KBU::KeypadColon as u8, KeyFlags::default());
pub const Key_KeypadPoundSign: Key = Key::new(KBU::KeypadHash as u8, KeyFlags::default());
pub const Key_KeypadSpace: Key = Key::new(KBU::KeypadSpace as u8, KeyFlags::default());
pub const Key_KeypadAtSign: Key = Key::new(KBU::KeypadAt as u8, KeyFlags::default());
pub const Key_KeypadExclamationPoint: Key =
    Key::new(KBU::KeypadExclamation as u8, KeyFlags::default());
pub const Key_KeypadMemoryStore: Key = Key::new(KBU::KeypadMemoryStore as u8, KeyFlags::default());
pub const Key_KeypadMemoryRecall: Key =
    Key::new(KBU::KeypadMemoryRecall as u8, KeyFlags::default());
pub const Key_KeypadMemoryClear: Key = Key::new(KBU::KeypadMemoryClear as u8, KeyFlags::default());
pub const Key_KeypadMemoryAdd: Key = Key::new(KBU::KeypadMemoryAdd as u8, KeyFlags::default());
pub const Key_KeypadMemorySubtract: Key =
    Key::new(KBU::KeypadMemorySubtract as u8, KeyFlags::default());
pub const Key_KeypadMemoryMultiply: Key =
    Key::new(KBU::KeypadMemoryMultiply as u8, KeyFlags::default());
pub const Key_KeypadMemoryDivide: Key =
    Key::new(KBU::KeypadMemoryDivide as u8, KeyFlags::default());
pub const Key_KeypadPlusSlashMinus: Key =
    Key::new(KBU::KeypadPositiveNegative as u8, KeyFlags::default());
pub const Key_KeypadClear: Key = Key::new(KBU::KeypadClear as u8, KeyFlags::default());
pub const Key_KeypadClearEntry: Key = Key::new(KBU::KeypadClearEntry as u8, KeyFlags::default());
pub const Key_KeypadBinary: Key = Key::new(KBU::KeypadBinary as u8, KeyFlags::default());
pub const Key_KeypadOctal: Key = Key::new(KBU::KeypadOctal as u8, KeyFlags::default());
pub const Key_KeypadDecimal: Key = Key::new(KBU::KeypadDecimal as u8, KeyFlags::default());
pub const Key_KeypadHexadecimal: Key = Key::new(KBU::KeypadHexadecimal as u8, KeyFlags::default());
pub const Key_LeftControl: Key = Key::new(KBU::KeyboardLeftControl as u8, KeyFlags::default());
pub const Key_LeftShift: Key = Key::new(KBU::KeyboardLeftShift as u8, KeyFlags::default());
pub const Key_LeftAlt: Key = Key::new(KBU::KeyboardLeftAlt as u8, KeyFlags::default());
pub const Key_LeftGui: Key = Key::new(KBU::KeyboardLeftGUI as u8, KeyFlags::default());
pub const Key_RightControl: Key = Key::new(KBU::KeyboardRightControl as u8, KeyFlags::default());
pub const Key_RightShift: Key = Key::new(KBU::KeyboardRightShift as u8, KeyFlags::default());
pub const Key_RightAlt: Key = Key::new(KBU::KeyboardRightAlt as u8, KeyFlags::default());
pub const Key_RightGui: Key = Key::new(KBU::KeyboardRightGUI as u8, KeyFlags::default());
