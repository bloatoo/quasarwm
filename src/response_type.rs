#[repr(u8)]
pub enum ResponseType {
    DestroyNotify,
    KeyPress,
    MapNotify,
    Unknown,
}

impl<T: Into<u8>> From<T> for ResponseType {
    fn from(data: T) -> ResponseType {
        match data.into() {
            xcb::DESTROY_NOTIFY => ResponseType::DestroyNotify,
            xcb::KEY_PRESS => ResponseType::KeyPress,
            xcb::MAP_NOTIFY => ResponseType::MapNotify,
            _ => ResponseType::Unknown,
        }
    }
}
