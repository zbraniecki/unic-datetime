pub mod layout;
pub mod layout2;
pub mod patterns;
pub mod patterns2;

#[cfg(feature = "serde")]
pub mod load;
#[cfg(feature = "serde")]
pub mod load2;
#[cfg(feature = "serde")]
pub mod load3;
#[cfg(feature = "bincode")]
pub mod load_bin;
#[cfg(not(feature = "no-static"))]
pub mod pl;
