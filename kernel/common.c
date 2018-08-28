
#include "../kernel/common.h"

// Write a byte out to the specified port.
void out8	(u16 port, u8 value) { asm volatile ("outb %1, %0" : : "dN" (port), "a" (value)); }
void out16	(u16 port, u16 value) { asm volatile ("outw %1, %0" : : "dN" (port), "a" (value)); }
void out32	(u16 port, u32 value) { asm volatile ("outd %1, %0" : : "dN" (port), "a" (value)); }

u8 in8(u16 port) {
    u8 ret;
    asm volatile("inb %1, %0" : "=a" (ret) : "dN" (port));
    return ret; }

u16 in16(u16 port) {
    u16 ret;
    asm volatile ("inw %1, %0" : "=a" (ret) : "dN" (port));
    return ret; }

u32 in32(u16 port) {
    u32 ret;
    asm volatile ("ind %1, %0" : "=a" (ret) : "dN" (port));
    return ret; }



// Copy len bytes from src to dest.
void memcpy(u8 *dest, const u8 *src, u32 len)
{
    const u8 *sp = (const u8*) src;
    u8 *dp = (u8 *)dest;
    for(; len != 0; len--) *dp++ = *sp++;
}

// Write len copies of val into dest.
void memset(u8int *dest, u8int val, u32int len)
{
    u8int *temp = (u8int *)dest;
    for ( ; len != 0; len--) *temp++ = val;
}



extern void panic(const char *message, const char *file, u32int line)
{
    // We encountered a massive problem and have to stop.
    asm volatile("cli"); // Disable interrupts.

    monitor_write("PANIC(");
    monitor_write(message);
    monitor_write(") at ");
    monitor_write(file);
    monitor_write(":");
    monitor_write_dec(line);
    monitor_write("\n");
    // Halt by going into an infinite loop.
    for(;;);
}

extern void panic_assert(const char *file, u32int line, const char *desc)
{
    // An assertion failed, and we have to panic.
    asm volatile("cli"); // Disable interrupts.

    monitor_write("ASSERTION-FAILED(");
    monitor_write(desc);
    monitor_write(") at ");
    monitor_write(file);
    monitor_write(":");
    monitor_write_dec(line);
    monitor_write("\n");
    // Halt by going into an infinite loop.
    for(;;);
}