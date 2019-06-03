#ifndef __STDLIB_H__
# define __STDLIB_H__

#include "i386.h"

void exit(int status);

void *malloc(size_t size);
int free(void *ptr);
void *calloc(size_t count, size_t size);
void *realloc(void *ptr, size_t size);

#endif
