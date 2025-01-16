use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use ordered_float::OrderedFloat;
use crate::error::{NokhwaError, NokhwaResult};
use crate::ranges::{Range, ValidatableRange};

pub type PlatformSpecificControlId = u64;

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum ControlId {
    FocusMode,
    FocusAutoType,
    FocusAutoRange,
    FocusAbsolute,
    FocusRelative,
    FocusStatus,

    ExposureMode,
    ExposureBias,
    ExposureMetering,
    ExposureAbsolute,
    ExposureRelative,

    IsoMode,
    IsoSensitivity,

    ApertureAbsolute,
    ApertureRelative,

    WhiteBalanceMode,
    WhiteBalanceTemperature,

    ZoomContinuous,
    ZoomRelative,
    ZoomAbsolute,

    LightingMode,
    LightingStart,
    LightingStop,
    LightingStatus,

    Orientation,

    PlatformSpecific(PlatformSpecificControlId)
}

impl Display for ControlId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Control ID: {self:?}")
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Controls {
    controls: HashMap<ControlId, ControlBody>,
    values: HashMap<ControlId, ControlId>,
}

impl Controls {
    pub fn new(device_controls: HashMap<ControlId, ControlBody>) -> Self {
        Self {
            controls: device_controls,
        }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn control_value(&self, control_id: &ControlId) -> Option<&ControlBody> {
        self.controls.get(control_id)
    }

    pub fn set_control_value(&mut self, control_id: &ControlId, value: ControlValue) -> NokhwaResult<()> {
        // see if it exists
        if let Some(control) = self.controls.get_mut(control_id) {
            // FIXME: Remove this clone one day!
            control.set_value(value.clone())?;
        }
        Err(NokhwaError::SetPropertyError {
            property: control_id.to_string(),
            value: value.to_string(),
            error: "Not Found/Not Supported".to_string(),
        })
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct ControlBody {
    flags: HashSet<ControlFlags>,
    descriptor: ControlValueDescriptor,
    default_value: Option<ControlValue>,
}

impl ControlBody {
    pub fn new(control_flags: HashSet<ControlFlags>, control_value_descriptor: ControlValueDescriptor, default_value: Option<ControlValue>) -> Self {
        Self {
            flags: control_flags,
            descriptor: control_value_descriptor,
            default_value,
        }
    }

    pub fn flags(&self) -> &HashSet<ControlFlags> {
        &self.flags
    }

    pub fn descriptor(&self) -> &ControlValueDescriptor {
        &self.descriptor
    }

    pub fn default_value(&self) -> &Option<ControlValue> {
        &self.default_value
    }

    pub fn add_flag(&mut self, flag: ControlFlags) {
        self.flags.insert(flag);
    }

    pub fn remove_flag(&mut self, flag: ControlFlags) -> bool {
        self.flags.remove(&flag)
    }
}

#[derive(Copy, Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum ControlFlags {
    Disabled,
    Busy,
    ReadOnly,
    CascadingUpdates,
    Inactive,
    Slider,
    WriteOnly,
    Volatile,
    ContinuousChange,
    ExecuteOnWrite,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ControlValueDescriptor {
    Null,
    Integer(Range<i64>),
    BitMask,
    Float(Range<f64>),
    String,
    Boolean,
    // Array of any values of singular type
    Array(ControlValueDescriptor),
    // Menu(Enum) of valid choices
    // The keys are valid choices,
    // the values represent what the choice is (usually a string or int).
    Menu(HashMap<ControlValue, ControlValue>),
    // lmao u deal with it
    // max/min length set by range. step is ALWAYS zero.
    Binary(Range<u64>),
    // An area (Resolution) like type
    Area {
        width_limits: Range<i64>,
        height_limits: Range<i64>,
    },
}

impl ControlValueDescriptor {
    pub fn validate(&self, value: &ControlValue) -> bool {
        match self {
            ControlValueDescriptor::Null => {
                if let &ControlValue::Null = value {
                    return false
                }
            }
            ControlValueDescriptor::Integer(int_range) => {
                if let ControlValue::Integer(i) = value {
                    return int_range.validate(i)
                }
            }
            ControlValueDescriptor::BitMask => {
                if let &ControlValue::BitMask(_) = value {
                    return true
                }
            }
            ControlValueDescriptor::Float(float_range) => {
                if let ControlValue::Float(i) = value {
                    return float_range.validate(i)
                }
            }
            ControlValueDescriptor::String => {
                if let &ControlValue::String(_) = value {
                    return true
                }
            }
            ControlValueDescriptor::Boolean => {
                if let &ControlValue::Boolean(_) = value {
                    return true
                }
            }
            ControlValueDescriptor::Array(arr) => {
                if let &ControlValue::Array(_) = value {
                    return arr.is_valid_value(value)
                }
            }
            ControlValueDescriptor::Binary(size_limits) => {
                if let ControlValue::Binary(bin) = value {
                    return size_limits.validate(bin.len() as u64)
                }
            }
            ControlValueDescriptor::Menu(choices) => {
                if let ControlValue::EnumPick(choice) = value {
                    return choices.contains_key(choice)
                }
            }
            ControlValueDescriptor::Area { width_limits, height_limits } => {
                if let ControlValue::Area { width, height } = &value {
                    return width_limits.validate(width) && height_limits.validate(height)
                }
            }
        }
        false
    }
}
//
// #[derive(Clone, Debug, PartialEq)]
// pub enum ControlValuePrimitiveDescriptor {
//     Null,
//     Integer(Range<i64>),
//     BitMask,
//     Float(Range<f64>),
//     String,
//     Binary,
//     Boolean,
// }
//
// impl ControlValuePrimitiveDescriptor {
//     pub fn is_valid_primitive_value(&self, other: &ControlValuePrimitive) -> bool {
//         match self {
//             ControlValuePrimitiveDescriptor::Null => {
//                 if let ControlValuePrimitive::Null = other {
//                     return true
//                 }
//             }
//             ControlValuePrimitiveDescriptor::Integer(i) => {
//                 if let ControlValuePrimitive::Integer(v) = other {
//                     return i.validate(v)
//                 }
//             }
//             ControlValuePrimitiveDescriptor::BitMask => {
//                 if let ControlValuePrimitive::BitMask(_) = other {
//                     return true
//                 }
//             }
//             ControlValuePrimitiveDescriptor::Float(f) => {
//                 if let ControlValuePrimitive::Float(v) = other {
//                     return f.validate(v)
//                 }
//             }
//             ControlValuePrimitiveDescriptor::String => {
//                 if let ControlValuePrimitive::String(_) = other {
//                     return true
//                 }
//             }
//             ControlValuePrimitiveDescriptor::Boolean => {
//                 if let ControlValuePrimitive::Boolean(_) = other {
//                     return true
//                 }
//             }
//         }
//         false
//     }
//
//     pub fn is_valid_value(&self, other: &ControlValue) -> bool {
//         match self {
//             ControlValuePrimitiveDescriptor::Null => {
//                 if let ControlValue::Null = other {
//                     return true
//                 }
//             }
//             ControlValuePrimitiveDescriptor::Integer(i) => {
//                 if let ControlValue::Integer(v) = other {
//                     return i.validate(v)
//                 }
//             }
//             ControlValuePrimitiveDescriptor::BitMask => {
//                 if let ControlValue::BitMask(_) = other {
//                     return true
//                 }
//             }
//             ControlValuePrimitiveDescriptor::Float(f) => {
//                 if let ControlValue::Float(v) = other {
//                     return f.validate(v)
//                 }
//             }
//             ControlValuePrimitiveDescriptor::String => {
//                 if let ControlValue::String(_) = other {
//                     return true
//                 }
//             }
//             ControlValuePrimitiveDescriptor::Boolean => {
//                 if let ControlValue::Boolean(_) = other {
//                     return true
//                 }
//             }
//         }
//         false
//     }
// }
//
// #[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
// pub enum ControlValuePrimitive {
//     Null,
//     Integer(i64),
//     BitMask(i64),
//     Float(OrderedFloat<f64>),
//     String(String),
//     Boolean(bool),
// }
//
// impl From<ControlValuePrimitive> for ControlValue {
//     fn from(value: ControlValuePrimitive) -> Self {
//         match value {
//             ControlValuePrimitive::Null => ControlValue::Null,
//             ControlValuePrimitive::Integer(i) => ControlValue::Integer(i),
//             ControlValuePrimitive::BitMask(b) => ControlValue::BitMask(b),
//             ControlValuePrimitive::Float(f) => ControlValue::Float(f),
//             ControlValuePrimitive::String(s) => ControlValue::String(s),
//             ControlValuePrimitive::Boolean(b) => ControlValue::Boolean(b),
//         }
//     }
// }

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd)]
pub enum ControlValue {
    Null,
    Integer(i64),
    BitMask(i64),
    Float(OrderedFloat<f64>),
    String(String),
    Boolean(bool),
    Array(Vec<ControlValue>),
    Binary(Vec<u8>),
    EnumPick(Box<ControlValue>),
    Area {
        width: i64,
        height: i64,
    }
}

impl ControlValue {
    pub fn is_primitive(&self) -> bool {
        match self {
            ControlValue::Null |
            ControlValue::Integer(_) |
            ControlValue::BitMask(_) |
            ControlValue::Float(_) |
            ControlValue::String(_)|
            ControlValue::Boolean(_) |
            ControlValue::Binary(_) |
            ControlValue::Area { .. } => true,
            _ => false,
        }
    }

    // pub fn primitive_same_type(&self, other: &ControlValuePrimitive) -> bool {
    //     match other {
    //         ControlValuePrimitive::Null => {
    //             if let ControlValue::Null = self {
    //                 return true
    //             }
    //         }
    //         ControlValuePrimitive::Integer(_) => {if let ControlValue::Integer(_) = self {return true}}
    //         ControlValuePrimitive::BitMask(_) => {if let ControlValue::BitMask(_) = self {return true}}
    //         ControlValuePrimitive::Float(_) => {if let ControlValue::Float(_) = self {return true}}
    //         ControlValuePrimitive::String(_) => {if let ControlValue::String(_) = self {return true}}
    //         ControlValuePrimitive::Boolean(_) => {if let ControlValue::Boolean(_) = self {return true}}
    //     }
    //     false
    // }

    pub fn same_type(&self, other: &ControlValue) -> bool {
        match self {
            ControlValue::Null => {
                if let ControlValue::Null = other {
                    return true;
                }
            }
            ControlValue::Integer(_) => {if let ControlValue::Integer(_) = other {
                return true;
            }}
            ControlValue::BitMask(_) => {if let ControlValue::BitMask(_) = other {
                return true;
            }}
            ControlValue::Float(_) => {if let ControlValue::Float(_) = other {
                return true;
            }}
            ControlValue::String(_) => {if let ControlValue::String(_) = other {
                return true;
            }}
            ControlValue::Boolean(_) => {if let ControlValue::Boolean(_) = other {
                return true;
            }}
            ControlValue::Array(_) => {if let ControlValue::Array(_) = other {
                return true;
            }}
            ControlValue::EnumPick(_) => {if let ControlValue::EnumPick(_) = other {
                return true;
            }}
            ControlValue::Binary(_) => {if let ControlValue::Binary(_) = other {
                return true;
            }}
            ControlValue::Area { .. } => {if let ControlValue::Area { .. } = other {
                return true;
            }}
        }

        false
    }


}

impl Display for ControlValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Control Value: {self:?}")
    }
}
