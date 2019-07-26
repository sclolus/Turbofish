use core::convert::TryFrom;
use core::fmt;
use core::mem;
use core::cmp::Ordering;
use errno::Errno;

use super::posix_consts::{NAME_MAX, PATH_MAX};

/// Newtype of filename
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Filename(pub [u8; NAME_MAX], pub usize);

impl TryFrom<&str> for Filename {
    type Error = Errno;
    fn try_from(s: &str) -> Result<Self, Errno> {
        let mut n = [0; NAME_MAX];
        if s.len() >= NAME_MAX {
            return Err(Errno::Enametoolong);
        } else {
            for (n, c) in n.iter_mut().zip(s.bytes()) {
                *n = c;
            }
            Ok(Self(n, s.len()))
        }
    }
}

impl Filename {
    pub fn len(&self) -> usize {
        self.1
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            let slice: &[u8] = core::slice::from_raw_parts(&self.0 as *const u8, self.1);
            core::str::from_utf8_unchecked(slice)
        }
    }
}

impl Default for Filename {
    fn default() -> Self {
        Self([0; NAME_MAX], 0)
    }
}

impl PartialEq for Filename {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1 &&
        self.0[..self.1] == other.0[..self.1]
    }
}

impl PartialEq<&str> for Filename {
    fn eq(&self, &other: &&str) -> bool {
        self.as_str() == other
    }
}

impl PartialOrd for Filename {
    fn partial_cmp(&self, other: &Filename) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Filename {
    fn cmp(&self, other: &Filename) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Eq for Filename {}

/// Debug boilerplate of filename
impl fmt::Debug for Filename {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.as_str())
    }
}

impl fmt::Display for Filename {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.as_str())
    }
}

pub struct Path {
    components: Vec<Filename>,
    total_length: usize,
    is_absolute: bool,
}

impl Path {
    fn null_path() -> Self {
        Self {
            components: Vec::new(),
            total_length: 0,
            is_absolute: false,
        }
    }

    pub fn new() -> Self {
        Self::null_path()
    }

    fn set_absolute(&mut self, value: bool) -> &mut Self {
        self.is_absolute = value;
        self
    }

    pub fn is_absolute(&self) -> bool {
        self.is_absolute
    }

    pub fn depth(&self) -> usize {
        self.components.len()
    }

    pub fn len(&self) -> usize {
        self.total_length
    }

    pub fn push(&mut self, component: Filename) -> &mut Self { // this is an Option return type actually
        self.total_length += component.len();
        self.components.push(component);
        self
    }

    pub fn pop(&mut self) -> Option<Filename> {
        let ret = self.components.pop()?;
        self.total_length -= ret.len();
        Some(ret)
    }
}

impl TryFrom<&str> for Path {
    type Error = Errno;
    fn try_from(s: &str) -> Result<Self, Errno> {
        let is_absolute = s.starts_with('/');
        let components = s.split('/').filter(|&x| x != "");

        let mut path = Path::new();

        path.set_absolute(is_absolute);
        for component in components {
            let filename = Filename::try_from(component)?;

            path.push(filename);
        }
        Ok(path)
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        if self.components.len() != other.components.len() {
            return false
        }

        let a = self.components.iter();
        let b = other.components.iter();

        a.zip(b)
            .find(|(&a, &b)| a != b)
            .is_none()
    }
}

impl Eq for Path {}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Path) -> Option<Ordering> {
        let a = self.components.iter();
        let b = other.components.iter();

        a.partial_cmp(b)
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Path) -> Ordering {
        let a = self.components.iter();
        let b = other.components.iter();

        a.cmp(b)
    }
}
