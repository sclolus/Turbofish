#ifndef __CUSTOM_H__
# define __CUSTOM_H__

#include <stdio.h>
#include <unistd.h>
#include <stdbool.h>

/* #define DUMMY dprintf(STDERR_FILENO, "dummy function called: %s\n", __func__); */
/* #define DUMMY_KERNEL dprintf(STDERR_FILENO, "dummy 'Kernel' implementation called: %s\n", __func__); */

#define DUMMY do {} while (0);
#define DUMMY_KERNEL do {} while (0);

void exit_qemu(int status);
bool is_ptr_valid(const char *path);

#endif
