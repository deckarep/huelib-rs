use crate::{Alert, ColorMode, CoordinateModifierType, Effect, ModifierType};
use serde::{de, de::Error, Deserialize, Serialize};
use std::fmt;

/// A light.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Light {
    /// Identifier of the light.
    #[serde(skip)]
    pub id: String,
    /// Name of the light.
    pub name: String,
    /// Type of the light.
    #[serde(rename = "type")]
    pub kind: String,
    /// Current state of the light.
    pub state: State,
    /// The hardware model of the light.
    #[serde(rename = "modelid")]
    pub model_id: String,
    /// Unique ID of the light.
    #[serde(rename = "uniqueid")]
    pub unique_id: String,
    /// Product ID of the light.
    #[serde(rename = "productid")]
    pub product_id: Option<String>,
    /// Product name of the light.
    #[serde(rename = "productname")]
    pub product_name: Option<String>,
    /// Manufacturer name of the light.
    #[serde(rename = "manufacturername")]
    pub manufacturer_name: Option<String>,
    /// The software version running on the light.
    #[serde(rename = "swversion")]
    pub software_version: String,
    /// Information about software updates of the light.
    #[serde(rename = "swupdate")]
    pub software_update: SoftwareUpdate,
    /// Configuration of the light.
    pub config: Config,
    /// Capabilities of the light.
    pub capabilities: Capabilities,
}

impl Light {
    pub(crate) fn with_id<S: Into<String>>(self, id: S) -> Self {
        Self {
            id: id.into(),
            ..self
        }
    }
}

/// State of a light.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct State {
    /// Whether the light is on.
    pub on: Option<bool>,
    /// Brightness of the light.
    ///
    /// The maximum brightness is 254 and 1 is the minimum brightness.
    #[serde(rename = "bri")]
    pub brightness: Option<u8>,
    /// Hue of the light.
    ///
    /// Both 0 and 65535 are red, 25500 is green and 46920 is blue.
    pub hue: Option<u16>,
    /// Saturation of the light.
    ///
    /// The most saturated (colored) is 254 and 0 is the least saturated (white).
    #[serde(rename = "sat")]
    pub saturation: Option<u8>,
    /// X and y coordinates of a color in CIE color space. Both values must be between 0 and 1.
    #[serde(rename = "xy")]
    pub color_space_coordinates: Option<(f32, f32)>,
    /// Mired color temperature of the light.
    #[serde(rename = "ct")]
    pub color_temperature: Option<u16>,
    /// Alert effect of the light.
    pub alert: Option<Alert>,
    /// Dynamic effect of the light.
    pub effect: Option<Effect>,
    /// Color mode of the light.
    #[serde(rename = "colormode")]
    pub color_mode: Option<ColorMode>,
    /// Whether the light can be reached by the bridge.
    pub reachable: bool,
}

/// Information about software updates of a light.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct SoftwareUpdate {
    /// State of software updates.
    pub state: SoftwareUpdateState,
    /// When the last update was installed.
    #[serde(rename = "lastinstall")]
    pub last_install: Option<chrono::NaiveDateTime>,
}

/// State of a software update.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SoftwareUpdateState {
    /// No updates are available.
    NoUpdates,
    /// Device cannot be updated.
    NotUpdatable,
    // TODO: Add missing variants for states (missing due to incomplete documentation)
}

/// Configuration of a light.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Config {
    /// Arche type of the light.
    #[serde(rename = "archetype")]
    pub arche_type: String,
    /// Function of the light.
    pub function: String,
    /// Direction of the light.
    pub direction: String,
    /// Startup configuration of the light.
    pub startup: Option<StartupConfig>,
}

/// Startup configuration of a light.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct StartupConfig {
    /// Mode of the startup.
    pub mode: String,
    /// Whether startup is configured for the light.
    pub configured: bool,
}

/// Capabilities of a light.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Capabilities {
    /// Whether the light is certified.
    pub certified: bool,
    /// Control capabilities of the light.
    pub control: ControlCapabilities,
    /// Streaming capabilities of the light.
    pub streaming: StreamingCapabilities,
}

/// Control capabilities of a light.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ControlCapabilities {
    /// Minimal dimlevel of the light.
    #[serde(rename = "mindimlevel")]
    pub min_dimlevel: Option<usize>,
    /// Maximal lumen of the light.
    #[serde(rename = "maxlumen")]
    pub max_lumen: Option<usize>,
    /// Color gamut of the light.
    #[serde(rename = "colorgamut")]
    pub color_gamut: Option<Vec<(f32, f32)>>,
    /// Type of the color gamut of the light.
    #[serde(rename = "colorgamuttype")]
    pub color_gamut_type: Option<String>,
    /// Maximal/minimal color temperature of the light.
    #[serde(rename = "ct")]
    pub color_temperature: Option<ColorTemperatureCapabilities>,
}

/// Color temperature capabilities of a light.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub struct ColorTemperatureCapabilities {
    /// Minimal color temperature.
    pub min: usize,
    /// Maximal color temperature.
    pub max: usize,
}

/// Streaming capabilities of a light.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub struct StreamingCapabilities {
    /// Whether a renderer is enabled.
    pub renderer: bool,
    /// Whether a proxy is enabled.
    pub proxy: bool,
}

/// Struct for new lights that were scanned by the bridge.
#[derive(Clone, Debug, PartialEq)]
pub struct Scan {
    /// When the bridge last scanned for new lights.
    pub last_scan: LastScan,
    /// New lights that were discovered.
    pub lights: Vec<ScanLight>,
}

impl<'de> Deserialize<'de> for Scan {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        enum Field {
            LastScan,
            LightId(String),
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value: String = Deserialize::deserialize(deserializer)?;
                Ok(match value.as_ref() {
                    "lastscan" => Field::LastScan,
                    v => Field::LightId(v.to_owned()),
                })
            }
        }

        struct ScanVisitor;

        impl<'de> de::Visitor<'de> for ScanVisitor {
            type Value = Scan;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("struct Scan")
            }

            fn visit_map<V: de::MapAccess<'de>>(self, mut map: V) -> Result<Scan, V::Error> {
                let mut lights = Vec::new();
                let mut last_scan = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::LastScan => {
                            last_scan = serde_json::from_value(map.next_value()?)
                                .map_err(V::Error::custom)?
                        }
                        Field::LightId(v) => {
                            let light = ScanLight {
                                id: v,
                                name: map.next_value()?,
                            };
                            lights.push(light);
                        }
                    }
                }
                let last_scan = last_scan.ok_or_else(|| de::Error::missing_field("lastscan"))?;
                Ok(Scan { lights, last_scan })
            }
        }

        const FIELDS: &[&str] = &["lastscan", "lights"];
        deserializer.deserialize_struct("Scan", FIELDS, ScanVisitor)
    }
}

/// Status of the last scan for new lights.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LastScan {
    /// Date and time of the last scan.
    DateTime(chrono::NaiveDateTime),
    /// The bridge is currently scanning.
    Active,
    /// The bridge did not scan since it was powered on.
    None,
}

impl<'de> Deserialize<'de> for LastScan {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value: String = Deserialize::deserialize(deserializer)?;
        Ok(match value.as_ref() {
            "active" => LastScan::Active,
            "none" => LastScan::None,
            v => LastScan::DateTime(
                chrono::NaiveDateTime::parse_from_str(v, "%Y-%m-%dT%H:%M:%S")
                    .map_err(D::Error::custom)?,
            ),
        })
    }
}

/// Informatino about a light that is returned from a scan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScanLight {
    /// Identifier of the light.
    pub id: String,
    /// Name of the light.
    pub name: String,
}

/// Modifier for light attributes.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct AttributeModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl crate::Modifier for AttributeModifier {}

impl AttributeModifier {
    /// Changes the name of the light.
    pub fn name<S: Into<String>>(self, value: S) -> Self {
        Self {
            name: Some(value.into()),
        }
    }
}

/// Modifier for the light state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize)]
pub struct StateModifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bri")]
    brightness: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hue: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sat")]
    saturation: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xy")]
    color_space_coordinates: Option<(f32, f32)>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ct")]
    color_temperature: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alert: Option<Alert>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<Effect>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "transitiontime")]
    transition_time: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bri_inc")]
    brightness_increment: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hue_inc")]
    hue_increment: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "sat_inc")]
    saturation_increment: Option<i16>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xy_inc")]
    color_space_coordinates_increment: Option<(f32, f32)>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ct_inc")]
    color_temperature_increment: Option<i32>,
}

impl crate::Modifier for StateModifier {}

impl StateModifier {
    /// Turns the light on or off.
    pub fn on(self, value: bool) -> Self {
        Self {
            on: Some(value),
            ..self
        }
    }

    /// Sets the brightness of the light.
    pub fn brightness(self, modifier_type: ModifierType, value: u8) -> Self {
        match modifier_type {
            ModifierType::Override => Self {
                brightness: Some(value),
                ..self
            },
            ModifierType::Increment => Self {
                brightness_increment: Some(value as i16),
                ..self
            },
            ModifierType::Decrement => Self {
                brightness_increment: Some(-(value as i16)),
                ..self
            },
        }
    }

    /// Sets the hue of a light.
    pub fn hue(self, modifier_type: ModifierType, value: u16) -> Self {
        match modifier_type {
            ModifierType::Override => Self {
                hue: Some(value),
                ..self
            },
            ModifierType::Increment => Self {
                hue_increment: Some(value as i32),
                ..self
            },
            ModifierType::Decrement => Self {
                hue_increment: Some(-(value as i32)),
                ..self
            },
        }
    }

    /// Sets the saturation of a light.
    pub fn saturation(self, modifier_type: ModifierType, value: u8) -> Self {
        match modifier_type {
            ModifierType::Override => Self {
                saturation: Some(value),
                ..self
            },
            ModifierType::Increment => Self {
                saturation_increment: Some(value as i16),
                ..self
            },
            ModifierType::Decrement => Self {
                saturation_increment: Some(-(value as i16)),
                ..self
            },
        }
    }

    /// Sets the x and y coordinates in the color space to set the color of a light.
    ///
    /// If the modifier type is `Override`, the values must be between 0 and 1. If the modifier
    /// type is not `Override`, the values must be between 0 and 0.5.
    pub fn color_space_coordinates(
        self,
        modifier_type: CoordinateModifierType,
        value: (f32, f32),
    ) -> Self {
        match modifier_type {
            CoordinateModifierType::Override => Self {
                color_space_coordinates: Some(value),
                ..self
            },
            CoordinateModifierType::Increment => Self {
                color_space_coordinates_increment: Some(value),
                ..self
            },
            CoordinateModifierType::Decrement => Self {
                color_space_coordinates_increment: Some((-value.0, -value.1)),
                ..self
            },
            CoordinateModifierType::IncrementDecrement => Self {
                color_space_coordinates_increment: Some((value.0, -value.1)),
                ..self
            },
            CoordinateModifierType::DecrementIncrement => Self {
                color_space_coordinates_increment: Some((-value.0, value.1)),
                ..self
            },
        }
    }

    /// Sets the color temperature of a light.
    pub fn color_temperature(self, modifier_type: ModifierType, value: u16) -> Self {
        match modifier_type {
            ModifierType::Override => Self {
                color_temperature: Some(value),
                ..self
            },
            ModifierType::Increment => Self {
                color_temperature_increment: Some(value as i32),
                ..self
            },
            ModifierType::Decrement => Self {
                color_temperature_increment: Some(-(value as i32)),
                ..self
            },
        }
    }

    /// Sets the alert effect of a light.
    pub fn alert(self, value: Alert) -> Self {
        Self {
            alert: Some(value),
            ..self
        }
    }

    /// Sets the dynamic effect of a light.
    pub fn effect(self, value: Effect) -> Self {
        Self {
            effect: Some(value),
            ..self
        }
    }

    /// Sets the transition duration of state changes.
    ///
    /// This is given as a multiple of 100ms.
    pub fn transition_time(self, value: u16) -> Self {
        Self {
            transition_time: Some(value),
            ..self
        }
    }
}
