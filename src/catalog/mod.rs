
#[cfg(not(dosomething="local"))]
pub use self::core::dosomething;

#[cfg(dosomething="local")]
pub use self::local::dosomething;

//not overridden
pub use self::core::init;

pub mod core;
pub mod local;
