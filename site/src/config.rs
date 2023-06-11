#[cfg(debug_assertions)]
pub const ADDRESS: &str = "127.0.0.1";
#[cfg(debug_assertions)]
pub const PORT: u16 = 8000;

#[cfg(not(debug_assertions))]
pub const ADDRESS: &str = "127.0.0.1";
#[cfg(not(debug_assertions))]
pub const PORT: u16 = 8000;
