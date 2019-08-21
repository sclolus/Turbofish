use super::SysResult;

use super::ipc::FileOperation;
use super::ipc::Std;

use libc_binding::Errno;

use hashmap_core::fnv::FnvHashMap as HashMap;

use alloc::string::String;
use alloc::sync::Arc;

type FileName = String;

pub struct DummyVfs {
    root: HashMap<String, Arc<dyn Inode>>,
}

trait Inode: core::fmt::Debug + Send {
    fn get_type(&self) -> InodeType;
    fn open(&self) -> Arc<dyn FileOperation>;
}

#[derive(Debug)]
struct Tty {
    std: Arc<Std>,
}

impl Tty {
    fn new(controlling_terminal: usize) -> Self {
        Self {
            std: Arc::new(Std::new(controlling_terminal)),
        }
    }
}

impl Inode for Tty {
    fn get_type(&self) -> InodeType {
        InodeType::Tty
    }
    fn open(&self) -> Arc<dyn FileOperation> {
        self.std.clone()
    }
}

enum InodeType {
    Tty,
}

impl DummyVfs {
    pub fn new() -> Self {
        Self {
            root: HashMap::new(),
        }
    }
    pub fn new_tty(&mut self, index: usize) -> SysResult<()> {
        let name = alloc::fmt::format(format_args!("tty{}", index));

        if self.root.contains_key(&name) {
            Err(Errno::EEXIST)
        } else {
            self.root.try_reserve(1)?;
            self.root.insert(name, Arc::new(Tty::new(index)));
            Ok(())
        }
    }
    pub fn open(&mut self, filename: &str) -> SysResult<Arc<dyn FileOperation>> {
        match self.root.get_mut(&String::from(filename)) {
            Some(elem) => Ok(elem.open()),
            None => Err(Errno::ENOENT),
        }
    }
}
