#![warn(missing_docs)]

//! Rust bindings for the [Philips Hue API].
//!
//! ## About
//!
//! This library sends HTTP requests to the bridge using the [ureq] crate. The responses/requests
//! are deserialized/serialized using the [serde], [serde_json] and [serde_repr] crates.
//!
//! [Philips Hue API]: https://developers.meethue.com/develop/hue-api
//! [ureq]: https://github.com/algesten/ureq
//! [serde]: https://github.com/serde-rs/serde
//! [serde_json]: https://github.com/serde-rs/json
//! [serde_repr]: https://github.com/dtolnay/serde-repr
//!
//! ## Example
//!
//! Register a user and set the brightness and saturation of a light.
//! ```rust
//! use huelib::{bridge, light, Modifier};
//! use std::net::{IpAddr, Ipv4Addr};
//!
//! let bridge_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2));
//! let username = match bridge::register_user(bridge_ip, "huelib-rs example", false) {
//!     Ok(v) => v.name,
//!     Err(e) => {
//!         println!("Failed to register user: {}", e);
//!         return;
//!     }
//! };
//! let bridge = huelib::Bridge::new(bridge_ip, &username);
//! let state_modifier = light::StateModifier::new()
//!     .brightness(huelib::ModifierType::Increment, 40)
//!     .saturation(huelib::ModifierType::Override, 200);
//! match bridge.set_light_state("1", &state_modifier) {
//!     Ok(v) => {
//!         for response in v {
//!             println!("{}", response);
//!         }
//!     },
//!     Err(e) => {
//!         println!("Failed to set the state of the light: {}", e);
//!         return;
//!     }
//! };
//! ```

/// Module for managing bridges.
pub mod bridge;

/// Bindings to the [Capabilities API].
///
/// [Capabilities API]: https://developers.meethue.com/develop/hue-api/10-capabilities-api/
pub mod capabilities;

/// Bindings to the [Configuration API].
///
/// [Configuration API]: https://developers.meethue.com/develop/hue-api/7-configuration-api
pub mod config;

/// Errors that can occur while interacting with the Philips Hue API.
pub mod error;

/// Bindings to the [Groups API].
///
/// [Groups API]: https://developers.meethue.com/develop/hue-api/groupds-api
pub mod group;

/// Bindings to the [Lights API].
///
/// [Lights API]: https://developers.meethue.com/develop/hue-api/lights-api
pub mod light;

/// Bindings to the [Resourcelinks API].
///
/// [Resourcelinks API]: https://developers.meethue.com/develop/hue-api/9-resourcelinks-api
pub mod resourcelink;

/// Responses returned from the Philips Hue API.
pub mod response;

/// Bindings to the [Rules API].
///
/// [Rules API]: https://developers.meethue.com/develop/hue-api/6-rules-api
pub mod rule;

/// Bindings to the [Scenes API].
///
/// [Scenes API]: https://developers.meethue.com/develop/hue-api/4-scenes
pub mod scene;

/// Bindings to the [Schedules API].
///
/// [Schedules API]: https://developers.meethue.com/develop/hue-api/3-schedules-api
pub mod schedule;

/// Bindings to the [Sensors API].
///
/// [Sensors API]: https://developers.meethue.com/develop/hue-api/5-sensors-api
pub mod sensor;

pub use bridge::Bridge;
pub use capabilities::Capabilities;
pub use config::Config;
pub use error::Error;
pub use group::Group;
pub use light::Light;
pub use resourcelink::Resourcelink;
pub use response::Response;
pub use rule::Rule;
pub use scene::Scene;
pub use schedule::Schedule;
pub use sensor::Sensor;

use serde::{Deserialize, Serialize};

/// Alert effect of a light.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Alert {
    /// Performs one breathe cycle.
    Select,
    /// Performs breathe cycles for 15 seconds or until the alert attribute is set to `None`.
    LSelect,
    /// Disables any alert.
    None,
}

/// Dynamic effect of a light.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Effect {
    /// Cycles through all hues with the current brightness and saturation.
    Colorloop,
    /// Disables any effect.
    None,
}

/// Color mode of a light.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub enum ColorMode {
    /// Uses a color temperatue to set the color of a light.
    #[serde(rename = "ct")]
    ColorTemperature,
    /// Uses hue and saturation to set the color of a light.
    #[serde(rename = "hs")]
    HueAndSaturation,
    /// Uses x and y coordinates in the color space to set the color of a light.
    #[serde(rename = "xy")]
    ColorSpaceCoordinates,
}

/// Action of a schedule or rule.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Action {
    /// Address where the action will be executed.
    pub address: String,
    /// The HTTP method used to send the body to the given address.
    #[serde(rename = "method")]
    pub request_type: ActionRequestType,
    /// Body of the request that the action sends.
    pub body: std::collections::HashMap<String, serde_json::Value>,
}

/// Request type of an action.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ActionRequestType {
    Put,
    Post,
    Delete,
}

/// Type of a modifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModifierType {
    /// Override the current value with the given value.
    Override,
    /// Add the given value to the current value.
    Increment,
    /// Subtract the given value to the current value.
    Decrement,
}

/// Type of a modifier for coordinates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoordinateModifierType {
    /// Override both current values with the given values.
    Override,
    /// Add the given values to the current values.
    Increment,
    /// Subtract the given values to the current values.
    Decrement,
    /// Add the given value for the first coordinate to the current value of the first coordinate
    /// and subtract the given value for the second coordinate to the current value of the second
    /// coordinate.
    IncrementDecrement,
    /// Subtract the given value for the first coordinate to the current value of the first
    /// coordinate and add the given value for the second coordinate to the current value of the
    /// second coordinate.
    DecrementIncrement,
}

/// Trait for modifiers.
pub trait Modifier: Default + PartialEq {
    /// Creates a new modifier.
    fn new() -> Self {
        Default::default()
    }

    /// Whether the modifier will not modify anything.
    fn is_empty(&self) -> bool {
        self == &Default::default()
    }
}

/// Trait for creators.
pub trait Creator {}
