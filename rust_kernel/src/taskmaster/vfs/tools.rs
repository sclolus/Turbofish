use core::cmp::Ord;
use core::ops::Add;

// use fallible_collections::btree::BTreeMap;

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

    fn gen_filter(&self, _key: K) -> bool {
        true
    }
}

// #[derive(Debug)]
// #[allow(dead_code)]
// pub enum MapperError {
//     EntryAlreadyExists,
//     NoSuchEntry,
//     Nomem,
// }

// #[allow(dead_code)]
// pub type MapperResult<T> = Result<T, MapperError>;

// pub trait Mapper<K, V>: KeyGenerator<K>
// where
//     K: Ord + Add<usize, Output = K> + Default + Copy,
// {
//     fn get_map(&mut self) -> &mut BTreeMap<K, V>;

//     fn add_entry(&mut self, entry: V) -> MapperResult<K> {
//         let key = self.gen();
//         let map = self.get_map();

//         if map.contains_key(&key) {
//             panic!("Mapper: KeyGenerator::gen() returned a contained key");
//         }

//         map.try_insert(key, entry)?;
//         Ok(key)
//     }

//     fn remove_entry(&mut self, key: K) -> MapperResult<V> {
//         let map = self.get_map();
//         if !map.contains_key(&key) {
//             return Err(MapperError::NoSuchEntry);
//         }
//         Ok(map
//             .remove(&key)
//             .expect("Entry is unexpectedly not contained"))
//     }
// }
