#ifndef __USER_SYSCALL_H__
# define __USER_SYSCALL_H__

# include <stdint.h>

int _user_syscall(u32 syscall_number, u32 args_len, ...);

#define EXIT          1
#define FORK          2
#define READ          3
#define WRITE         4
#define OPEN          5
#define CLOSE         6
#define WAITPID       7
#define LINK          9
#define UNLINK       10
#define EXECVE       11
#define CHDIR        12
#define MKNOD        14
#define CHMOD        15
#define STAT         18
#define LSEEK        19
#define GETPID       20
#define MOUNT        21
#define SETUID       23
#define GETUID       24
#define PAUSE        29
#define FSTAT        28
#define UTIME        30
#define ACCESS       33
#define KILL         37
#define RENAME       38
#define MKDIR        39
#define RMDIR        40
#define DUP          41
#define PIPE         42
#define TIMES	     43
#define SETGID       46
#define GETGID       47
#define GETEUID      49
#define GETEGID      50
#define UMOUNT       52
#define FCNTL        55
#define SIGNAL       48
#define SETPGID      57
#define UMASK	     60
#define DUP2         63
#define GETPPID      64
#define GETPGRP      65
#define SIGACTION    67
#define SIGSUSPEND   72
#define GETGROUPS    80
#define SETGROUPS    81
#define SYMLINK      83
#define LSTAT        84
#define READLINK     85
#define REBOOT       88
#define MMAP         90
#define MUNMAP       91
#define FCHMOD	     94
#define FCHOWN	     95
#define GETTIMEOFDAY 96
#define SOCKETCALL  102
#define CLONE       120
#define MPROTECT    125
#define SIGPROCMASK 126
#define GETPGID     132
#define STATFS	    137
#define FSTATFS	    138
#define NANOSLEEP   162
#define CHOWN       182
#define GETCWD      183
#define SIGRETURN   200
#define SHUTDOWN    293

#define TEST            0x80000000
#define STACK_OVERFLOW  0x80000001
#define EXIT_QEMU       0x80000002
#define TCGETATTR       0x80000003
#define TCSETATTR       0x80000004
#define TCGETPGRP       0x80000005
#define TCSETPGRP       0x80000006
#define SETEGID         0x80000007
#define SETEUID         0x80000008
#define ISATTY          0x80000009
#define OPENDIR         0x80000010
#define IS_STR_VALID    0x80000011

/*
 * Module Kernel specific
 */
#define INSMOD          0xC0000000
#define RMMOD           0xC0000001

#endif
