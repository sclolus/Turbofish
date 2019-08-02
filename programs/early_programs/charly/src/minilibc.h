#ifndef __MINILIBC_H__
# define __MINILIBC_H__

typedef long int ssize_t;
typedef unsigned long int size_t;

ssize_t write(int fd, const void *buf, size_t count);
void exit(int status);

#endif
