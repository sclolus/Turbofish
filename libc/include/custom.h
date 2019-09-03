#ifndef __CUSTOM_H__
# define __CUSTOM_H__

#include <stdio.h>
#include <unistd.h>

#define DUMMY dprintf(STDERR_FILENO, "dummy function called: %s\n", __func__);
#define DUMMY_KERNEL dprintf(STDERR_FILENO, "dummy 'Kernel' implementation called: %s\n", __func__);

void exit_qemu(int status);

#endif
