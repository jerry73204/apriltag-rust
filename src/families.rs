//! Dictionary of tag families.
//!
//! It provides the dictionary of pre-generated tag families.
//! The images of pre-generated tags can be found at the official repositoy
//! [https://github.com/AprilRobotics/apriltag-imgs](https://github.com/AprilRobotics/apriltag-imgs).

use crate::{common::*, error::Error};

/// Represent a family of pre-generated tags.
///
/// It can be instantiated by calling member methods or by [Family::from_str].
///
/// ```rust
/// use apriltag::Family;
/// let family: Family = "tag16h5".parse().unwrap();
/// ```
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Family {
    pub(crate) ptr: NonNull<sys::apriltag_family_t>,
}

impl Family {
    /// Create Tag16h5 family.
    pub fn tag_16h5() -> Self {
        let ptr = unsafe { sys::tag16h5_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Create Tag25h9 family.
    pub fn tag_25h9() -> Self {
        let ptr = unsafe { sys::tag25h9_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Create Tag36h11 family.
    pub fn tag_36h11() -> Self {
        let ptr = unsafe { sys::tag36h11_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Create TagCircle21h7 family.
    pub fn tag_circle_21h7() -> Self {
        let ptr = unsafe { sys::tagCircle21h7_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Create TagCircle49h12 family.
    pub fn tag_circle_49h12() -> Self {
        let ptr = unsafe { sys::tagCircle49h12_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Create TagCustom48h12 family.
    pub fn tag_custom_48h12() -> Self {
        let ptr = unsafe { sys::tagCustom48h12_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Create TagStandard41h12 family.
    pub fn tag_standard_41h12() -> Self {
        let ptr = unsafe { sys::tagStandard41h12_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Create TagStandard52h13 family.
    pub fn tag_standard_52h13() -> Self {
        let ptr = unsafe { sys::tagStandard52h13_create() };
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Creates an instance from pointer.
    ///
    /// The pointer will be managed by the type. Do not run manual deallocation on the pointer.
    /// Panics if the pointer is null.
    pub unsafe fn from_raw(ptr: *mut sys::apriltag_family_t) -> Self {
        Self {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }

    /// Returns the underlying pointer.
    pub fn into_raw(self) -> NonNull<sys::apriltag_family_t> {
        ManuallyDrop::new(self).ptr
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

impl Drop for Family {
    fn drop(&mut self) {
        unsafe {
            let ptr = self.ptr.as_ptr();
            libc::free((*ptr).codes as *mut c_void);
            libc::free((*ptr).bit_x as *mut c_void);
            libc::free((*ptr).bit_y as *mut c_void);
            libc::free((*ptr).name as *mut c_void);
            libc::free(ptr as *mut c_void);
        }
    }
}
