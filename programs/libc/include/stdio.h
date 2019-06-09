#ifndef __STDIO_H__
# define __STDIO_H__

#include "i386.h"

int printf(const char *restrict format, ...);
int eprintf(const char *restrict format, ...);
int fprintf(int const fd, const char *restrict format, ...);
int dprintf(bool display, const char *restrict format, ...);
int sprintf(char *str, const char *restrict format, ...);

#endif
