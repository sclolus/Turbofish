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

// TODO: Verify that before implemting sigaction
typedef u32 pid_t;
typedef u32 uid_t;
typedef u32 clock_t;
typedef u32 sigval_t;

struct siginfo {
	int      si_signo;       /* Numéro de signal         */
	int      si_errno;       /* Numéro d'erreur          */
	int      si_code;        /* Code du signal           */
	pid_t    si_pid;         /* PID de l'émetteur        */
	uid_t    si_uid;         /* UID réel de l'émetteur   */
	int      si_status;      /* Valeur de sortie         */
	clock_t  si_utime;       /* Temps utilisateur écoulé */
	clock_t  si_stime;       /* Temps système écoulé     */
	sigval_t si_value;       /* Valeur de signal         */
	int      si_int;         /* Signal POSIX.1b          */
	void    *si_ptr;         /* Signal POSIX.1b          */
	void    *si_addr;        /* Emplacement d'erreur     */
	int      si_band;        /* Band event               */
	int      si_fd;          /* Descripteur de fichier   */
};

typedef struct siginfo siginfo_t;

// TODO: Modify that dummy code
typedef u8 sigset_t[128];

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

#endif
