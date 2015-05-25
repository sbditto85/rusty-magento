
#[cfg(not(dosomething="local"))]
pub use self::core::dosomething;

#[cfg(dosomething="local")]
pub use self::local::dosomething;

pub mod core;
pub mod local;
