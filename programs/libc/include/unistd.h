#ifndef __UNISTD_H__
# define __UNISTD_H__

#include "i386.h"

int write(int fd, const char *s, size_t len);

typedef int pid_t;

int fork();
void exit(int status);
pid_t wait(int *stat_loc);


#define MAP_FAILED 0xFFFFFFFF

#define PROT_NONE 0
#define PROT_READ (1 << 0)
#define PROT_WRITE (1 << 1)
#define PROT_EXEC (1 << 2)

#define MAP_SHARED (1 << 0)
#define MAP_PRIVATE (1 << 1)
#define MAP_FIXED (1 << 4)
#define MAP_ANONYMOUS (1 << 5)
#define MAP_ANON MAP_ANONYMOUS

void *mmap(void *addr, size_t length, int prot, int flags, int fd, off_t offset);
int munmap(void *addr, size_t length);
int mprotect(void *addr, size_t length, int prot);

unsigned int sleep(unsigned int seconds);

#endif
