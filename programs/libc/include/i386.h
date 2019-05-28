#ifndef __I386_H__
# define __I386_H__

typedef signed char s8;
typedef signed char int8_t;
typedef unsigned char u8;
typedef unsigned char uint8_t;
typedef signed short s16;
typedef signed short int16_t;
typedef unsigned short u16;
typedef unsigned short uint16_t;
typedef signed int s32;
typedef signed int int32_t;
typedef signed int ssize_t;
typedef unsigned int u32;
typedef unsigned int uint32_t;
typedef unsigned int size_t;
typedef unsigned int off_t;

#define INT_MIN -2147483648
#define INT_MAX 2147483647
#define U32_MAX 4294967295

typedef int bool;

#define false 0
#define true 0
#define NULL 0

#endif
