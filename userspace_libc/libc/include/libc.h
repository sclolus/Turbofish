#ifndef __LIBC_H__
# define __LIBC_H__

#include <string.h>
#include <stdarg.h>

int xprintf(const char *format, ...);
int xdprintf(int const fd, const char *format, ...);
int xsprintf(char *str, const char *format, ...);
int xsnprintf(char *str, size_t size, const char *format, ...);
int xasprintf(char **strp, const char *format, ...);
int xvprintf(const char* format, va_list ap);
int xvdprintf(int fd, const char *format, va_list ap);
int xvsprintf(char *str, const char *format, va_list ap);
int xvsnprintf(char *str, size_t size, const char *format, va_list ap);
int xvasprintf(char **strp, const char *format, va_list ap);
int xeprintf(const char *format, ...);

#endif
