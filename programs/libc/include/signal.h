#ifndef __SIGNAL_H__
# define __SIGNAL_H__

#include "i386.h"

/* Signals.  */
#define SIGHUP         1       /* Hangup (POSIX).  */
#define SIGINT         2       /* Interrupt (ANSI).  */
#define SIGQUIT        3       /* Quit (POSIX).  */
#define SIGILL         4       /* Illegal instruction (ANSI).  */
#define SIGTRAP        5       /* Trace trap (POSIX).  */
#define SIGABRT        6       /* Abort (ANSI).  */
#define SIGIOT         6       /* IOT trap (4.2 BSD).  */
#define SIGBUS         7       /* BUS error (4.2 BSD).  */
#define SIGFPE         8       /* Floating-point exception (ANSI).  */
#define SIGKILL        9       /* Kill, unblockable (POSIX).  */
#define SIGUSR1        10      /* User-defined signal 1 (POSIX).  */
#define SIGSEGV        11      /* Segmentation violation (ANSI).  */
#define SIGUSR2        12      /* User-defined signal 2 (POSIX).  */
#define SIGPIPE        13      /* Broken pipe (POSIX).  */
#define SIGALRM        14      /* Alarm clock (POSIX).  */
#define SIGTERM        15      /* Termination (ANSI).  */
#define SIGSTKFLT      16      /* Stack fault.  */
#define SIGCLD         SIGCHLD /* Same as SIGCHLD (System V).  */
#define SIGCHLD        17      /* Child status has changed (POSIX).  */
#define SIGCONT        18      /* Continue (POSIX).  */
#define SIGSTOP        19      /* Stop, unblockable (POSIX).  */
#define SIGTSTP        20      /* Keyboard stop (POSIX).  */
#define SIGTTIN        21      /* Background read from tty (POSIX).  */
#define SIGTTOU        22      /* Background write to tty (POSIX).  */
#define SIGURG         23      /* Urgent condition on socket (4.2 BSD).  */
#define SIGXCPU        24      /* CPU limit exceeded (4.2 BSD).  */
#define SIGXFSZ        25      /* File size limit exceeded (4.2 BSD).  */
#define SIGVTALRM      26      /* Virtual alarm clock (4.2 BSD).  */
#define SIGPROF        27      /* Profiling alarm clock (4.2 BSD).  */
#define SIGWINCH       28      /* Window size change (4.3 BSD, Sun).  */
#define SIGPOLL        SIGIO   /* Pollable event occurred (System V).  */
#define SIGIO          29      /* I/O now possible (4.2 BSD).  */
#define SIGPWR         30      /* Power failure restart (System V).  */
#define SIGSYS         31      /* Bad system call.  */
#define SIGUNUSED      31

typedef void (*sighandler_t)(int);

sighandler_t signal(int signum, sighandler_t handler);

// TODO: Verify that before implementing sigaction
typedef int pid_t;
typedef int uid_t;
typedef u32 clock_t;
typedef u32 sigval_t;

struct siginfo {
	int      si_signo;       /* Signal number            */
	int      si_errno;       /* Error number             */
	int      si_code;        /* Signal code              */
	pid_t    si_pid;         /* Transmiter PID           */
	uid_t    si_uid;         /* Trasnmiter real UID      */
	int      si_status;      /* Output value             */
	clock_t  si_utime;       /* Elapsed user time        */
	clock_t  si_stime;       /* Elapsed system time      */
	sigval_t si_value;       /* Signal value             */
	int      si_int;         /* Signal POSIX.1b          */
	void    *si_ptr;         /* Signal POSIX.1b          */
	void    *si_addr;        /* Error place              */
	int      si_band;        /* Band event               */
	int      si_fd;          /* File descriptor          */
};

/*
 * SA_FLAGS values:
 *
 * SA_ONSTACK indicates that a registered stack_t will be used.
 * SA_RESTART flag to get restarting signals (which were the default long ago)
 * SA_NOCLDSTOP flag to turn off SIGCHLD when children stop.
 * SA_RESETHAND clears the handler when the signal is delivered.
 * SA_NOCLDWAIT flag on SIGCHLD to inhibit zombies.
 * SA_NODEFER prevents the current signal from being masked in the handler.
 *
 * SA_ONESHOT and SA_NOMASK are the historical Linux names for the Single
 * Unix names RESETHAND and NODEFER respectively.
 */
#define SA_NOCLDSTOP 0x00000001u
#define SA_NOCLDWAIT 0x00000002u
#define SA_SIGINFO   0x00000004u
#define SA_ONSTACK   0x08000000u
#define SA_RESTART   0x10000000u
#define SA_NODEFER   0x40000000u
#define SA_RESETHAND 0x80000000u

#define SA_NOMASK    SA_NODEFER
#define SA_ONESHOT   SA_RESETHAND

#define SA_RESTORER  0x04000000

typedef struct siginfo siginfo_t;

// TODO: Modify that dummy code
typedef u32 sigset_t;

struct sigaction {
	union {
		void     (*sa_handler)(int);
		void     (*sa_sigaction)(int, siginfo_t *, void *);
	};
	sigset_t   sa_mask;
	int        sa_flags;
	void     (*sa_restorer)(void);
};

// TODO: This function is on dummy state: Use signal instead of sigaction
int sigaction(int signum, const struct sigaction *act, struct sigaction *oldact);

int kill(pid_t pid, int sig);

#endif
