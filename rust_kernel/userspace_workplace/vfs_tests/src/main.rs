#![feature(underscore_const_names)]
use std::cmp::{Eq, PartialEq, Ord, PartialOrd};
use std::collections::BTreeMap;
use std::str::FromStr;

extern crate errno;
#[macro_use]
extern crate const_assert;

mod posix_consts;
mod path;
use path::{Path, Filename};

mod direntry {
    use std::str::FromStr;
    use super::{DcacheResult, DcacheError};
    use crate::path::{Path, Filename};
    use DcacheError::*;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DirectoryEntryId(usize);

    impl DirectoryEntryId {
        pub fn new(id: usize) -> DirectoryEntryId {
            Self(id)
        }
    }

    use std::fmt::{Display, Formatter, Error};
    use std::convert::TryFrom;
    impl Display for DirectoryEntryId {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            Ok(write!(f, "D #{}", self.0)?)
        }
    }

    #[derive(Debug, Clone)]
    pub struct EntryDirectory {
        entries: Vec<DirectoryEntryId>,
    }

    impl EntryDirectory {
        pub fn is_directory_empty(&self) -> bool {
            self.entries.len() == 0
        }

        pub fn entries(&self) -> &Vec<DirectoryEntryId> {
            &self.entries
        }
    }

    impl Default for EntryDirectory {
        fn default() -> Self {
            Self {
                entries: Vec::new(),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum DirectoryEntryInner {
        Regular,
        Directory(EntryDirectory),
        Symlink(Path),
    }

    use DirectoryEntryInner::*;
    macro_rules!    is_variant {
        ($pat: pat = $it: tt) => {
            if let $pat = $it {
                true
            } else {
                false
            }
        };
    }

    impl DirectoryEntryInner {
        pub fn is_directory(&self) -> bool {
            is_variant!(Directory(_) = self)

            // if let Directory(_) = self {
            //     true
            // } else {
            //     false
            // }
        }

        pub fn is_symlink(&self) -> bool {
            is_variant!(Symlink(_) = self)
        }

        pub fn is_regular(&self) -> bool {
            is_variant!(Regular = self)
        }

        pub fn is_directory_empty(&self) -> DcacheResult<bool> {
            // since empty directory as only . and .. entries.
            Ok(self.get_directory()?.is_directory_empty())
        }

        pub fn get_directory(&self) -> DcacheResult<&EntryDirectory> {
            use DirectoryEntryInner::*;
            Ok(match self {
                Directory(ref directory) => directory,
                _ => return Err(NotADirectory),
            })
        }

        pub fn get_directory_mut(&mut self) -> DcacheResult<&mut EntryDirectory> {
            use DirectoryEntryInner::*;
            Ok(match self {
                Directory(ref mut directory) => directory,
                _ => return Err(NotADirectory),
            })
        }

        pub fn get_symbolic_content(&self) -> DcacheResult<&Path> {
            use DirectoryEntryInner::*;
            Ok(match self {
                Symlink(ref path) => path,
                _ => return Err(NotASymlink),
            })
        }
    }

    #[derive(Debug, Clone)]
    pub struct DirectoryEntry {
        pub filename: Filename,
        inner: DirectoryEntryInner,
        pub id: DirectoryEntryId,
        pub parent_id: DirectoryEntryId,
        // _: (), // ensure privacy of the default struct constructor.
    }

    impl DirectoryEntry {
        // ---------- BUILDER PATTERN ------------
        pub fn set_filename(&mut self, filename: Filename) -> &mut Self {
            self.filename = filename;
            self
        }

        pub fn set_id(&mut self, id: DirectoryEntryId) -> &mut Self {
            self.id = id;
            self
        }

        pub fn set_parent_id(&mut self, parent_id: DirectoryEntryId) -> &mut Self {
            self.parent_id = parent_id;
            self
        }

        pub fn set_directory(&mut self) -> &mut Self {
            self.inner = DirectoryEntryInner::Directory(EntryDirectory::default());
            self
        }

        pub fn set_regular(&mut self) -> &mut Self {
            self.inner = DirectoryEntryInner::Regular;
            self
        }

        pub fn set_symlink(&mut self, path: Path) -> &mut Self {
            self.inner = DirectoryEntryInner::Symlink(path);
            self
        }

        pub fn root_entry() -> Self {
            let mut root_entry = DirectoryEntry::default();
            root_entry
                .set_filename(Filename::try_from("root").unwrap())
                .set_id(DirectoryEntryId::new(2))
                .set_directory();

            root_entry
        }
        // ---------- BUILDER PATTERN END ------------

        pub fn is_directory(&self) -> bool {
            self.inner.is_directory()
        }

        pub fn is_symlink(&self) -> bool {
            self.inner.is_symlink()
        }

        pub fn is_regular(&self) -> bool {
            self.inner.is_regular()
        }

        pub fn get_symbolic_content(&self) -> DcacheResult<&Path> {
            self.inner.get_symbolic_content()
        }

        pub fn get_directory(& self) -> DcacheResult<&EntryDirectory> {
            self.inner.get_directory()
        }

        pub fn get_directory_mut(&mut self) -> DcacheResult<&mut EntryDirectory> {
            self.inner.get_directory_mut()
        }

        pub fn is_directory_empty(&self) -> DcacheResult<bool> {
            self.inner.is_directory_empty()
        }

        pub fn add_entry(&mut self, entry: DirectoryEntryId) -> DcacheResult<()> {
            let directory = self.inner.get_directory_mut()?;

            directory.entries.push(entry);
            Ok(())
        }

        pub fn remove_entry(&mut self, entry: DirectoryEntryId) -> DcacheResult<()> {
            let directory = self.inner.get_directory_mut()?;

            let index = match directory.entries.iter().position(|&x| x == entry) {
                Some(index) => index,
                None => return Err(NoSuchEntry),
            };
            directory.entries.swap_remove(index);
            Ok(())
        }
    }

    impl Default for DirectoryEntry {
        fn default() -> Self {
            Self {
                filename: Filename::try_from("DefaultFilenameChangeThisLol").unwrap(), // remove this unwrap somehow
                inner: DirectoryEntryInner::Regular,
                id: DirectoryEntryId::new(0),
                parent_id: DirectoryEntryId::new(0),
            }
        }
    }
}

use direntry::{DirectoryEntry, DirectoryEntryId};
use errno::Errno;

#[derive(Debug, Copy, Clone)]
enum DcacheError {
    FileAlreadyExists,
    NoSuchEntry,
    NotADirectory,
    NotASymlink,
    InvalidEntryIdInDirectory,
    RootDoesNotExists,
    NotEmpty,
    EntryNotConnected,
    NotEnoughArguments,
    Errno(Errno),
}

impl From<Errno> for DcacheError {
    fn from(errno: Errno) -> Self {
        DcacheError::Errno(errno)
    }
}

type DcacheResult<T> = Result<T, DcacheError>;

struct Dcache {
    pub    root_id: DirectoryEntryId,
    pub    d_entries: BTreeMap<DirectoryEntryId, DirectoryEntry>, // remove those pubs
    pub    path_cache: BTreeMap<Path, DirectoryEntryId>,
}

use DcacheError::*;
impl Dcache {
    pub fn new() -> Self {
        let root_entry = DirectoryEntry::root_entry();

        let mut new = Self {
            root_id: root_entry.id,
            d_entries: BTreeMap::new(),
            path_cache: BTreeMap::new(),
        };

        new.add_entry(None, root_entry).expect("Could not add a root to the Dcache");
        new
    }

    pub fn add_entry(&mut self, parent: Option<DirectoryEntryId>, mut entry: DirectoryEntry) -> DcacheResult<()> {
        let id = entry.id;

        entry.parent_id = parent.unwrap_or(DirectoryEntryId::new(2)); //eeeeeh yeah
        if self.d_entries.contains_key(&id) {
            return Err(FileAlreadyExists)
        }
        self.d_entries.insert(id, entry);

        if let Some(parent) = parent {
            let parent = match self.d_entries.get_mut(&parent) {
                None => return Err(NoSuchEntry),
                Some(parent) => parent,
            };

            parent.add_entry(id)?;
        }
        Ok(())
    }

    pub fn remove_entry(&mut self, id: DirectoryEntryId) -> DcacheResult<DirectoryEntry> {
        let parent_id;
        {
            let entry = match self.d_entries.get(&id) {
                None => return Err(NoSuchEntry),
                Some(entry) => entry,
            };

            if entry.is_directory() && !entry.is_directory_empty()? {
                return Err(NotEmpty)
            }
            parent_id = entry.parent_id;
        }
        let parent_dir = self.d_entries.get_mut(&parent_id).ok_or(EntryNotConnected)?;

        parent_dir.remove_entry(id)?;
        Ok(match self.d_entries.remove(&id) {
            None => return Err(NoSuchEntry),
            Some(entry) => entry,
        })
    }

    pub fn dentry_path(&self, id: DirectoryEntryId) -> DcacheResult<Path> {
        let mut current_id = id;
        let mut rev_path = Path::new();
        loop {
            let entry = self.d_entries.get(&current_id).ok_or(NoSuchEntry)?;

            rev_path.push(entry.filename);
            if entry.id == entry.parent_id {
                break;
            }
            current_id = entry.parent_id;
        }
        let mut path = Path::new();

        while let Some(component) = rev_path.pop() {
            path.push(component);
        }
        Ok(path)
    }
    fn _pathname_resolution(&self, mut root: DirectoryEntryId, pathname: Path, recursion_level: usize) -> DcacheResult<DirectoryEntryId> {
        use crate::posix_consts::SYMLOOP_MAX;
        if recursion_level > SYMLOOP_MAX {
            return Err(Errno(Errno::Eloop))
        }

        if pathname.is_absolute() {
            root = self.root_id;
        }

        if !self.d_entries.contains_key(&root) {
            return Err(RootDoesNotExists)
        }

        let mut current_dir_id = root;
        let mut components = pathname.components();
        let mut was_symlink = false;
        let mut current_entry = self.d_entries.get(&current_dir_id).ok_or(NoSuchEntry)?;
        let mut current_next_entry_id = root;
        for component in components.by_ref() {
            let current_dir = current_entry.get_directory()?;

            if component == &"." {
                continue ;
            } else if component == &".." {
                current_dir_id = current_entry.parent_id;
                current_entry = self.d_entries.get(&current_dir_id).ok_or(NoSuchEntry)?;
                continue ;
            }
            let next_entry_id = current_dir.entries().iter()
                .find(|x| {
                    let filename = &self.d_entries.get(x)
                        .expect("Invalid entry id in a directory entry that is a directory").filename;
                    filename == component
                }).ok_or(NoSuchEntry)?;

            current_next_entry_id = *next_entry_id;
            current_entry = self.d_entries.get(next_entry_id).ok_or(NoSuchEntry)?;
            if current_entry.is_symlink() {
                was_symlink = true;
                break ;
            }
            current_dir_id = *next_entry_id;
        }
        if was_symlink {
            // let current_entry = self.d_entries.get(&current_next_entry_id).ok_or(NoSuchEntry)?;
            let mut new_path = current_entry.get_symbolic_content()?.clone();
            new_path.chain(components.try_into()?)?;

            self._pathname_resolution(current_dir_id, new_path, recursion_level + 1)
        } else {
            Ok(self.d_entries.get(&current_dir_id).unwrap().id)
        }

    }

    pub fn pathname_resolution(&self, root: DirectoryEntryId, pathname: Path) -> DcacheResult<DirectoryEntryId> {
        self._pathname_resolution(root, pathname, 0)
    }

    pub fn walk_tree<F: FnMut(&DirectoryEntry) -> DcacheResult<()>>(&self, root: &DirectoryEntry, mut callback: &mut F) -> DcacheResult<()>  {
        if !root.is_directory() {
            return Err(NotADirectory)
        }
        let directory = root.get_directory().unwrap(); // eh

        let mapping_closure = |entry_id| self.d_entries.get(entry_id).expect("Invalid entry_id in directory in dcache");
        for entry in directory.entries().iter().map(mapping_closure) {
            callback(entry)?;
        }

        for entry in directory.entries().iter().map(mapping_closure).filter(|x| x.is_directory()) {
            self.walk_tree(entry, callback)?;
        }
        Ok(())
    }

    pub fn rename_dentry(&mut self, id: DirectoryEntryId, filename: Filename) -> DcacheResult<()> {
        self.d_entries.get_mut(&id).ok_or(NoSuchEntry)?.set_filename(filename);
        Ok(())
    }
}
use core::fmt::{Display, Error, Formatter};

impl Display for Dcache {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let root = self.d_entries.get(&self.root_id).unwrap();
        self.walk_tree(root, &mut |entry: &DirectoryEntry| {writeln!(f, "-{}-", entry.filename ); Ok(())}).unwrap();

        Ok(())
    }
}

static mut CURRENT_ID: usize = 3;
fn get_available_directory_entry_id() -> DirectoryEntryId {
    unsafe {
        let id = CURRENT_ID;
        CURRENT_ID += 1;
        DirectoryEntryId::new(id)
    }
}

use walkdir::WalkDir;
use std::fs::{FileType, DirEntry, read_dir};
use std::path::Path as StdPath;
use std::convert::{TryFrom, TryInto};
fn main() {
    use std::env;
    let mut dcache = Dcache::new();

    let mut args = env::args().skip(1);

    fn construct_tree(dcache: &mut Dcache, root: &StdPath, parent_id: DirectoryEntryId) {
        let mut iter = read_dir(root).unwrap().filter_map(|e| e.ok());

        for entry in iter {
            let mut new = DirectoryEntry::default();

            new.set_filename(Filename::try_from(entry.file_name().to_str().unwrap()).unwrap());
            new.set_id(get_available_directory_entry_id());
            let filetype = entry.file_type().unwrap();

            if filetype.is_dir() {
                new.set_directory();
            } else if filetype.is_symlink() {
                let std_path = std::fs::read_link(entry.path()).unwrap();
                let path = std_path.as_os_str().to_str().unwrap().try_into().unwrap();
                new.set_symlink(path);
            } else {
                new.set_regular();
            }

            let new_id = new.id;
            dcache.add_entry(Some(parent_id), new).unwrap();

            if entry.file_type().unwrap().is_dir() {
                construct_tree(dcache, &entry.path(), new_id);
            }
        }
    }

    let path = args.next().unwrap();

    construct_tree(&mut dcache, &StdPath::new(&path), DirectoryEntryId::new(2));
    println!("{}", dcache);

    let mut line = String::new();
    let mut stdin = stdin();
    use std::io::stdin;

    let mut callbacks: Vec<Box<ReplClosures>> = Vec::new();

    let ls_closure = |dc: &mut Dcache, cwd: &mut DirectoryEntryId, args: Vec<&str>| -> DcacheResult<()> {
        let arg = args.get(0);
        let path;
        let entry;
        let entry_id;

        match arg {
            Some(&arg) => {
                path = Path::try_from(arg)?;
                entry_id = dc.pathname_resolution(*cwd, path)?;
                entry = dc.d_entries.get(&entry_id).ok_or(NoSuchEntry)?;

            },
            None => {
                entry_id = *cwd;
                entry = dc.d_entries.get(cwd).ok_or(NoSuchEntry)?;
            }
        }


        if entry.is_directory() {
            let directory = entry.get_directory()?;

            println!("(DIRECTORY {}):", entry.filename);
            for entry_id in directory.entries() {
                let entry = dc.d_entries.get(entry_id).ok_or(NoSuchEntry)?;

                let postfix: Option<String>;
                let prefix;
                if entry.is_directory() {
                    postfix = None;
                    prefix = "d---------";
                } else if entry.is_symlink() {
                    postfix = Some(format!("-> {}", entry.get_symbolic_content()?));
                    prefix = "l---------";
                } else {
                    postfix = None;
                    prefix = "----------";
                }
                println!("+={} {} {}", prefix, entry.filename, &postfix.unwrap_or("".to_string()));
            }
        } else {
            println!("-> {}", dc.dentry_path(entry_id)?);
        }
        Ok(())
    };
    let cd_closure = |dcache: &mut Dcache, cwd: &mut DirectoryEntryId, args: Vec<&str>| -> DcacheResult<()> {
        let path = *args.get(0).ok_or(NotEnoughArguments)?;
        let path = Path::try_from(path)?;
        let search_root;
        search_root = *cwd;

        let entry_id = dcache.pathname_resolution(search_root, path)?;
        let entry = dcache.d_entries.get(&entry_id).ok_or(NoSuchEntry)?;
        if entry.is_directory() {
            *cwd = entry_id;
        } else {
            return Err(NotADirectory)
        }
        Ok(())
    };
    let unlink_closure = |dc: &mut Dcache, cwd: &mut DirectoryEntryId, args: Vec<&str>| -> DcacheResult<()> {
        let path = *args.get(0).ok_or(NotEnoughArguments)?;
        let path = Path::try_from(path)?;

        let search_root;
        search_root = *cwd;

        let entry_id = dc.pathname_resolution(search_root, path)?;
        if entry_id == *cwd {
            *cwd = dc.d_entries.get(&entry_id).ok_or(EntryNotConnected)?.parent_id;
        }
        dc.remove_entry(entry_id)?;
        Ok(())
    };

    let rename_closure = |dc: &mut Dcache, cwd: &mut DirectoryEntryId, args: Vec<&str>| -> DcacheResult<()> {
        let path = *args.get(0).ok_or(NotEnoughArguments)?;
        let new_file_name = args.get(1).ok_or(NotEnoughArguments).map(|x| *x)?.try_into()?;
        let path = Path::try_from(path)?;

        let search_root;
            search_root = *cwd;

        let entry_id = dc.pathname_resolution(search_root, path)?;
        dc.rename_dentry(entry_id, new_file_name)?;
        Ok(())
    };

    let symlink_closure = |dc: &mut Dcache, cwd: &mut DirectoryEntryId, args: Vec<&str>| -> DcacheResult<()> {
        let path = *args.get(0).ok_or(NotEnoughArguments)?;
        let new_symlink_pathname = args.get(1).ok_or(NotEnoughArguments)?;
        let path = Path::try_from(path)?;
        let new_symlink_path = Path::try_from(*new_symlink_pathname)?;

        let search_root;
            search_root = *cwd;

        let parent_path = new_symlink_path.parent();
        let filename = new_symlink_path.filename().unwrap(); //remove this unwrap
        let parent_id = dc.pathname_resolution(search_root, parent_path)?;
        let mut new_symlink_entry = DirectoryEntry::default();

        println!("Created symlink {} with path: {}", new_symlink_path, path);

        new_symlink_entry
            .set_filename(*filename)
            .set_id(get_available_directory_entry_id())
            .set_symlink(path);

        dc.add_entry(Some(parent_id), new_symlink_entry)?;
        Ok(())
    };

    let no_such_command_closure = |dcache: &mut Dcache, cwd: &mut DirectoryEntryId, args: Vec<&str>| -> DcacheResult<()> {
        println!("No such command");
        Ok(())
    };

    let print_prompt_closure = |dcache: &Dcache, cwd: &DirectoryEntryId| {
        let entry = dcache.d_entries.get(cwd).unwrap();
        print!("{}> ", entry.filename);
        use std::io::{stdout, Write};

        stdout().flush()
    };

    let callbacks_strings = ["ls", "cd", "unlink", "rename", "symlink", ""];
    type ReplClosures = dyn Fn(&mut Dcache, &mut DirectoryEntryId, Vec<&str>) -> DcacheResult<()>;
    callbacks.push(Box::new(ls_closure));
    callbacks.push(Box::new(cd_closure));
    callbacks.push(Box::new(unlink_closure));
    callbacks.push(Box::new(rename_closure));
    callbacks.push(Box::new(symlink_closure));
    callbacks.push(Box::new(no_such_command_closure));
    let mut cwd_id = dcache.root_id;

    loop {
        line.clear();
        print_prompt_closure(&dcache, &cwd_id);
        match stdin.read_line(&mut line) {
            Ok(_) => {
                println!("-> {}", line);
            },
            Err(e) => {
                println!("(ERROR) -> {}", e);
            }
        }
        let fields = line.split_ascii_whitespace().collect::<Vec<&str>>();
        if fields.len() == 0 {
            continue
        }

        let callback = callbacks_strings.iter().zip(callbacks.iter()).find(|(&x, _)| x == fields[0] || x == "")
            .map(|(_, callback)| callback).unwrap();

        if let Err(e) = (callback)(&mut dcache, &mut cwd_id, fields[1..].to_vec()) {
            println!("Error(e) => {:?}", e);
        }
    }
}
