#ifndef __SRINGS_H__
# define __SRINGS_H__

#include <locale.h>
#include <sys/types.h>

//The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided for use with ISO C standard compilers.

//[XSI][Option Start]
int    ffs(int);
//[Option End]
int    strcasecmp(const char *, const char *);
int    strcasecmp_l(const char *, const char *, locale_t);
int    strncasecmp(const char *s1, const char *s2, size_t n);
int    strncasecmp_l(const char *, const char *, size_t, locale_t);

//The <strings.h> header shall define the locale_t type as described in <locale.h>.

//The <strings.h> header shall define the size_t type as described in <sys/types.h>.

#endif
