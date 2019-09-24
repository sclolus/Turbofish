use super::posix_consts::{NAME_MAX, PATH_MAX};
use super::SysResult;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::convert::{TryFrom, TryInto};
use core::fmt;
use fallible_collections::FallibleVec;
use libc_binding::{c_char, Errno};
use try_clone_derive::TryClone;

#[derive(Debug, Clone, TryClone)]
pub struct Path {
    components: Vec<Filename>,
    total_length: usize,
    is_absolute: bool,
}

impl Path {
    pub fn root() -> Self {
        // This should not failed
        Self::try_from("/").expect("path / creation failed")
    }

    pub fn write_path_in_buffer(&self, buf: &mut [c_char]) -> SysResult<u32> {
        let size = buf.len();
        let mut i = 0;
        for b in self.iter_bytes() {
            // keep a place for the \0
            if i >= size - 1 {
                return Err(Errno::ERANGE);
            }
            buf[i] = *b;
            i += 1;
        }
        if i > 0 &&
            // do not erase the slash if the path is /
            !(i == 1 && buf[0] == '/' as c_char)
        {
            buf[i - 1] = '\0' as c_char;
        }
        Ok(i as u32)
    }

    /// iterator over the char of the path
    /// WARNING: return a trailing slash
    pub fn iter_bytes(&self) -> impl Iterator<Item = &c_char> {
        if self.is_absolute() {
            Some(&('/' as c_char))
        } else {
            None
        }
        .into_iter()
        .chain(
            self.components
                .iter()
                .flat_map(|filename| filename.iter_bytes().chain(Some('/' as c_char).iter())),
        )
    }

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

    pub fn set_absolute(&mut self, value: bool) -> SysResult<&mut Self> {
        if !self.is_absolute() && value && self.total_length == PATH_MAX - 1 {
            return Err(Errno::ENAMETOOLONG);
        }
        self.is_absolute = value;
        self.update_len();
        Ok(self)
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

    pub fn filename(&self) -> Option<&Filename> {
        self.components.iter().last()
    }

    pub fn parent(&self) -> SysResult<Path> {
        let mut components = self.components();
        components.next_back();

        Self::try_from(components)
    }

    pub fn ancestors(&self) -> Ancestors {
        Ancestors::from_path(self)
    }

    pub fn push(&mut self, component: Filename) -> SysResult<&mut Self> {
        // this is an Option return type actually
        let total_length;
        if self.depth() != 0 {
            total_length = self.total_length + component.len() + 1;
        } else {
            total_length = self.total_length + component.len();
        }

        if total_length > PATH_MAX - 1 {
            return Err(Errno::ENAMETOOLONG);
        }
        self.total_length = total_length;
        self.components.try_push(component)?;
        Ok(self)
    }

    fn len_from_components(&self) -> usize {
        let mut len = 0;

        if self.is_absolute() {
            len += 1;
        }
        if self.depth() != 0 {
            len += self.components.iter().map(|x| x.len()).sum::<usize>() + self.depth() - 1;
        }

        len
    }

    fn update_len(&mut self) -> usize {
        self.total_length = self.len_from_components();
        self.total_length
    }

    pub fn pop(&mut self) -> Option<Filename> {
        let ret = self.components.pop()?;
        if self.depth() != 0 {
            self.total_length -= 1;
        }
        self.total_length -= ret.len();
        Some(ret)
    }

    pub fn components(&self) -> Components {
        Components::from_path(self)
    }

    pub fn chain(&mut self, other: Path) -> SysResult<&mut Self> {
        if self == &Path::null_path() {
            *self = other;
            Ok(self)
        } else {
            self.chain_components(other.components())?;
            Ok(self)
        }
    }

    pub fn chain_components<'a, T>(&mut self, comps: T) -> SysResult<&mut Self>
    where
        T: Iterator<Item = &'a Filename>,
    {
        for comp in comps {
            self.push(*comp)?;
        }
        Ok(self)
    }

    // pub fn replace(&mut self, offset: usize, other: Path) -> SysResult<&mut Self> {
    //     let stack = self.try_clone()?; // well need to copy data temporary.
    //     let (comps_begin, comps_end) = stack.components().divide_at(offset);

    //     // comps_begin.next_back();
    //     // comps_end.next();

    //     println!("Begin: {:?}", comps_begin);
    //     println!("End: {:?}", comps_end);
    //     let mut new = Self::try_from(comps_begin)?;

    //     new.chain(other)?.chain_components(comps_end)?;
    //     *self = new;
    //     Ok(self)
    // }
}

/// Newtype of filename
#[derive(Copy, Clone, TryClone)]
#[repr(C)]
pub struct Filename(pub [c_char; NAME_MAX as usize + 1], pub usize);

impl TryFrom<&str> for Filename {
    type Error = Errno;
    fn try_from(s: &str) -> SysResult<Self> {
        let mut n = [0 as c_char; NAME_MAX as usize + 1];
        if s.bytes().find(|&b| b == '/' as u8).is_some() || s.len() == 0 {
            return Err(Errno::EINVAL);
        }
        if s.len() > NAME_MAX as usize {
            return Err(Errno::ENAMETOOLONG);
        } else {
            for (n, c) in n.iter_mut().zip(s.bytes()) {
                *n = c as c_char;
            }
            Ok(Self(n, s.len()))
        }
    }
}

impl Filename {
    pub fn new(mut name: [c_char; NAME_MAX as usize + 1], len: usize) -> Self {
        // add the \0 at end of filename
        name[len] = '\0' as c_char;
        Self(name, len)
    }
    pub fn len(&self) -> usize {
        self.1
    }
    pub fn iter_bytes(&self) -> impl Iterator<Item = &c_char> {
        self.0[0..self.1].iter()
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            let slice: &[u8] =
                core::slice::from_raw_parts(&self.0 as *const c_char as *const u8, self.1);
            core::str::from_utf8_unchecked(slice)
        }
    }

    /// Creates a new Filename instance from a &str that's guaranted to be a valid
    /// filename.
    ///
    /// Panic:
    /// - Panics if `s` is not a valid filename.
    pub fn from_str_unwrap(s: &str) -> Self {
        match Self::try_from(s) {
            Err(e) => panic!("String slice: {} should be a valid Filename: {:?}", s, e),
            Ok(filename) => filename,
        }
    }
}

impl Default for Filename {
    fn default() -> Self {
        Self([0; NAME_MAX as usize + 1], 0)
    }
}

impl PartialEq for Filename {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
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
        Ok(write!(f, "{:?}", self.as_str())?)
    }
}

impl fmt::Display for Filename {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(write!(f, "{}", self.as_str())?)
    }
}

use core::ops::Range;
#[derive(Debug, Clone, Eq, PartialEq)] // Copy right ?
pub struct Components<'a> {
    path: &'a Path,
    current: Option<Range<usize>>,
}

impl<'a> Components<'a> {
    fn from_path(path: &'a Path) -> Self {
        let depth = path.depth();
        let current = if depth == 0 { None } else { Some(0..depth - 1) };

        Self { path, current }
    }

    /// Determines if the path composed by the remaining components of the path is absolute.
    fn is_absolute(&self) -> bool {
        if self.current.is_none() {
            false
        } else {
            self.path.is_absolute() && self.current.as_ref().unwrap().start == 0
        }
    }

    // pub fn divide_at(mut self, offset: usize) -> SysResult<(Self, Self)> {
    //     if !self.current.as_ref().unwrap_or(&(0..0)).contains(&offset) {
    //         self.current = None;
    //         let (a, b) = (self.try_clone()?, self.try_clone()?);
    //         return (a, b);
    //     }

    //     let (mut a, mut b) = (self.try_clone()?, self.try_clone()?);
    //     a.current = dbg!(a.current.map(|range| range.start..offset));
    //     b.current = b.current.map(|range| offset..range.end);
    //     (a, b)
    // }
}

impl<'a> Iterator for Components<'a> {
    type Item = &'a Filename;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            return None;
        }

        let current = self.current.as_ref().unwrap();

        let start = current.start;
        let end = current.end;

        if start > end {
            None
        } else {
            self.current = start.checked_add(1).map(|new_start| new_start..end);
            Some(&self.path.components[start])
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let current = self.current.as_ref().unwrap_or(&(0..0));
        let start = current.start;
        let end = current.end;
        let len = end.checked_sub(start).unwrap_or(0);
        (len, Some(len))
    }
}

impl<'a> ExactSizeIterator for Components<'a> {}

impl<'a> DoubleEndedIterator for Components<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            return None;
        }

        let current = self.current.as_ref().unwrap();

        let start = current.start;
        let end = current.end;

        if start > end {
            None
        } else {
            self.current = end.checked_sub(1).map(|new_end| start..new_end);
            Some(&self.path.components[end])
        }
    }
}

pub struct Ancestors<'a> {
    path: &'a Path,
    left_ancestors: Range<usize>,
}

impl<'a> Ancestors<'a> {
    pub fn from_path(path: &'a Path) -> Self {
        let left_ancestors = 1..path.depth();

        Self {
            path,
            left_ancestors,
        }
    }
}

impl<'a> Iterator for Ancestors<'a> {
    type Item = Path;
    fn next(&mut self) -> Option<Self::Item> {
        let current = &self.left_ancestors;

        let start = current.start;
        let end = current.end;
        if start > end {
            None
        } else {
            let depth = self.path.depth();
            let mut components = self.path.components();
            for _ in 0..depth - start {
                components.next_back();
            }
            let path = components.try_into().ok();
            let res = start.checked_add(1).map(|new_start| new_start..end);
            if res.is_none() {
                return None;
            }

            self.left_ancestors = res.unwrap();
            path
        }
    }
}

impl<'a> DoubleEndedIterator for Ancestors<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let current = &self.left_ancestors;

        let start = current.start;
        let end = current.end;
        if start > end {
            None
        } else {
            let depth = self.path.depth();
            let mut components = self.path.components();
            for _ in 0..depth - end {
                components.next_back();
            }
            let path = components.try_into().ok();
            let res = end.checked_sub(1).map(|new_end| start..new_end);
            if res.is_none() {
                return None;
            }

            self.left_ancestors = res.unwrap();
            path
        }
    }
}

impl<'a> From<&'a Path> for Components<'a> {
    fn from(value: &'a Path) -> Components<'a> {
        value.components()
    }
}

impl<'a> TryFrom<Components<'a>> for Path {
    type Error = Errno;
    fn try_from(comps: Components<'a>) -> SysResult<Self> {
        let mut path = Path::new();

        path.set_absolute(comps.is_absolute())?;
        for filename in comps {
            path.push(*filename)?;
        }
        Ok(path)
    }
}

impl TryFrom<&str> for Path {
    type Error = Errno;
    fn try_from(s: &str) -> SysResult<Self> {
        if s.len() > PATH_MAX - 1 {
            return Err(Errno::ENAMETOOLONG);
        }
        if s.len() == 0 {
            return Err(Errno::EINVAL);
        }
        let is_absolute = s.starts_with('/');
        let components = s.split('/').filter(|&x| x != "");

        let mut path = Path::new();

        path.set_absolute(is_absolute)?;
        for component in components {
            let filename = Filename::try_from(component)?;
            path.push(filename)?;
        }
        Ok(path)
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len()
            || self.depth() != other.depth()
            || self.is_absolute() != other.is_absolute()
        {
            return false;
        }

        let a = self.components.iter();
        let b = other.components.iter();

        a.zip(b).find(|(&a, &b)| a != b).is_none()
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

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_absolute() {
            write!(f, "/")?;
        }
        let depth = self.depth();
        for (index, component) in self.components().enumerate() {
            write!(f, "{}", component)?;
            if index + 1 != depth {
                write!(f, "/")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iter_bytes() {
        let p = "/a/b/c/";
        let path = Path::try_from(p).unwrap();
        let path_collected: Vec<c_char> = path.iter_bytes().map(|c| *c).collect();
        let p_collected: Vec<c_char> = p.bytes().map(|c| c as c_char).collect();
        assert_eq!(path_collected, p_collected);
        // panic!("{:?}", path_collected);
        // for (a, b) in .zip(p.bytes()) {
        //     assert_eq!(*a as u8, b);
        // }
    }

    macro_rules! make_test {
        (pass, $test_name: ident, $body: tt) => {
            #[test]
            fn $test_name() {
                $body
            }
        };
        (fail, $test_name: ident, $body: tt) => {
            #[test]
            #[should_panic]
            fn $test_name() {
                $body
            }
        };
    }

    macro_rules! make_filename_creation_test {
        ($body: block, $test_name: ident) => {
            make_test! {pass, $test_name, {
                Filename::try_from($body.as_str()).unwrap();
            }
            }
        };
        (fail, $body: block, $test_name: ident) => {
            make_test! {fail, $test_name, {
                Filename::try_from($body.as_str()).unwrap();
            }
            }
        };
        ($filename: expr, $test_name: ident) => {
            make_test! {pass, $test_name, {
                Filename::try_from($filename).unwrap();
            }
            }
        };
        (fail, $filename: expr, $test_name: ident) => {
            make_test! {fail, $test_name, {
                Filename::try_from($filename).unwrap();
            }
            }
        };
    }

    make_filename_creation_test! {fail, {
        let make_component = |count: usize| {
            let mut s = String::new();

            for _ in 0..count {
                s.push_str("a");
            }
            s
        };

        make_component(0)
    }, test_filename_posix_filename_cant_be_zero_len
    }

    make_filename_creation_test! {fail, {
        let make_component = |count: usize| {
            let mut s = String::new();

            for _ in 0..count {
                s.push_str("a");
            }
            s
        };

        make_component(NAME_MAX + 1)
    }, test_filename_posix_filename_cant_be_greater_than_name_max
    }

    make_filename_creation_test! {fail, {
        use std::str::FromStr;
        String::from_str("aaa/bbb.txt").expect("This should never happened") // the expect kind of breaks the test but hey, that should not happen anyway
    }, test_filename_posix_filename_cant_be_have_slash
    }

    make_filename_creation_test! {{
        let make_component = |count: usize| {
            let mut s = String::new();

            for _ in 0..count {
                s.push_str("a");
            }
            s
        };

        make_component(NAME_MAX)
    }, test_filename_posix_filename_can_be_name_max
    }

    make_filename_creation_test! {{
        let make_component = |count: usize| {
            let mut s = String::new();

            for _ in 0..count {
                s.push_str("a");
            }
            s
        };

        make_component(1)
    }, test_filename_posix_filename_can_be_one
    }

    make_test! {pass, test_path_root_path_is_absolute, {
        let path = Path::try_from("/").unwrap();
        assert!(path.is_absolute)
    }}

    make_test! {pass, test_path_root_path_has_zero_depth, {
        let path = Path::try_from("/").unwrap();
        assert!(path.depth() == 0)
    }}

    make_test! {pass, test_path_root_path_has_one_len, {
        let path = Path::try_from("/").unwrap();
        assert!(path.len() == 1)
    }}

    macro_rules! make_path_len_test {
        ($path: expr, $test_name: ident) => {
            make_test! {pass, $test_name, {
                let path_len = $path.len();
                let path = Path::try_from($path).unwrap();

                assert_eq!(path.len(), path_len);
            }
            }
        };
    }

    make_path_len_test! {"a", test_path_len_a_path}
    make_path_len_test! {"/a", test_path_len_root_a_path}
    make_path_len_test! {"a/b", test_path_len_a_b_path}
    make_path_len_test! {"/a/b", test_path_len_root_a_b_path}
    make_path_len_test! {"a/b/c", test_path_len_a_b_c_path}
    make_path_len_test! {"/a/b/c", test_path_len_root_a_b_c_path}
    make_path_len_test! {"a/bb/ccc", test_path_len_a_bb_ccc_path}
    make_path_len_test! {"/a/bb/ccc", test_path_len_root_a_bb_ccc_path}

    macro_rules! make_path_creation_test {
        ($body: block, $test_name: ident) => {
            make_test! {pass, $test_name, {
                Path::try_from($body.as_str()).unwrap();
            }
            }
        };
        (fail, $body: block, $test_name: ident) => {
            make_test! {fail, $test_name, {
                Path::try_from($body.as_str()).unwrap();
            }
            }
        };
        ($path: expr, $test_name: ident) => {
            make_test! {pass, $test_name, {
                Path::try_from($path).unwrap();
            }
            }
        };
        (fail, $path: expr, $test_name: ident) => {
            make_test! {fail, $test_name, {
                Path::try_from($path).unwrap();
            }
            }
        };
    }

    make_path_creation_test! {"////a/b/c", test_path_posix_path_can_have_multiple_beginning_slashes}
    make_path_creation_test! {"a/b/c////", test_path_posix_path_can_have_multiple_trailing_slashes}
    make_path_creation_test! {"/a////b//////////////////c/d//e/f///g//", test_path_posix_path_can_have_multiple_slashes}
    make_path_creation_test! {"/", test_path_posix_path_can_have_root_zero_filenames}
    make_path_creation_test! {fail, {
        let make_component = |count: usize| {
            let mut s = String::new();

            for _ in 0..count {
                s.push_str("a");
            }
            s
        };
        let mut path = String::new();
        let mut current_count = 0;

        loop {
            let additional_count;
            additional_count = NAME_MAX + 1;

            if current_count + additional_count > PATH_MAX - 1 {
                path.push_str("/");
                path.push_str(&make_component((PATH_MAX - 1) - current_count));
                break
            } else {
                path.push_str("/");
                path.push_str(&make_component(NAME_MAX));
                current_count += additional_count;
            }
        }
        path
    }, test_path_posix_path_cant_be_greater_than_path_max}

    make_path_creation_test! {{
        let make_component = |count: usize| {
            let mut s = String::new();

            for _ in 0..count {
                s.push_str("a");
            }
            s
        };
        let mut path = String::new();
        let mut current_count = 0;

        loop {
            let additional_count;
            additional_count = NAME_MAX + 1;

            if current_count + additional_count > PATH_MAX - 1 {
                path.push_str("/");
                path.push_str(&make_component((PATH_MAX - 1) - current_count - 1));
                break
            } else {
                path.push_str("/");
                path.push_str(&make_component(NAME_MAX));
                current_count += additional_count;
            }
        }
        path
    }, test_path_posix_path_can_have_len_path_max_minus_one}

    fn make_relative_str_path_of_length(length: usize) -> String {
        let make_component = |count: usize| {
            let mut s = String::new();

            for _ in 0..count {
                s.push_str("a");
            }
            s
        };
        let mut path = String::new();
        let mut current_count = 0;

        loop {
            let additional_count;
            additional_count = NAME_MAX + 1;

            if current_count + additional_count > length {
                path.push_str(&make_component((length) - current_count));
                break;
            } else {
                path.push_str(&make_component(NAME_MAX));
                path.push_str("/");
                current_count += additional_count;
            }
        }
        let path_str = path;
        let path = Path::try_from(path_str.as_str()).unwrap();
        assert_eq!(path.len(), path_str.len());
        path_str
    }

    make_test! {fail, test_path_posix_path_cant_be_greater_than_path_max_after_setting_to_absolute, {
        let make_component = |count: usize| {
            let mut s = String::new();

            for _ in 0..count {
                s.push_str("a");
            }
            s
        };
        let mut path = String::new();
        let mut current_count = 0;

        loop {
            let additional_count;
            additional_count = NAME_MAX + 1;

            if current_count + additional_count > PATH_MAX - 1 {
                path.push_str(&make_component((PATH_MAX - 1) - current_count));
                break
            } else {
                path.push_str(&make_component(NAME_MAX));
                path.push_str("/");
                current_count += additional_count;
            }
        }
        let mut path = Path::try_from(path.as_str()).unwrap();
        path.set_absolute(true).unwrap();
    }}

    make_test! {pass, test_path_parent_method, {
        let mut paths = Vec::new();
        let mut path = Path::new();

        for alpha in 0..5 {
            let mut string = String::new();
            let c = Some((alpha + 'a' as u8) as char);
            string.extend(c.iter());
            let filename = Filename::try_from(string.as_str()).unwrap();
            path.push(filename).unwrap();
            paths.push(path.clone());
        }
        paths.pop();
        loop {
            println!("{}", path);
            if path.depth() == 0 {
                break
            }
            let test_path = paths.pop().unwrap_or(Path::null_path());
            path = path.parent().expect("WOOT");

            assert_eq!(path, test_path);
        }
    }}

    macro_rules! make_path_chain_test {
        // please rewrite this, this is getting stupid, DRY
        ($make_path_pair: block, $test_name: ident) => {
            make_test! {pass, $test_name, {
                use std::convert::TryInto;
                let (a, b) = $make_path_pair;
                let (mut a, b): (Path, Path) =  (a.as_str().try_into().unwrap(), b.as_str().try_into().unwrap());
                a.chain(b).unwrap();
            }}
        };

        (fail, $make_path_pair: block, $test_name: ident) => {
            make_test! {fail, $test_name, {
                use std::convert::TryInto;
                let (a, b) = $make_path_pair;
                let (mut a, b): (Path, Path) =  (a.as_str().try_into().unwrap(), b.as_str().try_into().unwrap()); //creat somemacro to report test macro bad uses.

                a.chain(b).unwrap();
            }}
        };

        ($make_path_pair: block, $make_test_path: block, $test_name: ident) => {
            make_test! {pass, $test_name, {
                use std::convert::TryInto;
                let (a, b) = $make_path_pair;
                let (mut a, b): (Path, Path) =  (a.as_str().try_into().unwrap(), b.as_str().try_into().unwrap());
                let test_path: Path = ($make_test_path).try_into().unwrap();
                assert_eq!(a.chain(b).unwrap(), &test_path);
            }}
        };

        (fail, $make_path_pair: block, $make_test_path: block, $test_name: ident) => {
            make_test! {fail, $test_name, {
                use std::convert::TryInto;
                let (a, b) = $make_path_pair;
                let (mut a, b): (Path, Path) =  (a.as_str().try_into().unwrap(), b.as_str().try_into().unwrap()); //creat somemacro to report test macro bad uses.

                let test_path: Path = ($make_test_path).try_into().unwrap();
                assert_eq!(a.chain(b).unwrap(), &test_path);
            }}
        };
        ($make_path_pair: expr, $make_test_path: block, $test_name: ident) => {
            make_test! {pass, $test_name, {
                use std::convert::TryInto;
                let (a, b) = $make_path_pair;
                let (mut a, b): (Path, Path) =  (a.try_into().unwrap(), b.try_into().unwrap());
                let test_path: Path = ($make_test_path).try_into().unwrap();
                assert_eq!(a.chain(b).unwrap(), &test_path);
            }}
        };

        (fail, $make_path_pair: expr, $make_test_path: block, $test_name: ident) => {
            make_test! {fail, $test_name, {
                use std::convert::TryInto;
                let (a, b) = $make_path_pair;
                let (mut a, b): (Path, Path) =  (a.try_into().unwrap(), b.try_into().unwrap()); //creat somemacro to report test macro bad uses.

                let test_path: Path = ($make_test_path).try_into().unwrap();
                assert_eq!(a.chain(b).unwrap(), &test_path);
            }}
        };
    }

    make_path_chain_test! {("a/", "b"),
    {"a/b"},
    test_path_chain_a_b}

    make_path_chain_test! {("/a/", "b"),
    {"/a/b"},
    test_path_chain_root_a_b}

    make_path_chain_test! {("/a/b/", "b/c/d/"),
    {"/a/b/b/c/d/"},
    test_path_chain_root_a_b_b_c_d}

    make_path_chain_test! {("/a/b/", "b/c/d/"),
    {"/a/b/b/c/d/"},
    test_path_chain_root_a_b__b_c_d}

    make_path_chain_test! {fail, {
        let a = make_relative_str_path_of_length(PATH_MAX - 1);
        let b = make_relative_str_path_of_length(1);
        (a, b)
    }, test_path_chain_cant_create_bigger_path_than_posix_says}
    make_path_chain_test! {{
        let a = make_relative_str_path_of_length(PATH_MAX - 3);
        let b = make_relative_str_path_of_length(1);
        (a, b)
    }, test_path_chain_can_create_a_path_of_length_path_max_minus_three}

    #[test]
    fn test_components_iter_basic() {
        let path = Path::try_from("a/b/c/d/e/f/g/h").unwrap();
        let expected_filenames = ["a", "b", "c", "d", "e", "f", "g", "h"];
        let filenames = expected_filenames
            .iter()
            .map(|&f| Filename::try_from(f).unwrap())
            .collect::<Vec<Filename>>();
        let components = Components::from_path(&path);

        assert_eq!(filenames.iter().count(), components.clone().count());
        assert_eq!(
            filenames.iter().rev().count(),
            components.clone().rev().count()
        );
        assert!(filenames
            .iter()
            .zip(components.clone())
            .all(|(expect, component)| expect == component));
        // The rev method uses the DoubleEndedIterator implementation.
        assert!(filenames
            .iter()
            .rev()
            .zip(components.clone().rev())
            .all(|(expect, component)| expect == component));
    }

    macro_rules! make_components_iteration_test {
        ($path: expr, $test_name: ident) => {
            make_test! {pass, $test_name, {
                let path = Path::try_from($path).unwrap();
                let expected_filenames = $path.split('/').filter(|&x| x != "").collect::<Vec<&str>>();
                let _filenames = expected_filenames.iter().map(|&f| Filename::try_from(f).unwrap()).collect::<Vec<Filename>>();
                let _components = Components::from_path(&path);

            }
            }
        };
        (fail, $path: expr, $test_name: ident) => {
            make_test! {fail, $test_name, {
                let path = Path::try_from($path).unwrap();
                let expected_filenames = $path.split('/').filter(|&x| x != "").collect::Vec<&str>();
                let filenames = expected_filenames.iter().map(|&f| Filename::try_from(f).unwrap()).collect::<Vec<Filename>>();
                let components = Components::from_path(&path);

                assert_eq!(filenames.iter().count(), components.clone().count());
                assert_eq!(filenames.iter().rev().count(), components.clone().rev().count());
                assert!(filenames.iter().zip(components.clone()).all(|(expect, component)| expect == component));
                // The rev method uses the DoubleEndedIterator implementation.
                assert!(filenames.iter().rev().zip(components.clone().rev()).all(|(expect, component)| expect == component));
            }
            }
        };
    }

    make_components_iteration_test!("a", test_components_iter_basic_a);
    make_components_iteration_test!("a/b", test_components_iter_basic_a_b);
    make_components_iteration_test!("a/b/c", test_components_iter_basic_a_b_c);
    make_components_iteration_test!("a/b/c/d", test_components_iter_basic_a_b_c_d);
    make_components_iteration_test!("a/b/c/d/e", test_components_iter_basic_a_b_c_d_e);
    make_components_iteration_test!("a/b/c/d/e/f", test_components_iter_basic_a_b_c_d_e_f);
    make_components_iteration_test!("a/b/c/d/e/f/g", test_components_iter_basic_a_b_c_d_e_f_g);
    make_components_iteration_test!(
        "a/b/c/axiom/e/f/g/h",
        test_components_iter_basic_a_b_c_axiom_e_f_g_h
    );
    make_components_iteration_test!(
        "a/b/c/d/e/f/g/h/i",
        test_components_iter_basic_a_b_c_d_e_f_g_h_i
    );
    make_components_iteration_test!(
        "a/b/c/d/e/f/g/h/i/k",
        test_components_iter_basic_a_b_c_d_e_f_g_h_i_k
    );
    make_components_iteration_test!(
        "a/b/c/d/e/f/g/h/i/k/8",
        test_components_iter_basic_a_b_c_d_e_f_g_h_i_k_8
    );

    #[allow(unused_macros)]
    macro_rules! make_components_iteration_test {
        ($path: expr, $test_name: ident) => {
            make_test! {pass, $test_name, {
                let path = Path::try_from($path).unwrap();
                let expected_filenames = $path.split('/').filter(|&x| x != "").collect::<Vec<&str>>();
                let filenames = expected_filenames.iter().map(|&f| Filename::try_from(f).unwrap()).collect::<Vec<Filename>>();
                let components = Components::from_path(&path);

            }
            }
        };
        (fail, $path: expr, $test_name: ident) => {
            make_test! {fail, $test_name, {
                let path = Path::try_from($path).unwrap();
                let expected_filenames = $path.split('/').filter(|&x| x != "").collect::Vec<&str>();
                let filenames = expected_filenames.iter().map(|&f| Filename::try_from(f).unwrap()).collect::<Vec<Filename>>();
                let components = Components::from_path(&path);

                assert_eq!(filenames.iter().count(), components.clone().count());
                assert_eq!(filenames.iter().rev().count(), components.clone().rev().count());
                assert!(filenames.iter().zip(components.clone()).all(|(expect, component)| expect == component));
                // The rev method uses the DoubleEndedIterator implementation.
                assert!(filenames.iter().rev().zip(components.clone().rev()).all(|(expect, component)| expect == component));
            }
            }
        };
    }

    // #[test]
    // fn path_replace_basic() {
    //     let expect = |res: Result<_>| res.expect("Invalid hardcoded test values");

    //     let a = expect(Path::try_from("a/b/c"));
    //     let b = expect(Path::try_from("e/f"));
    //     let expected = [
    //         expect(Path::try_from("e/f/b/c")),
    //         expect(Path::try_from("a/e/f/c")),
    //         expect(Path::try_from("a/b/e/f")),
    //     ];

    //     for offset in 0..a.depth() {
    //         let mut test = a.clone();

    //         test.replace(offset, b.clone());
    //         assert_eq!(test, expected[offset]);
    //     }
    // }

    // #[test]
    // fn components_divide_at() {
    //     let expect = |res: Result<_>| res.expect("Invalid hardcoded test values");

    //     let a = expect(Path::try_from("/a/b/c/d/e/f"));
    //     let expected = [
    //         (
    //             expect(Path::try_from("/")),
    //             expect(Path::try_from("a/b/c/d/e/f")),
    //         ),
    //         (
    //             expect(Path::try_from("/a/")),
    //             expect(Path::try_from("b/c/d/e/f")),
    //         ),
    //         (
    //             expect(Path::try_from("/a/b/")),
    //             expect(Path::try_from("c/d/e/f")),
    //         ),
    //         (
    //             expect(Path::try_from("/a/b/c/")),
    //             expect(Path::try_from("d/e/f")),
    //         ),
    //         (
    //             expect(Path::try_from("/a/b/c/d/")),
    //             expect(Path::try_from("e/f")),
    //         ),
    //         (
    //             expect(Path::try_from("/a/b/c/d/e/")),
    //             expect(Path::try_from("f")),
    //         ),
    //         (
    //             expect(Path::try_from("/a/b/c/d/e/f")),
    //             expect(Path::try_from("")),
    //         ),
    //     ];

    //     for offset in 0..a.depth() {
    //         let test = a.clone();
    //         let (test_1, test_2) = test.components().divide_at(offset);

    //         println!(
    //             "test_1 {:?}\nexpect: {:?}",
    //             test_1,
    //             expected[offset].0.components()
    //         );
    //         println!(
    //             "test_2 {:?}\nexpect: {:?}",
    //             test_2,
    //             expected[offset].1.components()
    //         );

    //         assert!(test_1.eq(expected[offset].0.components()));
    //         assert!(test_2.eq(expected[offset].1.components()));
    //     }
    // }

    use core::convert::TryInto;

    macro_rules! make_path_assert {
        ($a: expr, $b: expr, $name: ident, $assertion: ident) => {
            make_test! {pass, $name, {
                let a: Path = $a.try_into().unwrap();
                let b: Path = $b.try_into().unwrap();

                $assertion!(a, b)
            }}
        };
        (failing, $a: expr, $b: expr, $name: ident, $assertion: ident) => {
            make_test! {failing, $name, {
                let a: Path = $a.try_into().unwrap();
                let b: Path = $b.try_into().unwrap();

                $assertion!(a, b)
            }}
        };
    }

    make_path_assert! {"/", "/", path_eq_root_root, assert_eq}
    make_path_assert! {"a", "a", path_eq_a_b, assert_eq}
    make_path_assert! {"a/b", "a/b", path_eq_a_b_a_b, assert_eq}
    make_path_assert! {"/a/b", "/a/b", path_eq_absolute_a_b_a_b, assert_eq}
    make_path_assert! {"/", "a", path_neq_root_a, assert_ne}
    make_path_assert! {"/", "/a", path_neq_root_root_a, assert_ne}
    make_path_assert! {"/", "/a/b", path_neq_root_root_a_b, assert_ne}
    make_path_assert! {"a/b", "/a/b", path_neq_relative_absolute, assert_ne}

    macro_rules! make_path_ancestors_assert {
        ($path: expr, $ancestors: expr, $name: ident, $assertion: ident) => {
            make_test! {pass, $name, {
                let path: Path = $path.try_into().unwrap();
                let ancestors: &[&str] = $ancestors;

                for (ancestor, &expected) in path.ancestors().zip(ancestors.iter().rev()) {
                    $assertion!(ancestor, Path::try_from(expected).unwrap())
                }
            }}
        };
        (failing, $path: expr, $ancestors: expr, $name: ident, $assertion: ident) => {
            make_test! {failing, $name, {
                let path: Path = $path.try_into().unwrap();
                let ancestors = $ancestors;

                for (ancestor, &expected) in path.ancestors().zip(ancestors.iter().rev()) {
                    $assertion!(ancestor, Path::try_from(expected).unwrap())
                }
            }}
        };
    }

    make_path_ancestors_assert! {"a", &["a"], path_ancestors_basic_a, assert_eq}
    make_path_ancestors_assert! {"/a", &["/a"], path_ancestors_basic_root_a, assert_eq}
    make_path_ancestors_assert! {"a/b", &["a/b",
    "a"], path_ancestors_basic_a_b, assert_eq}
    make_path_ancestors_assert! {"a/b/c/d/e/f/g/h/", &[
        "a/b/c/d/e/f/g/h/",
            "a/b/c/d/e/f/g",
            "a/b/c/d/e/f",
            "a/b/c/d/e",
            "a/b/c/d",
            "a/b/c",
            "a/b",
            "a",
    ], path_ancestors_basic_a_b_c_d_e_f_g_h, assert_eq}

    make_path_ancestors_assert! {"/a/b/c/d/e/f/g/h/", &[
        "/a/b/c/d/e/f/g/h/",
            "/a/b/c/d/e/f/g",
            "/a/b/c/d/e/f",
            "/a/b/c/d/e",
            "/a/b/c/d",
            "/a/b/c",
            "/a/b",
            "/a",
    ], path_ancestors_basic_root_a_b_c_d_e_f_g_h, assert_eq}

    make_path_ancestors_assert! {"a/b/c/d/e/f/g", &[
        "a/b/c/d/e/f/g",
            "a/b/c/d/e/f",
            "a/b/c/d/e",
            "a/b/c/d",
            "a/b/c",
            "a/b",
            "a",
    ], path_ancestors_basic_a_b_c_d_e_f_g, assert_eq}

    make_path_ancestors_assert! {"a/b/c/d/e/f", &[
        "a/b/c/d/e/f",
            "a/b/c/d/e",
            "a/b/c/d",
            "a/b/c",
            "a/b",
            "a",
    ], path_ancestors_basic_a_b_c_d_e_f, assert_eq}

    make_path_ancestors_assert! {"a/b/c/d/e", &[
        "a/b/c/d/e",
            "a/b/c/d",
            "a/b/c",
            "a/b",
            "a",
    ], path_ancestors_a_b_c_d_e, assert_eq}

    make_path_ancestors_assert! {"a/b/c/d", &[
        "a/b/c/d",
            "a/b/c",
            "a/b",
            "a",
    ], path_ancestors_a_b_c_d, assert_eq}

    make_path_ancestors_assert! {"a/b/c", &[
        "a/b/c",
        "a/b",
        "a",
    ], path_ancestors_basic_a_b_c, assert_eq}

    make_test! {
        pass, ancestors_basic, {
            let path: Path = "a/b/c/".try_into().unwrap();
            let mut ancestors = path.ancestors();

            assert_eq!(ancestors.next_back().unwrap(), path);
            // assert_eq!(ancestors.next_back().unwrap(), "a".try_into().unwrap());
            assert_eq!(ancestors.next_back().unwrap(), "a/b".try_into().unwrap());
            assert_eq!(ancestors.next_back().unwrap(), "a".try_into().unwrap());
            assert_eq!(ancestors.next_back(), None);
            assert_eq!(ancestors.next(), None);

            let mut ancestors = path.ancestors();
            assert_eq!(ancestors.next().unwrap(), "a".try_into().unwrap());
            assert_eq!(ancestors.next().unwrap(), "a/b".try_into().unwrap());
            assert_eq!(ancestors.next().unwrap(), path);
            assert_eq!(ancestors.next(), None);
            assert_eq!(ancestors.next_back(), None);
        }
    }
}
