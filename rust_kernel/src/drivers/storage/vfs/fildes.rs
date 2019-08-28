use core::cmp::Ord;
use core::ops::Add;
pub type Fd = usize;
pub type OFDId = usize;

pub struct Fildes {
    // fd: Fd,
    /// Ofd for OpenFileDescription
    ofd_id: OFDId,
}

impl Fildes {
    pub fn new(ofd_id: OFDId) -> Self {
        Self { ofd_id }
    }
}

use alloc::collections::BTreeMap;

pub trait KeyGenerator<K>
where
    K: Ord + Add<usize, Output = K> + Default + Copy,
{
    fn gen(&mut self) -> K {
        let mut cur = K::default();

        while !self.gen_filter(cur) {
            cur = cur + 1
        }
        cur
    }

    fn gen_filter(&self, key: K) -> bool {
        true
    }
}

#[derive(Debug)]
pub enum MapperError {
    EntryAlreadyExists,
    NoSuchEntry,
}

pub type MapperResult<T> = Result<T, MapperError>;

pub trait Mapper<K, V>: KeyGenerator<K>
where
    K: Ord + Add<usize, Output = K> + Default + Copy,
{
    fn get_map(&mut self) -> &mut BTreeMap<K, V>;

    fn add_entry(&mut self, entry: V) -> MapperResult<K> {
        let key = self.gen();
        let map = self.get_map();

        if map.contains_key(&key) {
            panic!("Mapper: KeyGenerator::gen() returned a contained key");
        }

        map.insert(key, entry);
        Ok(key)
    }

    fn remove_entry(&mut self, key: K) -> MapperResult<V> {
        let map = self.get_map();
        if !map.contains_key(&key) {
            return Err(MapperError::NoSuchEntry);
        }
        Ok(map
            .remove(&key)
            .expect("Entry is unexpectedly not contained"))
    }
}
