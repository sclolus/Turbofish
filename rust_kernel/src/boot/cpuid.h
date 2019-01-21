#ifndef __CPUID_H__
# define __CPUID_H__

# include "i386_type.h"

extern u32 check_cpuid_feature(void);
extern char *get_vendor_id(void);

extern u32 get_edx_cpu_features(void);
extern u32 get_ecx_cpu_features(void);

# define CPU_FLAG_NB 56
# define FIRST_ECX_FLAG 30

enum cpu_flag {
    CPUID_FEAT_EDX_FPU          = 1 << 0,
    CPUID_FEAT_EDX_VME          = 1 << 1,
    CPUID_FEAT_EDX_DE           = 1 << 2,
    CPUID_FEAT_EDX_PSE          = 1 << 3,
    CPUID_FEAT_EDX_TSC          = 1 << 4,
    CPUID_FEAT_EDX_MSR          = 1 << 5,
    CPUID_FEAT_EDX_PAE          = 1 << 6,
    CPUID_FEAT_EDX_MCE          = 1 << 7,
    CPUID_FEAT_EDX_CX8          = 1 << 8,
    CPUID_FEAT_EDX_APIC         = 1 << 9,
    CPUID_FEAT_EDX_SEP          = 1 << 11,
    CPUID_FEAT_EDX_MTRR         = 1 << 12,
    CPUID_FEAT_EDX_PGE          = 1 << 13,
    CPUID_FEAT_EDX_MCA          = 1 << 14,
    CPUID_FEAT_EDX_CMOV         = 1 << 15,
    CPUID_FEAT_EDX_PAT          = 1 << 16,
    CPUID_FEAT_EDX_PSE36        = 1 << 17,
    CPUID_FEAT_EDX_PSN          = 1 << 18,
    CPUID_FEAT_EDX_CLF          = 1 << 19,
    CPUID_FEAT_EDX_DTES         = 1 << 21,
    CPUID_FEAT_EDX_ACPI         = 1 << 22,
    CPUID_FEAT_EDX_MMX          = 1 << 23,
    CPUID_FEAT_EDX_FXSR         = 1 << 24,
    CPUID_FEAT_EDX_SSE          = 1 << 25,
    CPUID_FEAT_EDX_SSE2         = 1 << 26,
    CPUID_FEAT_EDX_SS           = 1 << 27,
    CPUID_FEAT_EDX_HTT          = 1 << 28,
    CPUID_FEAT_EDX_TM1          = 1 << 29,
    CPUID_FEAT_EDX_IA64         = 1 << 30,
    CPUID_FEAT_EDX_PBE          = 1 << 31,
    // ECX FLAGS ARE AUGMENTED BY 5 FOR MATHEMATICAL REASONS
    // (ecx group) 1, 2, 4, 8, 16, 32 VS (edx group) 6, 7, 9, 13, 21, 37
    CPUID_FEAT_ECX_SSE3         = (1 << 0) + 5,
    CPUID_FEAT_ECX_PCLMUL       = (1 << 1) + 5,
    CPUID_FEAT_ECX_DTES64       = (1 << 2) + 5,
    CPUID_FEAT_ECX_MONITOR      = (1 << 3) + 5,
    CPUID_FEAT_ECX_DS_CPL       = (1 << 4) + 5,
    CPUID_FEAT_ECX_VMX          = (1 << 5) + 5,
    CPUID_FEAT_ECX_SMX          = (1 << 6) + 5,
    CPUID_FEAT_ECX_EST          = (1 << 7) + 5,
    CPUID_FEAT_ECX_TM2          = (1 << 8) + 5,
    CPUID_FEAT_ECX_SSSE3        = (1 << 9) + 5,
    CPUID_FEAT_ECX_CID          = (1 << 10) + 5,
    CPUID_FEAT_ECX_FMA          = (1 << 12) + 5,
    CPUID_FEAT_ECX_CX16         = (1 << 13) + 5,
    CPUID_FEAT_ECX_ETPRD        = (1 << 14) + 5,
    CPUID_FEAT_ECX_PDCM         = (1 << 15) + 5,
    CPUID_FEAT_ECX_PCIDE        = (1 << 17) + 5,
    CPUID_FEAT_ECX_DCA          = (1 << 18) + 5,
    CPUID_FEAT_ECX_SSE4_1       = (1 << 19) + 5,
    CPUID_FEAT_ECX_SSE4_2       = (1 << 20) + 5,
    CPUID_FEAT_ECX_x2APIC       = (1 << 21) + 5,
    CPUID_FEAT_ECX_MOVBE        = (1 << 22) + 5,
    CPUID_FEAT_ECX_POPCNT       = (1 << 23) + 5,
    CPUID_FEAT_ECX_AES          = (1 << 25) + 5,
    CPUID_FEAT_ECX_XSAVE        = (1 << 26) + 5,
    CPUID_FEAT_ECX_OSXSAVE      = (1 << 27) + 5,
    CPUID_FEAT_ECX_AVX          = (1 << 28) + 5,
};

struct cpu_flag_string{
	enum cpu_flag flag;
	char *s;
};
#endif
