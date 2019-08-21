use super::SysResult;

use libc_binding::Errno;

use hashmap_core::fnv::FnvHashMap as HashMap;

use alloc::string::String;

use crate::Spinlock;
use lazy_static::lazy_static;

/// IPC dependances list
use super::ipc::Driver;
use super::ipc::FileOperation;
use super::ipc::IpcResult;

use alloc::sync::Arc;
use sync::DeadMutex;

// bah la c'est une globale sout LazyStatic parce que c'est tout con ainsi :p
// si l'ownership est a scheduler, ca va etre super super chaud a gerer a cause des niveaux d'encapsulation
lazy_static! {
    pub static ref DUMMY_VFS: Spinlock<DummyVfs> = Spinlock::new(DummyVfs::new());
}

/// Rien n'oblige ici que le vfs soit une HashMap <Filename, Arc<DeadMutex<dyn Driver>>> xD
/// J'ai fait au plus simple pour mon exemple
pub struct DummyVfs {
    root: HashMap<String, Arc<DeadMutex<dyn Driver>>>,
}

impl DummyVfs {
    /// Un new() tres tres dummy
    fn new() -> Self {
        Self {
            root: HashMap::new(),
        }
    }

    /*
     * All the vfs methods
     * ...
     * ..
     */

    /// L'essentiel pour cette fonction. c'est qu'elle active le trait Drop() du driver
    /// Ca me permet de marquer les FileOperation associes au driver comme 'Broken' par exemple
    /// contrainte: Supprimer un fichier doit appelr Drop du driver
    #[allow(dead_code)]
    pub fn remove_file(&mut self, filename: &str) -> SysResult<()> {
        self.root.remove(&String::from(filename));
        Ok(())
    }

    /// La fonction open() du vfs sera appelee par la fonction open() de l'ipc
    /// Elle doit logiquement renvoyer un FileOperation ou une erreur
    /// C'est le driver attache a l'inode qui se gere de retourner le bon FileOperation
    /// Open du driver doit etre appele
    /// constrainte: Prototype, filename en param et Arc<DeadMutex<dyn FileOperation>> en retour
    /// Ce sont les 'Driver' qui auront l'ownership des 'FileOperation'
    pub fn open(
        &mut self,
        filename: &str, /* access_mode: Mode ? */
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        match self.root.get_mut(&String::from(filename)) {
            Some(elem) => elem.lock().open(),
            None => Err(Errno::ENOENT),
        }
    }

    /// Ici j'enregistre un filename associe a son driver (que je provide depuis l'ipc)
    /// constrainte: Prototype, filename et Arc<DeadMutex<dyn Driver>> en param
    /// Je pense pas qu'il soit oblige d'envoyer un Arc<DeadMutes<...>> ici, une simple Box<dyn ...> pourrait faire l'affaire
    /// L'arc ca peut apporter un avantage pour gerer les liens symboliques en interne, mais c'est tout relatif
    /// Je te passe l'ownership complet du 'Driver'
    pub fn new_driver(
        &mut self,
        filename: String,
        driver: Arc<DeadMutex<dyn Driver>>, /* rights: Rights ? */
    ) -> SysResult<()> {
        if self.root.contains_key(&filename) {
            Err(Errno::EEXIST)
        } else {
            self.root.try_reserve(1)?;
            // la fonction driver.set_inode_id() doit etre appele lors de la creation. C'est pour joindre l'inode au cas ou
            // Je ne sais pas encore si ce sera completement indispensable. Il vaut mieux que ce soit un type primitif afin
            // qu'il n'y ait pas de reference croisees (j'ai mis usize dans l'exemple)
            driver.lock().set_inode_id(0x42);
            self.root.insert(filename, driver);
            Ok(())
        }
    }
}
