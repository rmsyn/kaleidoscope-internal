use usbd_hid::descriptor::MediaKey;

use super::*;

pub const Consumer_VolumeIncrement: Key =
    Key::consumer_key(MediaKey::VolumeIncrement as u16, HID_TYPE_RTC as u16);
pub const Consumer_VolumeDecrement: Key =
    Key::consumer_key(MediaKey::VolumeDecrement as u16, HID_TYPE_RTC as u16);
pub const Consumer_PlaySlashPause: Key =
    Key::consumer_key(MediaKey::PlayPause as u16, HID_TYPE_OSC as u16);
