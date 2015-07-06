
#[cfg(not(checkout_dosomething="local"))]
pub use self::core::dosomething;

#[cfg(checkout_dosomething="local")]
pub use self::local::dosomething;

//not overridden
pub use self::core::init;

pub mod core;
pub mod local;
