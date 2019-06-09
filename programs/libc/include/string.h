#ifndef __STRING_H__
# define __STRING_H__

#include "i386.h"

#define HEX_T(x)	"0123456789ABCDEF"[x]

void *ft_memset(void *b, int c, size_t len);
void *ft_memcpy(void *restrict dst, const void *restrict src, size_t n);
void *ft_memmove(void *dst, const void *src, size_t len);
int ft_memcmp(const void *s1, const void *s2, size_t n);

void bzero(void *s, size_t n);
void *memccpy(void *restrict dest, const void *restrict src, int c, size_t n);
void *memchr(const void *s, int c, size_t n);

size_t strlen(const char *s);
char *strcpy(char *dst, const char *src);
char *strncpy(char *dst, const char *src, size_t len);
char *strcat(char *restrict s1, const char *restrict s2);
char *strncat(char *restrict s1, const char *restrict s2, size_t n);
size_t strlcat(char *restrict dst, const char *restrict src, size_t size);
char *strchr(const char *s, int c);
char *strrchr(const char *s, int c);
char *strstr(const char *big, const char *little);
char *strnstr(const char *big, const char *little, size_t len);
int strcmp(const char *s1, const char *s2);
int strncmp(const char *s1, const char *s2, size_t n);
int atoi(const char *str);

int isalpha(int c);
int isdigit(int c);
int isalnum(int c);
int isascii(int c);
int isprint(int c);
int toupper(int c);
int tolower(int c);

void strclr(char *s);
void striter(char *s, void (*f)(char *));
void striteri(char *s, void (*f)(unsigned int, char *));
int strequ(char const *s1, char const *s2);
int strnequ(char const *s1, char const *s2, size_t n);

void putchar(char c);
void putstr(char const *s);
void putendl(char const *s);
void putnbr(int n);
void putnbr_base(int n, int base);

void *aligned_memcpy(void *restrict dst, const void *restrict src, size_t n);
void aligned_bzero(void *s, size_t n);

#endif
