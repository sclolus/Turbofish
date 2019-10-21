#ifndef __CUSTOM_H__
# define __CUSTOM_H__

#include <stdio.h>
#include <unistd.h>
#include <stdbool.h>

#define DUMMY
#define DUMMY_KERNEL

void exit_qemu(int status);
bool is_ptr_valid(const char *path);

#endif
