
// common.h -- Defines typedefs and some global functions.
//             Inspiré de JamesM's kernel development tutorials.

#ifndef COMMON_H
#define COMMON_H

// Some nice typedefs, to standardise sizes across platforms.
// These typedefs are written for 32-bit X86.
typedef unsigned char  	u8;
typedef unsigned short 	u16;
typedef unsigned int   	u32;

typedef char  s8;
typedef short s16;
typedef int   s32;

// ASM common routines

void out8	(u16 port, u8 value);
void out16	(u16 port, u16 value);
void out32	(u16 port, u32 value);

u8   in8	(u16 port);
u16  in16	(u16 port);
u32  in32	(u16 port);

// Kernel Panic

#define PANIC(msg) panic(msg, __FILE__, __LINE__);
#define ASSERT(b) ((b) ? (void)0 : panic_assert(__FILE__, __LINE__, #b))

extern void panic(const char *message, const char *file, u32int line);
extern void panic_assert(const char *file, u32int line, const char *desc);

#endif // COMMON_H