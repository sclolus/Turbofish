/// This module contains the implementation of the CPUID interface for the x86 CPUID instruction
/// For relevant documentation see: https://en.wikipedia.org/wiki/CPUID https://c9x.me/x86/html/file_module_x86_id_45.html
use bit_field::BitField;
use core::arch::x86::{CpuidResult, __cpuid_count};

#[inline(always)]
pub fn cpuid(leaf: u32, sub_leaf: u32) -> CpuidResult {
    let res = unsafe { __cpuid_count(leaf, sub_leaf) };

    res
}

pub fn get_cpuid_feature(feature: CpuidFeatures) -> CpuidRes {
    use CpuidFeatures::*;
    match feature {
        HighestCpuidParameter => {
            let res = cpuid(0, 0);

            CpuidRes::HighestCpuidParameter(res.eax)
        }
        ManufacturerId => {
            use core::mem::transmute;
            let res = cpuid(0, 0);
            let mut id = [0u8; 3 * 4];

            id[0..4].copy_from_slice(&unsafe { transmute::<u32, [u8; 4]>(res.ebx) });
            id[4..8].copy_from_slice(&unsafe { transmute::<u32, [u8; 4]>(res.edx) });
            id[8..12].copy_from_slice(&unsafe { transmute::<u32, [u8; 4]>(res.ecx) });
            CpuidRes::ManufacturerId(ManufacturerIdStr(id))
        }
        ProcessorInfo => {
            let res = cpuid(1, 0);

            CpuidRes::ProcInfoAndFeatures { proc_info: res.eax.into(), additional_info: res.ebx.into() }
        }
        /* _ => unimplemented!(), */
    }
}

pub enum CpuidFeatures {
    HighestCpuidParameter,
    ManufacturerId,
    ProcessorInfo,
}

#[derive(Debug)]
pub enum CpuidRes {
    HighestCpuidParameter(u32),
    ManufacturerId(ManufacturerIdStr),
    ProcInfoAndFeatures { proc_info: ProcessorInfo, additional_info: ProcessorAdditionalInfo },
}

#[derive(Debug)]
pub struct ManufacturerIdStr([u8; 3 * 4]);

use core::convert::AsRef;

impl AsRef<str> for ManufacturerIdStr {
    fn as_ref(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.0) }
    }
}

#[derive(Debug)]
pub struct ProcessorInfo {
    extended_family_id: u8,
    extended_model_id: u8,
    processor_type: ProcessorType,
    family_id: u8,
    model_id: u8,
    stepping_id: u8,
}

#[derive(Debug)]
pub enum ProcessorType {
    /// Generic type of processor.
    OriginalEOM,

    /// The Pentium Overdrive.
    IntelOverdrive,
    /// This not applicable to Intel486 processors.
    DualProcessor,

    ReservedValue,
}

impl core::convert::From<u8> for ProcessorType {
    fn from(value: u8) -> Self {
        use ProcessorType::*;
        match value {
            0 => OriginalEOM,
            1 => IntelOverdrive,
            2 => DualProcessor,
            // Since from can't fail and that would be an hursle to use try_from. let's to this.
            _ => ReservedValue,
        }
    }
}

impl core::convert::From<u32> for ProcessorInfo {
    fn from(eax: u32) -> Self {
        Self {
            extended_family_id: eax.get_bits(20..28) as u8,
            extended_model_id: eax.get_bits(16..20) as u8,
            processor_type: (eax.get_bits(12..13) as u8).into(),
            family_id: eax.get_bits(8..12) as u8,
            model_id: eax.get_bits(4..8) as u8,
            stepping_id: eax.get_bits(0..4) as u8,
        }
    }
}

/// Those are the additional informations about the CPU retrieved by cpuid with input value 1.
#[derive(Debug)]
pub struct ProcessorAdditionalInfo {
    /// This is EBX[0..8], The brand index.
    brand_index: u8,

    /// This is EBX[8..16], the line size of CLFLUSH. This information is valid if the cpu has the CLFLUSH feature.
    /// This can be checked by calling cpuid::has_feature(Feature::Clfsh).
    clflush_line_size: u8,

    /// This is EBX[16..24], the Maximum number of addressable IDs for logical processors in this physical package.
    /// The nearest power-of-2 integer that is not smaller than this value is the number of unique initial APIC IDs reserved for addressing different logical processors in a physical package.
    /// This information is valid if the cpu has the Hyper-threading feature.
    /// This can be checked by calling cpuid::has_feature(Feature::Htt).
    max_id: u8,

    /// This is EBX[24..32], the Local APIC ID: The initial APIC-ID is used to identify the executing logical processor.
    /// This information is valid if the current processor is an Pentium 4 or any subsequent processor.
    local_apic_id: u8,
}

impl core::convert::From<u32> for ProcessorAdditionalInfo {
    fn from(ebx: u32) -> Self {
        Self {
            brand_index: ebx.get_bits(0..8) as u8,
            clflush_line_size: ebx.get_bits(8..16) as u8,
            max_id: ebx.get_bits(16..24) as u8,
            local_apic_id: ebx.get_bits(24..32) as u8,
        }
    }
}

/// The variants of this enum are the features described in the EDX and ECX register when cpuid is called with an input value of 1.
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Feature {
    // change this name.
    /// Onboard x87 FPU.
    Fpu,

    /// Virtual 8086 mode extensions.
    Vme,

    /// Debugging extensions.
    De,

    /// Page Size Extension.
    Pse,

    /// Time Stamp Counter.
    Tsc,

    /// Model-specific registers.
    Msr,

    /// Physical Address Extension.
    Pae,

    /// Machine Check Exception.
    Mce,

    /// CMPXCHG8 (compare-and-swap) instruction.
    Cx8,

    /// Onboard Advanced Programmable Interrupt Controller (apic).
    Apic,

    /// SYSENTER and SYSEXIT instructions.
    Sep = 11,

    /// Memory Type Range Registers.
    Mtrr,

    /// Page Global Enable bit in CR4.
    Pge,

    /// Machine check architecture.
    Mca,

    /// Conditional move and FCMOV instructions.
    Cmov,

    /// Page Attribute Table.
    Pat,

    /// 36-bit page size extension.
    Pse36,

    /// Processor Serial Number.
    Psn,

    /// CLFLUSH instruction (SSE2)
    Clfsh,

    /// Debug store: save trace of executed jumps.
    Ds = 21,

    /// Onboard thermal control MSRs for ACPI.
    Acpi,

    /// MMX instructions.
    Mmx,

    /// FXSAVE, FXRESTOR instructions CR4 bit 9.
    Fxsr,

    /// SSE instructions.
    Sse,

    /// SSE2 instructions.
    Sse2,

    /// CPU cache implements self-snoop
    Ss,

    /// Hyper-threading.
    Htt,

    /// Thermal monitor automatically limits temperature.
    Tm,

    /// IA64 processor emulating x86
    Ia64,

    /// Pending Break Enable wakeup capability.
    Pbe,

    /// SSE3 instructions.
    Sse3,

    /// PCLMULQDQ instruction.
    Pclmulqdq,

    /// 64-bit debug store (edx bit 21).
    Dtes64,

    /// Monitor and MWAIT instructions (SSE3)
    Monitor,

    /// CPL qualified debug store.
    DsCpl,

    /// Virtual Machine Extensions.
    Vmx,

    /// Safer Mode extensions (LaGrande).
    Smx,

    /// Enhanced SpeedStep.
    Est,

    /// Thermal Monitor 2.
    Tm2,

    /// Supplemental SSE3 instructions.
    Ssse3,

    /// L1 Context ID.
    CnxtId,

    /// Silicon Debug interface.
    Sdbg,

    /// Fused multiply-add (FMA3)
    Fma,

    /// CMPXCHG16B instruction.
    Cx16,

    /// Can disable sending task priority messages.
    Xtpr,

    /// Perfmon and debug capability.
    Pdcm,

    /// Process Context Identifiers (CR4 bit 17).
    Pcid = 49,

    /// Direct cache access for DMA writes.
    Dca,

    /// SSE4.1 instructions.
    Sse41,

    /// SSE4.2 instructions.
    Sse42,

    /// x2APIC.
    X2APIC,

    /// MOVBE instruction (bid-endian).
    Movbe,

    /// POPCNT instruction.
    Popcnt,

    /// APIC implements one-shot operation using TSC deadline value.
    TscDeadline,

    /// AES instruction set.
    Aes,

    /// XSAVE, XRESTOR, XSETBV, XGETBV instructions.
    Xsave,

    /// XSAVE enabled by OS.
    OsXsave,

    /// Advanced Vector Extensions (AVX).
    Avx,

    /// F16C (half-precision) FP feature.
    F16c,

    /// RDRAND (on-ship random number generator) feature.
    Rdrnd,
}

pub fn has_feature(feature: Feature) -> bool {
    let mut index = feature as usize;
    let res = cpuid(1, 0);

    if index > 31 {
        index -= 32;
        res.ecx.get_bit(index)
    } else {
        res.edx.get_bit(index)
    }
}

/// The variants of this enum are the features described in the registers EBX, ECX and EDX when CPUID is called with input value 7.
#[derive(Debug, Copy, Clone)]
pub enum ExtendedFeature {
    /// Access to base of %fs and %gs.
    FsgsBase,

    /// IA32_TSC_ADJUST.
    TscAdjust,

    /// Software Guard Extensions.
    Sgx,

    /// Bit Manipulation Instruction Set 1.
    Bmi1,

    /// Transactional Synchronization Extensions.
    Hle,

    /// Advanced Vector Extensions 2.
    Avx2,

    /// Supervisor-Mode Execution Prevention.
    Smep = 7,

    /// Bit Manipulation Instruction Set 2.
    Bmi2,

    /// Enhanced REP MOVSB/STOSB instructions.
    Erms,

    /// INVPCID instruction.
    Invpcid,

    /// Transactional Synchronization Extensions (Restricted Transactional Memory).
    Rtm,

    /// Platform Quality of Service Monitoring.
    Pqm,

    /// FPU CS and FPU DS deprecated.
    FpuCsDsDeprecated,

    /// Intel MPX (Memory Protection Extensions).
    Mpx,

    /// Platform Quality Of Service Enforcement.
    Pqe,

    /// AVX-512 Foundation.
    Avx512f,

    /// AVX-512 Doubleword and Quadword instructions.
    Avx512dq,

    /// RDSEED instruction.
    Rdseed,

    /// Intel ADX (Multi-Precision Add-Carry Instruction Extensions).
    Adx,

    /// Supervisor-Mode Access Prevention.
    Smap,

    /// AVX-512 Integer Fused Multiply-Add instructions.
    Avx512ifma,

    /// PCOMMIT instruction.
    Pcommit,

    /// CLFLUSHOPT instruction.
    Clflushopt,

    /// CLWB instruction.
    Clwb,

    /// Intel Processor Trace.
    IntelPt,

    /// AVX-512 Prefetch instructions.
    Avx512pf,

    /// AVX-512 Exponential and Reciprocal instructions.
    Avx512er,

    /// AVX-512 Conflict Detection instructions.
    Avx512cd,

    /// Intel SHA extensions,
    Sha,

    /// AVX-512 Byte and Word instructions.
    Avx512bw,

    /// AVX-512 Vector Length Extensions.
    Avx512vl,

    /// PREFETCHWT1 instruction.
    Prefetchwt1,

    /// AVX-512 Vector Bit Manipulation Instructions.
    Avx512vbmi,

    /// User-mode Instruction Prevention.
    Umip,

    /// Memory Protection Keys for User-mode pages.
    Pku,

    /// PKU enabled by OS.
    OsPke,

    /// AVX-512 Vector Bit Manipulation Instructions 2.
    Avx512vbmi2 = 38,

    /// Galois field instructions.
    Gfni = 40,

    /// Vector AES instruction set (VEX-256/EVEX).
    Vaes,

    /// CLMUL instruction set (VEX-256/EVEX).
    Vpclmulqdq,

    /// AVX-512 Vector Neural Network Instructions.
    Avx512vnni,

    /// AVX-512 BITALG instructions.
    Avx512bitalg,

    /// AVX-512 Vector Population Count Double and Quad-word.
    Avx512vpopcntdq = 46,
    // Let's skip mawau value since it's literally fucking everything up.
    /// Read Processor ID.
    Rdpid = 54,

    /// SGX Launch Configuration.
    SgxLc = 62,

    /// AVX-512 4-register Neural Network instructions.
    Avx5124vnniw = 66,

    /// AVX-512 4-register Multipy Accumulation Single precision.
    Avx5124fmaps,

    /// Platform Configuration (Memory Encryption Technologies instructions).
    Pconfig = 82,

    /// Speculation Control:
    /// Indirect Branch Restricted Speculation (IBRS) and
    /// Indirect Branch Prediction Barrier (IBPB).
    SpecCtrl = 90,

    /// Single Thread Indirect Branch Predictor (STIBP),
    Stibp,

    /// IA32_ARCH_CAPABILITIES.
    IA32ArchCapabilities = 93,

    /// Speculative Store Bypass Disable (SSBD),[16] as mitigation for Speculative Store Bypass.
    Ssbd,
}

pub fn has_extended_feature(feature: ExtendedFeature) -> bool {
    let mut index = feature as usize;
    let res = cpuid(7, 0);

    if index > 63 {
        index -= 64;
        res.edx.get_bit(index)
    } else if index > 31 {
        index -= 32;
        res.ecx.get_bit(index)
    } else {
        res.ebx.get_bit(index)
    }
}
