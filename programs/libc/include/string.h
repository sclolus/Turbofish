
/* string.h: */
#ifndef _STRING_H
#define _STRING_H
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif


//    [CX] [Option Start] Some of the functionality described on this reference page extends the ISO C standard. Applications shall define the appropriate feature test macro (see XSH The Compilation Environment ) to enable the visibility of these symbols in this header. [Option End]

//    The <string.h> header shall define NULL and size_t as described in <stddef.h>.

//    [CX] [Option Start] The <string.h> header shall define the locale_t type as described in <locale.h>. [Option End]
	
#include <locale.h>
//    The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided for use with ISO C standard compilers.

//    [XSI][Option Start]
	void    *memccpy(void *restrict, const void *restrict, int, size_t);
//    [Option End]
	void    *memchr(const void *, int, size_t);
	int      memcmp(const void *, const void *, size_t);
	void    *memcpy(void *restrict, const void *restrict, size_t);
	void    *memmove(void *, const void *, size_t);
	void    *memset(void *, int, size_t);
//    [CX][Option Start]
	char    *stpcpy(char *restrict, const char *restrict);
	char    *stpncpy(char *restrict, const char *restrict, size_t);
//    [Option End]
	char    *strcat(char *restrict, const char *restrict);
	char    *strchr(const char *, int);
	char	*strchrnul(const char *, int);
	int      strcmp(const char *, const char *);
	int      strcoll(const char *, const char *);
//    [CX][Option Start]
	int      strcoll_l(const char *, const char *, locale_t);
//    [Option End]
	char    *strcpy(char *restrict, const char *restrict);
	size_t   strcspn(const char *, const char *);
//    [CX][Option Start]
	char    *strdup(const char *);
//    [Option End]
	char    *strerror(int);
//    [CX][Option Start]
	char    *strerror_l(int, locale_t);
	int      strerror_r(int, char *, size_t);
//    [Option End]
	size_t   strlen(const char *);
	char    *strncat(char *restrict, const char *restrict, size_t);
	int      strncmp(const char *, const char *, size_t);
	char    *strncpy(char *restrict, const char *restrict, size_t);
//    [CX][Option Start]
	char    *strndup(const char *, size_t);
	size_t   strnlen(const char *, size_t);
//    [Option End]
	char    *strpbrk(const char *, const char *);
	char    *strrchr(const char *, int);
//    [CX][Option Start]
	char    *strsignal(int);
//    [Option End]
	size_t   strspn(const char *, const char *);
	char    *strstr(const char *, const char *);
	char    *strtok(char *restrict, const char *restrict);
//    [CX][Option Start]
	char    *strtok_r(char *restrict, const char *restrict, char **restrict);
//    [Option End]
	size_t   strxfrm(char *restrict, const char *restrict, size_t);
//    [CX][Option Start]
	size_t   strxfrm_l(char *restrict, const char *restrict,
					   size_t, locale_t);
//    [Option End]

//    [CX] [Option Start] Inclusion of the <string.h> header may also make visible all symbols from <stddef.h>. [Option End]
#ifdef __cplusplus
}
#endif
#endif
