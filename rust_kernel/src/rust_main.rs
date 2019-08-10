use crate::drivers::pit_8253::OperatingMode;
// use crate::drivers::{pic_8259, Acpi, ACPI, PCI, PIC_8259, PIT0};
use crate::drivers::{pic_8259, Acpi, ACPI, PIC_8259, PIT0};

use crate::interrupts;
use crate::keyboard::init_keyboard_driver;
use crate::memory;
use crate::memory::tools::device_map::get_device_map_slice;
use crate::memory::tools::DeviceMap;
use crate::multiboot::MultibootInfo;
use crate::terminal::ansi_escape_code::color::Colored;
use crate::terminal::init_terminal;
use crate::terminal::monitor::Drawer;
use crate::terminal::monitor::SCREEN_MONAD;
use crate::timer::Rtc;
use crate::watch_dog;
use core::time::Duration;

/// Introduction of the trait which will be used as Dynamic
trait Trait: core::fmt::Debug + Send {
    /// Immutable method
    fn wake(&self);
    /// mutable method
    fn sleep(&mut self, _value: u32);
}

/// A raw structure
#[derive(Debug)]
struct A {
    _a: u32,
    _b: u32,
    _c: u32,
}

/// A raw structure
#[derive(Debug)]
struct B {
    _a: u32,
}

/// A raw implementation of the future shared trait
impl Trait for A {
    fn wake(&self) {
        println!("wake for A");
    }
    fn sleep(&mut self, _value: u32) {
        println!("sleep for A");
    }
}

/// A raw implementation of the future shared trait
impl Trait for B {
    fn wake(&self) {
        println!("wake for B");
    }
    fn sleep(&mut self, _value: u32) {
        println!("sleep for B");
    }
}

use alloc::boxed::Box;

/// Simple and basic test with a simple Box<dyn Trait>. The Send trait is just for testing
fn test() {
    println!("Test 1");
    let mut s: Box<dyn Trait + Send> = Box::new(B { _a: 42 });
    s.wake();
    s.sleep(0);

    println!("Test 2");
    test2();
}

/// Introduce a structure wich content a dynamic trait in his fields
struct Content {
    _a: u32,
    _b: u32,
    _data: Box<dyn Trait>,
}

use alloc::vec::Vec;

/// Same test as above but the Box<dyn Trait> is in the embeded structure instead of stack
fn test2() {
    let mut v: Vec<Content> = Vec::new();

    v.push(Content {
        _a: 42,
        _b: 84,
        _data: Box::new(A {
            _a: 42,
            _b: 43,
            _c: 44,
        }),
    });
    v.push(Content {
        _a: 11,
        _b: 22,
        _data: Box::new(B { _a: 42 }),
    });
    v[0]._data.wake();
    v[1]._data.wake();

    println!("Test 3");
    let _i = test3();
}

/// A structure which contains a reference to a dyn trait object
struct Content2<'a> {
    _a: u32,
    _b: u32,
    _data: &'a dyn Trait,
}

/// Test with reference `&Trait`
fn test3() {
    let mut v: Vec<Content2> = Vec::new();

    v.push(Content2 {
        _a: 42,
        _b: 84,
        _data: &A {
            _a: 42,
            _b: 43,
            _c: 44,
        },
    });
    v.push(Content2 {
        _a: 11,
        _b: 22,
        _data: &B { _a: 42 },
    });
    v[0]._data.wake();
    v[1]._data.wake();

    println!("Test 4");
    let _a = test4();
    drop(_a);
    println!("Test 5");
    let _b = test5();
    drop(_b);
    println!("Test 6");
    let _c = test6();
    drop(_c);
    println!("Test 7");
    let _d = test7();
    drop(_d);
    println!("Test 8");
    let _e = test8();
    drop(_e);
}

use alloc::sync::Arc;

/// Introduce another struct which implements Trait
#[derive(Debug)]
struct C {
    a: u32,
    b: u32,
    c: u32,
}

/// Trait implementation
impl Trait for C {
    fn wake(&self) {
        println!("wake for C");
    }
    fn sleep(&mut self, _value: u32) {
        println!("sleep for C");
    }
}

/// Boilerplate for Debug
impl Drop for C {
    fn drop(&mut self) {
        println!("C droped !");
    }
}

/// Test with Arc<dyn Trait> instead of Box<dyn Trait> (Arc has also `DispatchFromDyn` trait, like Box)
fn test4() -> Arc<dyn Trait> {
    let a: Arc<dyn Trait> = Arc::new(C {
        a: 42,
        b: 43,
        c: 44,
    });
    let b = a.clone();

    // If there still at least one reference, the Arc was not droped
    drop(a);
    b.wake();
    b
}

use sync::DeadMutex;

/// Same test as above but we use a mutex to protect the content against race conditions
fn test5() -> Arc<DeadMutex<dyn Trait>> {
    let a: Arc<DeadMutex<dyn Trait>> = Arc::new(DeadMutex::new(C {
        a: 42,
        b: 43,
        c: 44,
    }));
    let b = a.clone();

    // If there still at least one reference, the Arc was not droped
    drop(a);
    b.lock().wake();
    b
}

/// An another struct which implements Trait, with an embeded box
#[derive(Debug)]
struct E {
    a: u32,
    content: Box<u32>,
}

/// Custom implementation
impl E {
    fn new(a: u32, content: Box<u32>) -> Self {
        Self { a, content }
    }
}

/// Boilerplate for Debug
impl Drop for E {
    fn drop(&mut self) {
        println!("E droped !");
    }
}

/// Trait implementation
impl Trait for E {
    fn wake(&self) {
        println!("wake for E: value is {:?}", self.a);
    }
    fn sleep(&mut self, value: u32) {
        println!("sleep for E: changing value...");
        self.a = value;
    }
}

/// Same test as above but with an embeded Box and custom implemented method
fn test6() -> Arc<DeadMutex<dyn Trait>> {
    let a: Arc<DeadMutex<dyn Trait>> = Arc::new(DeadMutex::new(E {
        a: 42,
        content: Box::new(42),
    }));
    let b = a.clone();

    // If there still at least one reference, the Arc was not droped
    drop(a);
    b.lock().wake();

    let c: Arc<DeadMutex<dyn Trait>> = Arc::new(DeadMutex::new(E::new(42, Box::new(42))));
    let d = c.clone();

    d.lock().sleep(666);
    d.lock().wake();
    drop(d);
    c.lock().wake();

    b
}

use alloc::collections::btree_map::BTreeMap;

/// An another struct which implements Trait, with an embeded box
#[derive(Debug)]
struct F {
    a: u32,
    b: u32,
    content: Box<u32>,
}

/// Custom implementation
impl F {
    fn new(a: u32, b: u32, content: Box<u32>) -> Self {
        Self { a, b, content }
    }
}

/// Boilerplate for Debug
impl Drop for F {
    fn drop(&mut self) {
        println!("F droped !");
    }
}

/// Trait implementation
impl Trait for F {
    fn wake(&self) {
        println!("wake for F: value is {:?}", self.b);
    }
    fn sleep(&mut self, value: u32) {
        println!("sleep for F: changing value...");
        self.b = value;
    }
}

///  This test includes BtreeMap which handle all
fn test7() -> BTreeMap<u32, Arc<DeadMutex<dyn Trait>>> {
    let mut z: BTreeMap<u32, Arc<DeadMutex<dyn Trait>>> = BTreeMap::new();

    let a: Arc<DeadMutex<dyn Trait>> = Arc::new(DeadMutex::new(E {
        a: 42,
        content: Box::new(42),
    }));
    let b = a.clone();

    let _r = z.insert(0, a);
    let _r = z.insert(1, b);
    println!("Trashing elements:");

    println!("Trashing first");
    z.remove(&0).unwrap();
    println!("Trashing second");
    z.remove(&1).unwrap();
    println!("Elements trashed");

    let mut z: BTreeMap<u32, Arc<DeadMutex<dyn Trait>>> = BTreeMap::new();

    let a: Arc<DeadMutex<dyn Trait>> = Arc::new(DeadMutex::new(E {
        a: 42,
        content: Box::new(42),
    }));
    let b = a.clone();

    let _r = z.insert(0, a);
    let _r = z.insert(1, b);

    // Accessing methods in Dynamic Trait
    z.get(&0).unwrap().lock().sleep(1234);
    z.get(&1).unwrap().lock().wake();

    let a: Arc<DeadMutex<dyn Trait>> = Arc::new(DeadMutex::new(F::new(42, 84, Box::new(42))));

    let _r = z.insert(2, a);
    // It is strange but a mutable reference is not necessary here (using get() works perfectly)
    z.get_mut(&2).unwrap().lock().sleep(333);
    z.get(&2).unwrap().lock().wake();

    z
}

#[derive(Debug)]
enum TraitType {
    ClassE,
    ClassF,
}

/// Structre Key
#[derive(Debug)]
pub struct Key(usize);

/// Into boilerplate
impl Into<usize> for Key {
    #[inline(always)]
    fn into(self) -> usize {
        self.0
    }
}

/// From boilerplate
impl From<usize> for Key {
    #[inline(always)]
    fn from(addr: usize) -> Self {
        Self(addr)
    }
}

/// Eq boilerplate
impl Eq for Key {}

/// Ord boilerplate
impl Ord for Key {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

/// PartialOrd boilerplate
impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// PartialWq boilerplaye
impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// Embeded structure definition
#[derive(Debug)]
struct Value {
    trait_type: TraitType,
    content: Arc<DeadMutex<dyn Trait>>,
}

/// Standard Implementation
impl Value {
    fn new(trait_type: TraitType, content: Arc<DeadMutex<dyn Trait>>) -> Self {
        Self {
            trait_type,
            content,
        }
    }
}

///  This test includes BtreeMap which handle all and a special Key
fn test8() -> BTreeMap<Key, Value> {
    let mut z: BTreeMap<Key, Value> = BTreeMap::new();

    let a: Arc<DeadMutex<dyn Trait>> = Arc::new(DeadMutex::new(E {
        a: 42,
        content: Box::new(42),
    }));
    let b = a.clone();

    let _r = z.insert(Key(0), Value::new(TraitType::ClassE, a));
    let _r = z.insert(Key(1), Value::new(TraitType::ClassE, b));
    println!("Trashing elements:");

    println!("Trashing first");
    z.remove(&Key(0)).unwrap();
    println!("Trashing second");
    z.remove(&Key(1)).unwrap();
    println!("Elements trashed");

    let mut z: BTreeMap<Key, Value> = BTreeMap::new();

    let a: Arc<DeadMutex<dyn Trait>> = Arc::new(DeadMutex::new(E {
        a: 42,
        content: Box::new(42),
    }));
    let b = a.clone();

    let _r = z.insert(Key(0), Value::new(TraitType::ClassE, a));
    let _r = z.insert(Key(1), Value::new(TraitType::ClassE, b));

    // Accessing methods in Dynamic Trait
    z.get(&Key(0)).unwrap().content.lock().sleep(1234);
    z.get(&Key(1)).unwrap().content.lock().wake();

    let a: Arc<DeadMutex<dyn Trait>> = Arc::new(DeadMutex::new(F::new(42, 84, Box::new(42))));

    let _r = z.insert(Key(2), Value::new(TraitType::ClassF, a));

    println!("Super Debug");
    println!("{:?}", z.get(&Key(2)).unwrap());

    // It is strange but a mutable reference is not necessary here (using get() works perfectly)
    z.get_mut(&Key(2)).unwrap().content.lock().sleep(333);
    z.get(&Key(2)).unwrap().content.lock().wake();

    // Some bullshit code
    let a: Arc<DeadMutex<u32>> = Arc::new(DeadMutex::new(42));
    println!("{:?}", a);

    let b: Box<dyn Trait> = Box::new(E {
        a: 42,
        content: Box::new(42),
    });
    println!("{:?}", b);
    z
}

#[no_mangle]
pub extern "C" fn kmain(
    multiboot_info: *const MultibootInfo,
    device_map_ptr: *const DeviceMap,
) -> ! {
    #[cfg(feature = "serial-eprintln")]
    {
        unsafe { crate::terminal::UART_16550.init() };
        eprintln!("you are in serial eprintln mode");
    }
    let multiboot_info: MultibootInfo = unsafe { *multiboot_info };

    unsafe {
        interrupts::init();
        PIC_8259.lock().init();
        PIC_8259.lock().disable_all_irqs();
        init_keyboard_driver();

        watch_dog();
        interrupts::enable();

        let device_map = get_device_map_slice(device_map_ptr);
        memory::init_memory_system(multiboot_info.get_memory_amount_nb_pages(), device_map)
            .expect("init memory system failed");
    }
    SCREEN_MONAD.lock().switch_graphic_mode(0x118).unwrap();
    init_terminal();
    println!("TTY system initialized");

    PIT0.lock().configure(OperatingMode::RateGenerator);
    PIT0.lock().start_at_frequency(1000.).unwrap();
    log::info!("PIT FREQUENCY: {:?} hz", PIT0.lock().get_frequency());

    match Acpi::init() {
        Ok(()) => match ACPI.lock().expect("acpi init failed").enable() {
            Ok(()) => log::info!("ACPI driver initialized"),
            Err(e) => log::error!("Cannot initialize ACPI: {:?}", e),
        },
        Err(e) => log::error!("Cannot initialize ACPI: {:?}", e),
    };

    unsafe {
        PIC_8259
            .lock()
            .enable_irq(pic_8259::Irq::KeyboardController); // enable only the keyboard.
    }
    log::info!(
        "Keyboard has been initialized: IRQ mask: {:X?}",
        PIC_8259.lock().get_masks()
    );

    let size = SCREEN_MONAD.lock().query_window_size();
    printfixed!(
        Pos {
            line: 1,
            column: size.column - 17
        },
        "{}",
        "Turbo Fish v0.3".green()
    );

    // TODO: Find why it crashs in Sclolus Qemu version
    // log::info!("Scanning PCI buses ...");
    // PCI.lock().scan_pci_buses();
    // log::info!("PCI buses has been scanned");

    crate::test_helpers::really_lazy_hello_world(Duration::from_millis(100));

    test();

    let mut rtc = Rtc::new();
    log::info!("RTC system seems to be working perfectly");
    let date = rtc.read_date();
    println!("{}", date);

    log::error!("this is an example of error");

    watch_dog();

    crate::drivers::storage::init(&multiboot_info);

    use crate::taskmaster::{Process, ProcessOrigin, UserProcess};
    // Load some processes into the scheduler
    let user_process_list = unsafe {
        vec![
            UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/init")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/shell")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(
            // UserProcess::new(ProcessOrigin::Elf(
            //     &include_bytes!("userland/CanonicalRead")[..],
            // ))
            // .unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/shell")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Raw(&_dummy_asm_process_code_a, _dummy_asm_process_len_a)).unwrap(),
            // UserProcess::new(ProcessOrigin::Raw(&_dummy_asm_process_code_b, _dummy_asm_process_len_b)).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/richard")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/vincent")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/fork_fucker")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/fork_me_baby")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/fork_fucker")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/stack_overflow")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/sys_stack_overflow")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/mordak")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/mordak")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/mordak")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/fork_bomb")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/WaitChildDieBefore")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/WaitChildDieAfter")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/sleepers")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/sleepers")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/Timer")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/ConnectionlessSimpleTest")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/ConnectionOrientedSimpleTest")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/DummyRead")[..])).unwrap(),
            /*
             * Signal tests
             */
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/SegFault")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/Ud2")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/Csignal")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/SonKillFather")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/RecursiveSignal")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/recursive_signal_no_defer")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/SaRestart")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/NoSaRestart")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/SaRestartMultiple")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/NoSaRestartMultiple")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/Continue")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/SignalSimple")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/SignalSimpleDuo")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/SignalSimpleDuoRecurse")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/SignalSimpleStopContinue")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/SignalStopContinueOverload")[..])).unwrap(),
            // UserProcess::new(ProcessOrigin::Elf(&include_bytes!("userland/Clone")[..])).unwrap(),
        ]
    };
    crate::taskmaster::start(user_process_list);
}
