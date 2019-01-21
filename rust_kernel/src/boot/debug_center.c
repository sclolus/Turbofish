
#include "i386_type.h"
#include "libft.h"

#include "cpuid.h"
#include "vga_text.h"

static struct cpu_flag_string flag_string[CPU_FLAG_NB] = {
	{CPUID_FEAT_EDX_FPU, "fpu"},
	{CPUID_FEAT_EDX_VME, "vme"},
	{CPUID_FEAT_EDX_DE, "de"},
        {CPUID_FEAT_EDX_PSE, "pse"},
	{CPUID_FEAT_EDX_TSC, "tsc"},
	{CPUID_FEAT_EDX_MSR, "msr"},
	{CPUID_FEAT_EDX_PAE, "pae"},
	{CPUID_FEAT_EDX_MCE, "mce"},
	{CPUID_FEAT_EDX_CX8, "cx8"},
	{CPUID_FEAT_EDX_APIC, "apic"},
	{CPUID_FEAT_EDX_SEP, "sep"},
	{CPUID_FEAT_EDX_MTRR, "mtrr"},
	{CPUID_FEAT_EDX_PGE, "pge"},
	{CPUID_FEAT_EDX_MCA, "mca"},
	{CPUID_FEAT_EDX_CMOV, "cmov"},
	{CPUID_FEAT_EDX_PAT, "pat"},
	{CPUID_FEAT_EDX_PSE36, "pse36"},
	{CPUID_FEAT_EDX_PSN, "psn"},
	{CPUID_FEAT_EDX_CLF, "clf"},
	{CPUID_FEAT_EDX_DTES, "dtes"},
	{CPUID_FEAT_EDX_ACPI, "acpi"},
	{CPUID_FEAT_EDX_MMX, "mmx"},
	{CPUID_FEAT_EDX_FXSR, "fxsr"},
	{CPUID_FEAT_EDX_SSE, "sse"},
	{CPUID_FEAT_EDX_SSE2, "sse2"},
	{CPUID_FEAT_EDX_SS, "ss"},
	{CPUID_FEAT_EDX_HTT, "htt"},
	{CPUID_FEAT_EDX_TM1, "tmi"},
	{CPUID_FEAT_EDX_IA64, "ia64"},
	{CPUID_FEAT_EDX_PBE, "pbe"},

	{CPUID_FEAT_ECX_SSE3, "sse3"},
	{CPUID_FEAT_ECX_PCLMUL, "pclmul"},
	{CPUID_FEAT_ECX_DTES64, "dtes64"},
	{CPUID_FEAT_ECX_MONITOR, "monitor"},
	{CPUID_FEAT_ECX_DS_CPL, "ds_cpl"},
	{CPUID_FEAT_ECX_VMX, "vmx"},
	{CPUID_FEAT_ECX_SMX, "smx"},
	{CPUID_FEAT_ECX_EST, "est"},
	{CPUID_FEAT_ECX_TM2,"tm2"},
	{CPUID_FEAT_ECX_SSSE3, "sse3"},
	{CPUID_FEAT_ECX_CID, "cid"},
	{CPUID_FEAT_ECX_FMA, "fma"},
	{CPUID_FEAT_ECX_CX16, "cx16"},
	{CPUID_FEAT_ECX_ETPRD, "etprd"},
	{CPUID_FEAT_ECX_PDCM, "pdcm"},
	{CPUID_FEAT_ECX_PCIDE, "cpide"},
	{CPUID_FEAT_ECX_DCA, "dca"},
	{CPUID_FEAT_ECX_SSE4_1, "sse4_1"},
	{CPUID_FEAT_ECX_SSE4_2, "sse4_2"},
	{CPUID_FEAT_ECX_x2APIC, "x2apic"},
	{CPUID_FEAT_ECX_MOVBE, "movbe"},
	{CPUID_FEAT_ECX_POPCNT, "popcnt"},
	{CPUID_FEAT_ECX_AES, "aes"},
	{CPUID_FEAT_ECX_XSAVE, "xsave"},
	{CPUID_FEAT_ECX_OSXSAVE, "osxsave"},
	{CPUID_FEAT_ECX_AVX, "avx"},
};

static int query_flag(enum cpu_flag cpu_flag, u32 edx, u32 ecx)
{
	u32 flag = (u32)cpu_flag;

	if (flag == 1)
		return edx & flag;
	else if ((flag) == 6)
		return ecx & ((flag) - 5);
	else if (((flag) >> 1) * 2 == (flag))
		return edx & (flag);
	else
		return ecx & ((flag) - 5);
}

int debug_center(void) {
	clear_screen();
	set_text_color(white);
	int cpuid = check_cpuid_feature();
	printk("CPUID FEATURE: %s\n", cpuid == 0 ? "YES" : "NO");
	if (cpuid != 0) {
		printk("Cannot continue Analyse\nSTOP");
		while (1) {}
	}
	printk("Vendor id: %s\n", get_vendor_id());
	printk("Kernel string for b%u\n", 42);
	u32 edx = get_edx_cpu_features();
	u32 ecx = get_ecx_cpu_features();
	printk("EDX cpu_feature: 0x%0.8x\n", edx);
	printk("ECX cpu_feature: 0x%0.8x\n", ecx);
	int i = 0;
	while (i < FIRST_ECX_FLAG) {
		if (flag_string[i].flag & edx)
			printk("%s ", flag_string[i].s);
		i++;
	}
	while (i < CPU_FLAG_NB) {
		if ((flag_string[i].flag - 5) & ecx)
			printk("%s ", flag_string[i].s);
		i++;
	}
	printk("\n");
	printk("fpu: %s\n", query_flag(CPUID_FEAT_EDX_FPU, edx, ecx) ? "yes" : "no");
	printk("mmx: %s\n", query_flag(CPUID_FEAT_EDX_MMX, edx, ecx) ? "yes" : "no");
	printk("sse: %s\n", query_flag(CPUID_FEAT_EDX_SSE, edx, ecx) ? "yes" : "no");
	printk("sse2: %s\n", query_flag(CPUID_FEAT_EDX_SSE2, edx, ecx) ? "yes" : "no");
	printk("sse3: %s\n", query_flag(CPUID_FEAT_ECX_SSE3, edx, ecx) ? "yes" : "no");
	printk("sse4_1: %s\n", query_flag(CPUID_FEAT_ECX_SSE4_1, edx, ecx) ? "yes" : "no");
	printk("sse4_2: %s\n", query_flag(CPUID_FEAT_ECX_SSE4_2, edx, ecx) ? "yes" : "no");
	printk("osxsave: %s\n", query_flag(CPUID_FEAT_ECX_OSXSAVE, edx, ecx) ? "yes" : "no");
	printk("avx: %s\n", query_flag(CPUID_FEAT_ECX_AVX, edx, ecx) ? "yes" : "no");
	while(1) {}
	return 0;
}
