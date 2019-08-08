#ifndef __USER_SYSCALL_H__
# define __USER_SYSCALL_H__

#include <i386.h>

int _user_syscall(u32 syscall_number, u32 args_len, ...);

#define EXIT          1
#define FORK          2
#define READ          3
#define WRITE         4
#define CLOSE         6
#define WAITPID       7
#define UNLINK       10
#define EXECVE       11
#define GETPID       20
#define GETUID       24
#define PAUSE        29
#define KILL         37
#define SIGNAL       48
#define SETPGID      57
#define GETPPID		 64
#define GETPGRP		 65
#define SIGACTION    67
#define SIGSUSPEND   72
#define REBOOT       88
#define MMAP         90
#define MUNMAP       91
#define SOCKETCALL  102
#define CLONE       120
#define MPROTECT    125
#define SIGPROCMASK 126
#define GETPGID		132
#define NANOSLEEP   162
#define SHUTDOWN    293

#define EXIT_QEMU   0x80000002
#define TCGETATTR   0x80000003
#define TCSETATTR   0x80000004

#endif
