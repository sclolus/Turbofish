#ifndef __UNISTD_H__
# define __UNISTD_H__

#include "i386.h"

int write(int fd, const char *s, size_t len);

#define MAP_FAILED 0xFFFFFFFF
#define PROT_READ 0
#define PROT_WRITE 0
#define MAP_ANON 0
#define MAP_PRIVATE 0

void *mmap(void *addr, size_t length, int prot, int flags, int fd, off_t offset);
int munmap(void *addr, size_t length);

#endif
