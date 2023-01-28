//! Dictionary of tag families.
//!
//! It provides the dictionary of pre-generated tag families.
//! The images of pre-generated tags can be found at the official repositoy
//! [https://github.com/AprilRobotics/apriltag-imgs](https://github.com/AprilRobotics/apriltag-imgs).

use crate::error::Error;
use apriltag_sys as sys;
use std::{fmt::Debug, mem::ManuallyDrop, str::FromStr};

pub trait ApriltagFamily
where
    Self: Debug,
{
    fn into_raw(self) -> *mut sys::apriltag_family_t;
}

macro_rules! declare_family {
    ($name:ident, $init_fn:ident, $fini_fn:ident) => {
        #[derive(Debug)]
        #[repr(transparent)]
        pub struct $name {
            pub(crate) ptr: *mut sys::apriltag_family_t,
        }

        impl ApriltagFamily for $name {
            fn into_raw(self) -> *mut sys::apriltag_family_t {
                ManuallyDrop::new(self).ptr
            }
        }

        impl Default for $name {
            fn default() -> Self {
                unsafe {
                    Self {
                        ptr: sys::$init_fn(),
                    }
                }
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    sys::$fini_fn(self.ptr);
                }
            }
        }
    };
}

declare_family!(Tag16h5, tag16h5_create, tag16h5_destroy);
declare_family!(Tag25h9, tag25h9_create, tag25h9_destroy);
declare_family!(Tag36h11, tag36h11_create, tag36h11_destroy);
declare_family!(TagCircle21h7, tagCircle21h7_create, tagCircle21h7_destroy);
declare_family!(
    TagCircle49h12,
    tagCircle49h12_create,
    tagCircle49h12_destroy
);
declare_family!(
    TagStandard41h12,
    tagStandard41h12_create,
    tagStandard41h12_destroy
);
declare_family!(
    TagStandard52h13,
    tagStandard52h13_create,
    tagStandard52h13_destroy
);
declare_family!(
    TagCustom48h12,
    tagCustom48h12_create,
    tagCustom48h12_destroy
);

/// Represent a family of pre-generated tags.
///
/// It can be instantiated by calling member methods or by [Family::from_str].
///
/// ```rust
/// use apriltag::Family;
/// let family: Family = "tag16h5".parse().unwrap();
/// ```
#[derive(Debug)]
pub enum Family {
    Tag16h5(Tag16h5),
    Tag25h9(Tag25h9),
    Tag36h11(Tag36h11),
    TagCircle21h7(TagCircle21h7),
    TagCircle49h12(TagCircle49h12),
    TagStandard41h12(TagStandard41h12),
    TagStandard52h13(TagStandard52h13),
    TagCustom48h12(TagCustom48h12),
}

impl From<TagCustom48h12> for Family {
    fn from(v: TagCustom48h12) -> Self {
        Self::TagCustom48h12(v)
    }
}

impl From<TagStandard52h13> for Family {
    fn from(v: TagStandard52h13) -> Self {
        Self::TagStandard52h13(v)
    }
}

impl From<TagStandard41h12> for Family {
    fn from(v: TagStandard41h12) -> Self {
        Self::TagStandard41h12(v)
    }
}

impl From<TagCircle49h12> for Family {
    fn from(v: TagCircle49h12) -> Self {
        Self::TagCircle49h12(v)
    }
}

impl From<TagCircle21h7> for Family {
    fn from(v: TagCircle21h7) -> Self {
        Self::TagCircle21h7(v)
    }
}

impl From<Tag36h11> for Family {
    fn from(v: Tag36h11) -> Self {
        Self::Tag36h11(v)
    }
}

impl From<Tag25h9> for Family {
    fn from(v: Tag25h9) -> Self {
        Self::Tag25h9(v)
    }
}

impl From<Tag16h5> for Family {
    fn from(v: Tag16h5) -> Self {
        Self::Tag16h5(v)
    }
}

impl ApriltagFamily for Family {
    fn into_raw(self) -> *mut sys::apriltag_family_t {
        match self {
            Family::Tag16h5(family) => family.into_raw(),
            Family::Tag25h9(family) => family.into_raw(),
            Family::Tag36h11(family) => family.into_raw(),
            Family::TagCircle21h7(family) => family.into_raw(),
            Family::TagCircle49h12(family) => family.into_raw(),
            Family::TagStandard41h12(family) => family.into_raw(),
            Family::TagStandard52h13(family) => family.into_raw(),
            Family::TagCustom48h12(family) => family.into_raw(),
        }
    }
}

impl Family {
    /// Create Tag16h5 family.
    pub fn tag_16h5() -> Self {
        Tag16h5::default().into()
    }

    /// Create Tag25h9 family.
    pub fn tag_25h9() -> Self {
        Tag25h9::default().into()
    }

    /// Create Tag36h11 family.
    pub fn tag_36h11() -> Self {
        Tag36h11::default().into()
    }

    /// Create TagCircle21h7 family.
    pub fn tag_circle_21h7() -> Self {
        TagCircle21h7::default().into()
    }

    /// Create TagCircle49h12 family.
    pub fn tag_circle_49h12() -> Self {
        TagCircle49h12::default().into()
    }

    /// Create TagCustom48h12 family.
    pub fn tag_custom_48h12() -> Self {
        TagCustom48h12::default().into()
    }

    /// Create TagStandard41h12 family.
    pub fn tag_standard_41h12() -> Self {
        TagStandard41h12::default().into()
    }

    /// Create TagStandard52h13 family.
    pub fn tag_standard_52h13() -> Self {
        TagStandard52h13::default().into()
    }
}

impl FromStr for Family {
    type Err = Error;

    /// Creates a [Family](Family) instance by family name.
    ///
    /// Supported names:
    /// - tag16h5
    /// - tag25h9
    /// - tag36h11
    /// - tagCircle21h7
    /// - tagCircle49h12
    /// - tagStandard41h12
    /// - tagStandard52h13
    /// - tagCustom48h12
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let family = match text {
            "tag16h5" => Self::tag_16h5(),
            "tag25h9" => Self::tag_25h9(),
            "tag36h11" => Self::tag_36h11(),
            "tagCircle21h7" => Self::tag_circle_21h7(),
            "tagCircle49h12" => Self::tag_circle_49h12(),
            "tagStandard41h12" => Self::tag_standard_41h12(),
            "tagStandard52h13" => Self::tag_standard_52h13(),
            "tagCustom48h12" => Self::tag_custom_48h12(),
            _ => return Err(Error::ParseFamilyStringError(text.to_owned())),
        };
        Ok(family)
    }
}
