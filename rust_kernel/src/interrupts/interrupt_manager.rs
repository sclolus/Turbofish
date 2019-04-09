//! This file contains the code related to the interrupt managing system, i.e the InterruptManger

use crate::interrupts::idt::{GateType, InterruptTable};
use crate::utils::{Either, Either::*};
use alloc::boxed::Box;
use alloc::vec::Vec;
use HandlingState::*;

extern "C" {
    fn _isr_generic_handler_0() -> !;
    fn _isr_generic_handler_1() -> !;
    fn _isr_generic_handler_2() -> !;
    fn _isr_generic_handler_3() -> !;
    fn _isr_generic_handler_4() -> !;
    fn _isr_generic_handler_5() -> !;
    fn _isr_generic_handler_6() -> !;
    fn _isr_generic_handler_7() -> !;
    fn _isr_generic_handler_8() -> !;
    fn _isr_generic_handler_9() -> !;
    fn _isr_generic_handler_10() -> !;
    fn _isr_generic_handler_11() -> !;
    fn _isr_generic_handler_12() -> !;
    fn _isr_generic_handler_13() -> !;
    fn _isr_generic_handler_14() -> !;
    fn _isr_generic_handler_15() -> !;
    fn _isr_generic_handler_16() -> !;
    fn _isr_generic_handler_17() -> !;
    fn _isr_generic_handler_18() -> !;
    fn _isr_generic_handler_19() -> !;
    fn _isr_generic_handler_20() -> !;
    fn _isr_generic_handler_21() -> !;
    fn _isr_generic_handler_22() -> !;
    fn _isr_generic_handler_23() -> !;
    fn _isr_generic_handler_24() -> !;
    fn _isr_generic_handler_25() -> !;
    fn _isr_generic_handler_26() -> !;
    fn _isr_generic_handler_27() -> !;
    fn _isr_generic_handler_28() -> !;
    fn _isr_generic_handler_29() -> !;
    fn _isr_generic_handler_30() -> !;
    fn _isr_generic_handler_31() -> !;
    fn _isr_generic_handler_32() -> !;
    fn _isr_generic_handler_33() -> !;
    fn _isr_generic_handler_34() -> !;
    fn _isr_generic_handler_35() -> !;
    fn _isr_generic_handler_36() -> !;
    fn _isr_generic_handler_37() -> !;
    fn _isr_generic_handler_38() -> !;
    fn _isr_generic_handler_39() -> !;
    fn _isr_generic_handler_40() -> !;
    fn _isr_generic_handler_41() -> !;
    fn _isr_generic_handler_42() -> !;
    fn _isr_generic_handler_43() -> !;
    fn _isr_generic_handler_44() -> !;
    fn _isr_generic_handler_45() -> !;
    fn _isr_generic_handler_46() -> !;
    fn _isr_generic_handler_47() -> !;
    fn _isr_generic_handler_48() -> !;
    fn _isr_generic_handler_49() -> !;
    fn _isr_generic_handler_50() -> !;
    fn _isr_generic_handler_51() -> !;
    fn _isr_generic_handler_52() -> !;
    fn _isr_generic_handler_53() -> !;
    fn _isr_generic_handler_54() -> !;
    fn _isr_generic_handler_55() -> !;
    fn _isr_generic_handler_56() -> !;
    fn _isr_generic_handler_57() -> !;
    fn _isr_generic_handler_58() -> !;
    fn _isr_generic_handler_59() -> !;
    fn _isr_generic_handler_60() -> !;
    fn _isr_generic_handler_61() -> !;
    fn _isr_generic_handler_62() -> !;
    fn _isr_generic_handler_63() -> !;
    fn _isr_generic_handler_64() -> !;
    fn _isr_generic_handler_65() -> !;
    fn _isr_generic_handler_66() -> !;
    fn _isr_generic_handler_67() -> !;
    fn _isr_generic_handler_68() -> !;
    fn _isr_generic_handler_69() -> !;
    fn _isr_generic_handler_70() -> !;
    fn _isr_generic_handler_71() -> !;
    fn _isr_generic_handler_72() -> !;
    fn _isr_generic_handler_73() -> !;
    fn _isr_generic_handler_74() -> !;
    fn _isr_generic_handler_75() -> !;
    fn _isr_generic_handler_76() -> !;
    fn _isr_generic_handler_77() -> !;
    fn _isr_generic_handler_78() -> !;
    fn _isr_generic_handler_79() -> !;
    fn _isr_generic_handler_80() -> !;
    fn _isr_generic_handler_81() -> !;
    fn _isr_generic_handler_82() -> !;
    fn _isr_generic_handler_83() -> !;
    fn _isr_generic_handler_84() -> !;
    fn _isr_generic_handler_85() -> !;
    fn _isr_generic_handler_86() -> !;
    fn _isr_generic_handler_87() -> !;
    fn _isr_generic_handler_88() -> !;
    fn _isr_generic_handler_89() -> !;
    fn _isr_generic_handler_90() -> !;
    fn _isr_generic_handler_91() -> !;
    fn _isr_generic_handler_92() -> !;
    fn _isr_generic_handler_93() -> !;
    fn _isr_generic_handler_94() -> !;
    fn _isr_generic_handler_95() -> !;
    fn _isr_generic_handler_96() -> !;
    fn _isr_generic_handler_97() -> !;
    fn _isr_generic_handler_98() -> !;
    fn _isr_generic_handler_99() -> !;
    fn _isr_generic_handler_100() -> !;
    fn _isr_generic_handler_101() -> !;
    fn _isr_generic_handler_102() -> !;
    fn _isr_generic_handler_103() -> !;
    fn _isr_generic_handler_104() -> !;
    fn _isr_generic_handler_105() -> !;
    fn _isr_generic_handler_106() -> !;
    fn _isr_generic_handler_107() -> !;
    fn _isr_generic_handler_108() -> !;
    fn _isr_generic_handler_109() -> !;
    fn _isr_generic_handler_110() -> !;
    fn _isr_generic_handler_111() -> !;
    fn _isr_generic_handler_112() -> !;
    fn _isr_generic_handler_113() -> !;
    fn _isr_generic_handler_114() -> !;
    fn _isr_generic_handler_115() -> !;
    fn _isr_generic_handler_116() -> !;
    fn _isr_generic_handler_117() -> !;
    fn _isr_generic_handler_118() -> !;
    fn _isr_generic_handler_119() -> !;
    fn _isr_generic_handler_120() -> !;
    fn _isr_generic_handler_121() -> !;
    fn _isr_generic_handler_122() -> !;
    fn _isr_generic_handler_123() -> !;
    fn _isr_generic_handler_124() -> !;
    fn _isr_generic_handler_125() -> !;
    fn _isr_generic_handler_126() -> !;
    fn _isr_generic_handler_127() -> !;
    fn _isr_generic_handler_128() -> !;
    fn _isr_generic_handler_129() -> !;
    fn _isr_generic_handler_130() -> !;
    fn _isr_generic_handler_131() -> !;
    fn _isr_generic_handler_132() -> !;
    fn _isr_generic_handler_133() -> !;
    fn _isr_generic_handler_134() -> !;
    fn _isr_generic_handler_135() -> !;
    fn _isr_generic_handler_136() -> !;
    fn _isr_generic_handler_137() -> !;
    fn _isr_generic_handler_138() -> !;
    fn _isr_generic_handler_139() -> !;
    fn _isr_generic_handler_140() -> !;
    fn _isr_generic_handler_141() -> !;
    fn _isr_generic_handler_142() -> !;
    fn _isr_generic_handler_143() -> !;
    fn _isr_generic_handler_144() -> !;
    fn _isr_generic_handler_145() -> !;
    fn _isr_generic_handler_146() -> !;
    fn _isr_generic_handler_147() -> !;
    fn _isr_generic_handler_148() -> !;
    fn _isr_generic_handler_149() -> !;
    fn _isr_generic_handler_150() -> !;
    fn _isr_generic_handler_151() -> !;
    fn _isr_generic_handler_152() -> !;
    fn _isr_generic_handler_153() -> !;
    fn _isr_generic_handler_154() -> !;
    fn _isr_generic_handler_155() -> !;
    fn _isr_generic_handler_156() -> !;
    fn _isr_generic_handler_157() -> !;
    fn _isr_generic_handler_158() -> !;
    fn _isr_generic_handler_159() -> !;
    fn _isr_generic_handler_160() -> !;
    fn _isr_generic_handler_161() -> !;
    fn _isr_generic_handler_162() -> !;
    fn _isr_generic_handler_163() -> !;
    fn _isr_generic_handler_164() -> !;
    fn _isr_generic_handler_165() -> !;
    fn _isr_generic_handler_166() -> !;
    fn _isr_generic_handler_167() -> !;
    fn _isr_generic_handler_168() -> !;
    fn _isr_generic_handler_169() -> !;
    fn _isr_generic_handler_170() -> !;
    fn _isr_generic_handler_171() -> !;
    fn _isr_generic_handler_172() -> !;
    fn _isr_generic_handler_173() -> !;
    fn _isr_generic_handler_174() -> !;
    fn _isr_generic_handler_175() -> !;
    fn _isr_generic_handler_176() -> !;
    fn _isr_generic_handler_177() -> !;
    fn _isr_generic_handler_178() -> !;
    fn _isr_generic_handler_179() -> !;
    fn _isr_generic_handler_180() -> !;
    fn _isr_generic_handler_181() -> !;
    fn _isr_generic_handler_182() -> !;
    fn _isr_generic_handler_183() -> !;
    fn _isr_generic_handler_184() -> !;
    fn _isr_generic_handler_185() -> !;
    fn _isr_generic_handler_186() -> !;
    fn _isr_generic_handler_187() -> !;
    fn _isr_generic_handler_188() -> !;
    fn _isr_generic_handler_189() -> !;
    fn _isr_generic_handler_190() -> !;
    fn _isr_generic_handler_191() -> !;
    fn _isr_generic_handler_192() -> !;
    fn _isr_generic_handler_193() -> !;
    fn _isr_generic_handler_194() -> !;
    fn _isr_generic_handler_195() -> !;
    fn _isr_generic_handler_196() -> !;
    fn _isr_generic_handler_197() -> !;
    fn _isr_generic_handler_198() -> !;
    fn _isr_generic_handler_199() -> !;
    fn _isr_generic_handler_200() -> !;
    fn _isr_generic_handler_201() -> !;
    fn _isr_generic_handler_202() -> !;
    fn _isr_generic_handler_203() -> !;
    fn _isr_generic_handler_204() -> !;
    fn _isr_generic_handler_205() -> !;
    fn _isr_generic_handler_206() -> !;
    fn _isr_generic_handler_207() -> !;
    fn _isr_generic_handler_208() -> !;
    fn _isr_generic_handler_209() -> !;
    fn _isr_generic_handler_210() -> !;
    fn _isr_generic_handler_211() -> !;
    fn _isr_generic_handler_212() -> !;
    fn _isr_generic_handler_213() -> !;
    fn _isr_generic_handler_214() -> !;
    fn _isr_generic_handler_215() -> !;
    fn _isr_generic_handler_216() -> !;
    fn _isr_generic_handler_217() -> !;
    fn _isr_generic_handler_218() -> !;
    fn _isr_generic_handler_219() -> !;
    fn _isr_generic_handler_220() -> !;
    fn _isr_generic_handler_221() -> !;
    fn _isr_generic_handler_222() -> !;
    fn _isr_generic_handler_223() -> !;
    fn _isr_generic_handler_224() -> !;
    fn _isr_generic_handler_225() -> !;
    fn _isr_generic_handler_226() -> !;
    fn _isr_generic_handler_227() -> !;
    fn _isr_generic_handler_228() -> !;
    fn _isr_generic_handler_229() -> !;
    fn _isr_generic_handler_230() -> !;
    fn _isr_generic_handler_231() -> !;
    fn _isr_generic_handler_232() -> !;
    fn _isr_generic_handler_233() -> !;
    fn _isr_generic_handler_234() -> !;
    fn _isr_generic_handler_235() -> !;
    fn _isr_generic_handler_236() -> !;
    fn _isr_generic_handler_237() -> !;
    fn _isr_generic_handler_238() -> !;
    fn _isr_generic_handler_239() -> !;
    fn _isr_generic_handler_240() -> !;
    fn _isr_generic_handler_241() -> !;
    fn _isr_generic_handler_242() -> !;
    fn _isr_generic_handler_243() -> !;
    fn _isr_generic_handler_244() -> !;
    fn _isr_generic_handler_245() -> !;
    fn _isr_generic_handler_246() -> !;
    fn _isr_generic_handler_247() -> !;
    fn _isr_generic_handler_248() -> !;
    fn _isr_generic_handler_249() -> !;
    fn _isr_generic_handler_250() -> !;
    fn _isr_generic_handler_251() -> !;
    fn _isr_generic_handler_252() -> !;
    fn _isr_generic_handler_253() -> !;
    fn _isr_generic_handler_254() -> !;
    fn _isr_generic_handler_255() -> !;
}

const HANDLERS: [unsafe extern "C" fn() -> !; 256] = [
    _isr_generic_handler_0,
    _isr_generic_handler_1,
    _isr_generic_handler_2,
    _isr_generic_handler_3,
    _isr_generic_handler_4,
    _isr_generic_handler_5,
    _isr_generic_handler_6,
    _isr_generic_handler_7,
    _isr_generic_handler_8,
    _isr_generic_handler_9,
    _isr_generic_handler_10,
    _isr_generic_handler_11,
    _isr_generic_handler_12,
    _isr_generic_handler_13,
    _isr_generic_handler_14,
    _isr_generic_handler_15,
    _isr_generic_handler_16,
    _isr_generic_handler_17,
    _isr_generic_handler_18,
    _isr_generic_handler_19,
    _isr_generic_handler_20,
    _isr_generic_handler_21,
    _isr_generic_handler_22,
    _isr_generic_handler_23,
    _isr_generic_handler_24,
    _isr_generic_handler_25,
    _isr_generic_handler_26,
    _isr_generic_handler_27,
    _isr_generic_handler_28,
    _isr_generic_handler_29,
    _isr_generic_handler_30,
    _isr_generic_handler_31,
    _isr_generic_handler_32,
    _isr_generic_handler_33,
    _isr_generic_handler_34,
    _isr_generic_handler_35,
    _isr_generic_handler_36,
    _isr_generic_handler_37,
    _isr_generic_handler_38,
    _isr_generic_handler_39,
    _isr_generic_handler_40,
    _isr_generic_handler_41,
    _isr_generic_handler_42,
    _isr_generic_handler_43,
    _isr_generic_handler_44,
    _isr_generic_handler_45,
    _isr_generic_handler_46,
    _isr_generic_handler_47,
    _isr_generic_handler_48,
    _isr_generic_handler_49,
    _isr_generic_handler_50,
    _isr_generic_handler_51,
    _isr_generic_handler_52,
    _isr_generic_handler_53,
    _isr_generic_handler_54,
    _isr_generic_handler_55,
    _isr_generic_handler_56,
    _isr_generic_handler_57,
    _isr_generic_handler_58,
    _isr_generic_handler_59,
    _isr_generic_handler_60,
    _isr_generic_handler_61,
    _isr_generic_handler_62,
    _isr_generic_handler_63,
    _isr_generic_handler_64,
    _isr_generic_handler_65,
    _isr_generic_handler_66,
    _isr_generic_handler_67,
    _isr_generic_handler_68,
    _isr_generic_handler_69,
    _isr_generic_handler_70,
    _isr_generic_handler_71,
    _isr_generic_handler_72,
    _isr_generic_handler_73,
    _isr_generic_handler_74,
    _isr_generic_handler_75,
    _isr_generic_handler_76,
    _isr_generic_handler_77,
    _isr_generic_handler_78,
    _isr_generic_handler_79,
    _isr_generic_handler_80,
    _isr_generic_handler_81,
    _isr_generic_handler_82,
    _isr_generic_handler_83,
    _isr_generic_handler_84,
    _isr_generic_handler_85,
    _isr_generic_handler_86,
    _isr_generic_handler_87,
    _isr_generic_handler_88,
    _isr_generic_handler_89,
    _isr_generic_handler_90,
    _isr_generic_handler_91,
    _isr_generic_handler_92,
    _isr_generic_handler_93,
    _isr_generic_handler_94,
    _isr_generic_handler_95,
    _isr_generic_handler_96,
    _isr_generic_handler_97,
    _isr_generic_handler_98,
    _isr_generic_handler_99,
    _isr_generic_handler_100,
    _isr_generic_handler_101,
    _isr_generic_handler_102,
    _isr_generic_handler_103,
    _isr_generic_handler_104,
    _isr_generic_handler_105,
    _isr_generic_handler_106,
    _isr_generic_handler_107,
    _isr_generic_handler_108,
    _isr_generic_handler_109,
    _isr_generic_handler_110,
    _isr_generic_handler_111,
    _isr_generic_handler_112,
    _isr_generic_handler_113,
    _isr_generic_handler_114,
    _isr_generic_handler_115,
    _isr_generic_handler_116,
    _isr_generic_handler_117,
    _isr_generic_handler_118,
    _isr_generic_handler_119,
    _isr_generic_handler_120,
    _isr_generic_handler_121,
    _isr_generic_handler_122,
    _isr_generic_handler_123,
    _isr_generic_handler_124,
    _isr_generic_handler_125,
    _isr_generic_handler_126,
    _isr_generic_handler_127,
    _isr_generic_handler_128,
    _isr_generic_handler_129,
    _isr_generic_handler_130,
    _isr_generic_handler_131,
    _isr_generic_handler_132,
    _isr_generic_handler_133,
    _isr_generic_handler_134,
    _isr_generic_handler_135,
    _isr_generic_handler_136,
    _isr_generic_handler_137,
    _isr_generic_handler_138,
    _isr_generic_handler_139,
    _isr_generic_handler_140,
    _isr_generic_handler_141,
    _isr_generic_handler_142,
    _isr_generic_handler_143,
    _isr_generic_handler_144,
    _isr_generic_handler_145,
    _isr_generic_handler_146,
    _isr_generic_handler_147,
    _isr_generic_handler_148,
    _isr_generic_handler_149,
    _isr_generic_handler_150,
    _isr_generic_handler_151,
    _isr_generic_handler_152,
    _isr_generic_handler_153,
    _isr_generic_handler_154,
    _isr_generic_handler_155,
    _isr_generic_handler_156,
    _isr_generic_handler_157,
    _isr_generic_handler_158,
    _isr_generic_handler_159,
    _isr_generic_handler_160,
    _isr_generic_handler_161,
    _isr_generic_handler_162,
    _isr_generic_handler_163,
    _isr_generic_handler_164,
    _isr_generic_handler_165,
    _isr_generic_handler_166,
    _isr_generic_handler_167,
    _isr_generic_handler_168,
    _isr_generic_handler_169,
    _isr_generic_handler_170,
    _isr_generic_handler_171,
    _isr_generic_handler_172,
    _isr_generic_handler_173,
    _isr_generic_handler_174,
    _isr_generic_handler_175,
    _isr_generic_handler_176,
    _isr_generic_handler_177,
    _isr_generic_handler_178,
    _isr_generic_handler_179,
    _isr_generic_handler_180,
    _isr_generic_handler_181,
    _isr_generic_handler_182,
    _isr_generic_handler_183,
    _isr_generic_handler_184,
    _isr_generic_handler_185,
    _isr_generic_handler_186,
    _isr_generic_handler_187,
    _isr_generic_handler_188,
    _isr_generic_handler_189,
    _isr_generic_handler_190,
    _isr_generic_handler_191,
    _isr_generic_handler_192,
    _isr_generic_handler_193,
    _isr_generic_handler_194,
    _isr_generic_handler_195,
    _isr_generic_handler_196,
    _isr_generic_handler_197,
    _isr_generic_handler_198,
    _isr_generic_handler_199,
    _isr_generic_handler_200,
    _isr_generic_handler_201,
    _isr_generic_handler_202,
    _isr_generic_handler_203,
    _isr_generic_handler_204,
    _isr_generic_handler_205,
    _isr_generic_handler_206,
    _isr_generic_handler_207,
    _isr_generic_handler_208,
    _isr_generic_handler_209,
    _isr_generic_handler_210,
    _isr_generic_handler_211,
    _isr_generic_handler_212,
    _isr_generic_handler_213,
    _isr_generic_handler_214,
    _isr_generic_handler_215,
    _isr_generic_handler_216,
    _isr_generic_handler_217,
    _isr_generic_handler_218,
    _isr_generic_handler_219,
    _isr_generic_handler_220,
    _isr_generic_handler_221,
    _isr_generic_handler_222,
    _isr_generic_handler_223,
    _isr_generic_handler_224,
    _isr_generic_handler_225,
    _isr_generic_handler_226,
    _isr_generic_handler_227,
    _isr_generic_handler_228,
    _isr_generic_handler_229,
    _isr_generic_handler_230,
    _isr_generic_handler_231,
    _isr_generic_handler_232,
    _isr_generic_handler_233,
    _isr_generic_handler_234,
    _isr_generic_handler_235,
    _isr_generic_handler_236,
    _isr_generic_handler_237,
    _isr_generic_handler_238,
    _isr_generic_handler_239,
    _isr_generic_handler_240,
    _isr_generic_handler_241,
    _isr_generic_handler_242,
    _isr_generic_handler_243,
    _isr_generic_handler_244,
    _isr_generic_handler_245,
    _isr_generic_handler_246,
    _isr_generic_handler_247,
    _isr_generic_handler_248,
    _isr_generic_handler_249,
    _isr_generic_handler_250,
    _isr_generic_handler_251,
    _isr_generic_handler_252,
    _isr_generic_handler_253,
    _isr_generic_handler_254,
    _isr_generic_handler_255,
];

pub static mut INTERRUPT_MANAGER: Option<Manager> = None;

#[derive(Debug)]
pub enum InterruptManagerError {
    IdtNotInitialized,
}

#[no_mangle]
unsafe extern "C" fn generic_handler(interrupt_number: usize) {
    assert!(INTERRUPT_MANAGER.is_some());
    match INTERRUPT_MANAGER.as_mut().unwrap().dispatch(interrupt_number as u8) {
        Handled => (),
        NotHandled => log::warn!("Interrupt of number {} was not handled", interrupt_number),
    }
}

/// The type of the interrupt manager, which centralises the interrupts.
/// The Manager dispatch the interrupts to the registered InterruptHandler implementors.
/// The Manager implements InterruptManager.
/// The InterruptHandler implementors can also implement InterruptManager,
/// enabling them to further dispatch the interrupt to a list of registered InterruptHandler.
pub struct Manager<'a> {
    // 256 entries in the IDT, put a constant here please.
    handlers: [Option<Box<InterruptHandler>>; 256],
    interrupt_table: InterruptTable<'a>,
}

/// The InterruptManager trait.
/// The Manager implements it.
pub trait InterruptManager {
    fn dispatch(&mut self, _interrupt_number: u8) -> HandlingState {
        NotHandled
    }

    fn register(&mut self, handler: Box<InterruptHandler>, interrupt_number: u8) -> Result<(), ()>;
}

impl<T> InterruptHandler for T
where
    T: InterruptManager,
{
    fn handle(&mut self, interrupt_number: u8) -> HandlingState {
        self.dispatch(interrupt_number)
    }

    fn kind(&mut self) -> Either<&mut dyn InterruptHandler, &mut dyn InterruptManager> {
        Right(self)
    }
}

impl<'a> Manager<'a> {
    pub fn new() -> Result<Self, InterruptManagerError> {
        use core::mem;
        let mut handlers: [Option<Box<InterruptHandler>>; 256] = unsafe { core::mem::uninitialized() };

        for handler in handlers.iter_mut() {
            mem::forget(mem::replace(handler, None));
        }

        let interrupt_table = unsafe { InterruptTable::current_interrupt_table() };

        match interrupt_table {
            Some(table) => Ok(Self { handlers, interrupt_table: table }),
            None => Err(InterruptManagerError::IdtNotInitialized),
        }
    }
}

impl<'a> InterruptManager for Manager<'a> {
    fn dispatch(&mut self, interrupt_number: u8) -> HandlingState {
        match &mut self.handlers[interrupt_number as usize] {
            Some(handler) => handler.handle(interrupt_number),
            None => {
                log::warn!("No handled registered for interrupt number {}", interrupt_number);
                NotHandled
            }
        }
    }

    fn register(&mut self, handler: Box<InterruptHandler>, interrupt_number: u8) -> Result<(), ()> {
        unsafe {
            without_interrupts!({
                match &mut self.handlers[interrupt_number as usize] {
                    Some(registered_handler) => registered_handler
                        .kind()
                        .map_left(|handler| {
                            log::warn!(
                                "Handler {} for interrupt number {} is already registered",
                                handler.name(),
                                interrupt_number
                            );
                            Err(())
                        })
                        .map_right(|interrupt_manager| interrupt_manager.register(handler, interrupt_number))
                        .move_out()?,
                    None => {
                        self.interrupt_table[interrupt_number as usize]
                            .set_storage_segment(false)
                            .set_privilege_level(0) // To be discussed when different context are a thing.
                            .set_selector(1 << 3)
                            .set_gate_type(GateType::InterruptGate32)
                            .set_present(true)
                            .set_handler(HANDLERS[interrupt_number as usize] as *const () as u32);

                        self.handlers[interrupt_number as usize] = Some(handler);
                    }
                }
            });
        }
        Ok(())
    }
}

pub trait InterruptHandler {
    fn name(&self) -> &str {
        fn type_name_of<T: ?Sized>() -> &'static str {
            extern crate core;
            unsafe { core::intrinsics::type_name::<T>() }
        }

        type_name_of::<Self>()
    }

    fn handle(&mut self, _interrupt_number: u8) -> HandlingState {
        NotHandled
    }

    fn kind(&mut self) -> Either<&mut dyn InterruptHandler, &mut dyn InterruptManager>; // {
                                                                                        //     Left(&self)
                                                                                        // }
}

// Please, find a way to make this work.
// impl<T: FnMut(u8) -> HandlingState> InterruptHandler for Box<T> {
//     fn handle(&mut self, interrupt_number: u8) -> HandlingState {
//         self(interrupt_number)
//     }

//     fn kind(&mut self) -> Either<&mut dyn InterruptHandler, &mut dyn InterruptManager> {
//         Left(self)
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlingState {
    Handled,
    NotHandled,
}

pub struct DummyHandler;

impl DummyHandler {
    pub fn new() -> Self {
        Self
    }
}

impl InterruptHandler for DummyHandler {
    fn handle(&mut self, _interrupt_number: u8) -> HandlingState {
        println!("Dummy Handler was dispatched");
        NotHandled
    }

    fn kind(&mut self) -> Either<&mut dyn InterruptHandler, &mut dyn InterruptManager> {
        Left(self)
    }
}

pub struct GenericManager {
    handlers: Vec<Box<InterruptHandler>>,
}

impl GenericManager {
    pub fn new() -> Self {
        Self { handlers: Vec::new() }
    }
}

impl InterruptManager for GenericManager {
    fn dispatch(&mut self, interrupt_number: u8) -> HandlingState {
        for handler in self.handlers.iter_mut() {
            if let Handled = handler.handle(interrupt_number) {
                return Handled;
            }
        }
        NotHandled
    }

    fn register(&mut self, handler: Box<InterruptHandler>, _interrupt_number: u8) -> Result<(), ()> {
        Ok(self.handlers.push(handler))
    }
}

/// So this is the abstraction used for making InterruptHandler from closures.
/// It's unfortunate but the above-tried generic implementation of InterruptHandler for FnMut(u8) -> HandlingState does not work.
pub struct FnHandler {
    callback: Box<FnMut(u8) -> HandlingState>,
}

impl InterruptHandler for FnHandler {
    fn handle(&mut self, interrupt_number: u8) -> HandlingState {
        (self.callback)(interrupt_number)
    }

    fn kind(&mut self) -> Either<&mut dyn InterruptHandler, &mut dyn InterruptManager> {
        Left(self)
    }
}

impl FnHandler {
    pub fn new(callback: Box<FnMut(u8) -> HandlingState>) -> Self {
        FnHandler { callback }
    }
}

// impl From<Box<FnMut(u8) -> HandlingState>> for FnHandler {
//     fn from(callback: Box<FnMut(u8) -> HandlingState>) -> Self {
//         FnHandler { callback }
//     }
// }
