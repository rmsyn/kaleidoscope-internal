/// Creates a [Key](crate::key_defs::Key) based on the [MACRO_FIRST](crate::plugins::ranges::MACRO_FIRST) constant definition.
#[macro_export]
macro_rules! M {
    ($k:tt) => {
        $crate::key_defs::Key::from_raw($crate::plugins::ranges::MACRO_FIRST + $k as u16)
    };
}
