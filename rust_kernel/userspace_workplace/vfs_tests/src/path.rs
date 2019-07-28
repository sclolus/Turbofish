use core::convert::TryFrom;
use core::fmt;
use core::mem;
use core::cmp::Ordering;
use errno::Errno;
use core::slice::Iter;

use super::posix_consts::{NAME_MAX, PATH_MAX};

/// Newtype of filename
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Filename(pub [u8; NAME_MAX], pub usize);

impl TryFrom<&str> for Filename {
    type Error = Errno;
    fn try_from(s: &str) -> Result<Self, Errno> {
        let mut n = [0; NAME_MAX];
        if s.bytes().find(|&b| b == '/' as u8).is_some() {
            return Err(Errno::Einval);
        }
        if s.len() > NAME_MAX || s.len() == 0 {
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

#[derive(Debug, Clone)]
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

    fn set_absolute(&mut self, value: bool) -> Result<&mut Self, Errno> {
        if !self.is_absolute() && value && self.total_length == PATH_MAX - 1 {
            return Err(Errno::Enametoolong)
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

    pub fn parent(&self) -> Path {
        let mut components = self.components();
        components.next_back();

        Self::try_from(components).unwrap() // well for now this should not be happening
    }

    pub fn push(&mut self, component: Filename) -> Result<&mut Self, Errno> { // this is an Option return type actually
        let total_length;
        if self.depth() != 0 {
            total_length = self.total_length + component.len() + 1;
        } else {
            total_length = self.total_length + component.len();
        }

        if total_length > PATH_MAX - 1 {
            return Err(Errno::Enametoolong)
        }
        self.total_length = total_length;
        self.components.push(component);
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

    pub fn components(&self) -> Iter<Filename> {
        self.components.iter()
    }

    pub fn chain(&mut self, other: Path) -> Result<&mut Self, Errno> {
        if self == &Path::null_path() {
            *self = other;
            Ok(self)
        } else {
            for component in other.components() { // implement into iter to prevent useless copies of components
                self.push(*component)?;
            }
            Ok(self)
        }
    }
}

impl<'a> TryFrom<Iter<'a, Filename>> for Path {
    type Error = Errno;
    fn try_from(iter: Iter<Filename>) -> Result<Self, Errno> {
        let mut path = Path::new();

        for filename in iter {
            path.push(*filename)?;
        }
        Ok(path)

    }
}

impl TryFrom<&str> for Path {
    type Error = Errno;
    fn try_from(s: &str) -> Result<Self, Errno> {
        if s.len() > PATH_MAX - 1 {
            return Err(Errno::Enametoolong);
        }
        let is_absolute = s.starts_with('/');
        let components = s.split('/').filter(|&x| x != "");

        let mut path = Path::new();

        path.set_absolute(is_absolute);
        for component in components {
            let filename = Filename::try_from(component)?;
            path.push(filename)?;
        }
        Ok(path)
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
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

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_absolute() {
            write!(f, "/")?;
        }
        let depth = self.depth();
        for (index, component) in self.components().enumerate() {
            write!(f, "{}", component)?;
            if (index + 1 != depth) {
                write!(f, "/")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules!    make_test {
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
        }
    }

    macro_rules! make_filename_creation_test {
        ($body: block, $test_name: ident) => {
            make_test!{pass, $test_name, {
                Filename::try_from($body.as_str()).unwrap();
            }
            }
        };
        (fail, $body: block, $test_name: ident) => {
            make_test!{fail, $test_name, {
                Filename::try_from($body.as_str()).unwrap();
            }
            }
        };
        ($filename: expr, $test_name: ident) => {
            make_test!{pass, $test_name, {
                Filename::try_from($filename).unwrap();
            }
            }
        };
        (fail, $filename: expr, $test_name: ident) => {
            make_test!{fail, $test_name, {
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

    make_test!{pass, test_path_root_path_is_absolute, {
        let path = Path::try_from("/").unwrap();
        assert!(path.is_absolute)
    }}

    make_test!{pass, test_path_root_path_has_zero_depth, {
        let path = Path::try_from("/").unwrap();
        assert!(path.depth() == 0)
    }}

    make_test!{pass, test_path_root_path_has_one_len, {
        let path = Path::try_from("/").unwrap();
        assert!(path.len() == 1)
    }}

    macro_rules! make_path_len_test {
        ($path: expr, $test_name: ident) => {
            make_test!{pass, $test_name, {
                let path_len = $path.len();
                let path = Path::try_from($path).unwrap();

                assert_eq!(path.len(), path_len);
            }
            }
        };
    }

    make_path_len_test!{"", test_path_len_empty_path}
    make_path_len_test!{"a", test_path_len_a_path}
    make_path_len_test!{"/a", test_path_len_root_a_path}
    make_path_len_test!{"a/b", test_path_len_a_b_path}
    make_path_len_test!{"/a/b", test_path_len_root_a_b_path}
    make_path_len_test!{"a/b/c", test_path_len_a_b_c_path}
    make_path_len_test!{"/a/b/c", test_path_len_root_a_b_c_path}
    make_path_len_test!{"a/bb/ccc", test_path_len_a_bb_ccc_path}
    make_path_len_test!{"/a/bb/ccc", test_path_len_root_a_bb_ccc_path}

    macro_rules! make_path_creation_test {
        ($body: block, $test_name: ident) => {
            make_test!{pass, $test_name, {
                Path::try_from($body.as_str()).unwrap();
            }
            }
        };
        (fail, $body: block, $test_name: ident) => {
            make_test!{fail, $test_name, {
                Path::try_from($body.as_str()).unwrap();
            }
            }
        };
        ($path: expr, $test_name: ident) => {
            make_test!{pass, $test_name, {
                Path::try_from($path).unwrap();
            }
            }
        };
        (fail, $path: expr, $test_name: ident) => {
            make_test!{fail, $test_name, {
                Path::try_from($path).unwrap();
            }
            }
        };



    }

    make_path_creation_test! {"////a/b/c", test_path_posix_path_can_have_multiple_beginning_slashes}
    make_path_creation_test! {"a/b/c////", test_path_posix_path_can_have_multiple_trailing_slashes}
    make_path_creation_test! {"/a////b//////////////////c/d//e/f///g//", test_path_posix_path_can_have_multiple_slashes}
    make_path_creation_test! {"", test_path_posix_path_can_have_zero_filenames}
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
                break
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


    make_test!{pass, test_path_parent_method, {
        let mut paths = Vec::new();
        let mut path = Path::new();

        for alpha in 0..5 {
            let mut string = String::new();
            let c = Some(((alpha + 'a' as u8) as char));
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
            path = path.parent();

            assert_eq!(path, test_path);
        }
    }}

    macro_rules! make_path_chain_test { // please rewrite this, this is getting stupid, DRY
        ($make_path_pair: block, $test_name: ident) => {
            make_test!{pass, $test_name, {
                use std::convert::TryInto;
                let (a, b) = $make_path_pair;
                let (mut a, b): (Path, Path) =  (a.as_str().try_into().unwrap(), b.as_str().try_into().unwrap());
                a.chain(b).unwrap();
            }}
        };

        (fail, $make_path_pair: block, $test_name: ident) => {
            make_test!{fail, $test_name, {
                use std::convert::TryInto;
                let (a, b) = $make_path_pair;
                let (mut a, b): (Path, Path) =  (a.as_str().try_into().unwrap(), b.as_str().try_into().unwrap()); //creat somemacro to report test macro bad uses.

                a.chain(b).unwrap();
            }}
        };

        ($make_path_pair: block, $make_test_path: block, $test_name: ident) => {
            make_test!{pass, $test_name, {
                use std::convert::TryInto;
                let (a, b) = $make_path_pair;
                let (mut a, b): (Path, Path) =  (a.as_str().try_into().unwrap(), b.as_str().try_into().unwrap());
                let test_path: Path = ($make_test_path).try_into().unwrap();
                assert_eq!(a.chain(b).unwrap(), &test_path);
            }}
        };

        (fail, $make_path_pair: block, $make_test_path: block, $test_name: ident) => {
            make_test!{fail, $test_name, {
                use std::convert::TryInto;
                let (a, b) = $make_path_pair;
                let (mut a, b): (Path, Path) =  (a.as_str().try_into().unwrap(), b.as_str().try_into().unwrap()); //creat somemacro to report test macro bad uses.

                let test_path: Path = ($make_test_path).try_into().unwrap();
                assert_eq!(a.chain(b).unwrap(), &test_path);
            }}
        };
        ($make_path_pair: expr, $make_test_path: block, $test_name: ident) => {
            make_test!{pass, $test_name, {
                use std::convert::TryInto;
                let (a, b) = $make_path_pair;
                let (mut a, b): (Path, Path) =  (a.try_into().unwrap(), b.try_into().unwrap());
                let test_path: Path = ($make_test_path).try_into().unwrap();
                assert_eq!(a.chain(b).unwrap(), &test_path);
            }}
        };

        (fail, $make_path_pair: expr, $make_test_path: block, $test_name: ident) => {
            make_test!{fail, $test_name, {
                use std::convert::TryInto;
                let (a, b) = $make_path_pair;
                let (mut a, b): (Path, Path) =  (a.try_into().unwrap(), b.try_into().unwrap()); //creat somemacro to report test macro bad uses.

                let test_path: Path = ($make_test_path).try_into().unwrap();
                assert_eq!(a.chain(b).unwrap(), &test_path);
            }}
        };

    }

    make_path_chain_test!{("a/", "b"),
                          {"a/b"},
                          test_path_chain_a_b}
    make_path_chain_test!{("/a/", "b"),
                          {"/a/b"},
                          test_path_chain_root_a_b}

    make_path_chain_test!{("/a/b/", "b/c/d/"),
                              {"/a/b/b/c/d/"},
                          test_path_chain_root_a_b_b_c_d}

    make_path_chain_test!{("/a/b/", "b/c/d/"),
                                  {"/a/b/b/c/d/"},
                          test_path_chain_root_a_b__b_c_d}

    make_path_chain_test!{("", "a"),
                          {"a"},
                          test_path_chain_root_zero_a_is_a}

    make_path_chain_test!{("a", ""),
                          {"a"},
                          test_path_chain_root_a_zero_is_a}

    make_path_chain_test!{("/a", ""),
                          {"/a"},
                          test_path_chain_root_a_zero_is_root_a}

    make_path_chain_test!{("", "/a"),
                          {"/a"},
                          test_path_chain_zero_root_a_is_root_a}

    make_path_chain_test!{("", "a/b/c"),
                          {"a/b/c"},
                          test_path_chain_zero_a_b_c_is_a_b_c}

    make_path_chain_test!{("a/b/c", ""),
                          {"a/b/c"},
                          test_path_chain_a_b_c_zero_is_a_b_c}

    make_path_chain_test!{fail, {
        let a = make_relative_str_path_of_length(PATH_MAX - 1);
        let b = make_relative_str_path_of_length(1);
        (a, b)
    }, test_path_chain_cant_create_bigger_path_than_posix_says}
    make_path_chain_test!{{
        let a = make_relative_str_path_of_length(PATH_MAX - 3);
        let b = make_relative_str_path_of_length(1);
        (a, b)
    }, test_path_chain_can_create_a_path_of_length_path_max_minus_three}

}
