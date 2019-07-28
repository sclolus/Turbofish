#ifndef __SIGNAL_H__
# define __SIGNAL_H__

//#include "i386.h"

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

//[CX] [Option Start] Some of the functionality described on this reference page extends the ISO C standard. Applications shall define the appropriate feature test macro (see XSH The Compilation Environment ) to enable the visibility of these symbols in this header. [Option End]

//The <signal.h> header shall define the following macros, which shall expand to constant expressions with distinct values that have a type compatible with the second argument to, and the return value of, the signal() function, and whose values shall compare unequal to the address of any declarable function.

/* Default actions. */

#define SIG_DFL        0
//    Request for default signal handling.
//SIG_ERR
//    Return value from signal() in case of error.
//SIG_HOLD
    //[OB XSI] [Option Start] Request that signal be held. [Option End]
#define SIG_IGN        1
//   Request that signal be ignored.

//[CX] [Option Start] The <signal.h> header shall define the pthread_t, size_t, and uid_t types as described in <sys/types.h>.

#include <time.h>
//The <signal.h> header shall define the timespec structure as described in <time.h>. //[Option End]

//The <signal.h> header shall define the following data types:

#define sig_atomic_t int
//    Possibly volatile-qualified integer type of an object that can be accessed as an atomic entity, even in the presence of asynchronous interrupts.
#define sigset_t int
    //[CX] [Option Start] Integer or structure type of an object used to represent sets of signals. [Option End]
//pid_t
    //[CX] [Option Start] As described in <sys/types.h>. [Option End]

#include <sys/types.h>

//[CX] [Option Start] The <signal.h> header shall define the pthread_attr_t type as described in <sys/types.h>.

//The <signal.h> header shall define the sigevent structure, which shall include at least the following members:

//The sigval union shall be defined as:

typedef union sigval {
	int    sival_int;//    Integer signal value. 
	void  *sival_ptr;//    Pointer signal value. 
} sigval_t; 

struct sigevent {
	int              sigev_notify            ;//Notification type. 
	int              sigev_signo             ;//Signal number. 
	union sigval     sigev_value             ;//Signal value. 
	void           (*sigev_notify_function)(union sigval);
	//Notification function. 
	pthread_attr_t *sigev_notify_attributes;//  Notification attributes. 
};

//The <signal.h> header shall define the following symbolic constants for the values of sigev_notify:

//SIGEV_NONE
//    No asynchronous notification is delivered when the event of interest occurs.
//SIGEV_SIGNAL
//    A queued signal, with an application-defined value, is generated when the event of interest occurs.
//SIGEV_THREAD
//    A notification function is called to perform notification.



//The <signal.h> header shall declare the SIGRTMIN and SIGRTMAX macros, which shall expand to positive integer expressions with type int, but which need not be constant expressions. These macros specify a range of signal numbers that are reserved for application use and for which the realtime signal behavior specified in this volume of POSIX.1-2017 is supported. The signal numbers in this range do not overlap any of the signals specified in the following table.

//The range SIGRTMIN through SIGRTMAX inclusive shall include at least {RTSIG_MAX} signal numbers.

//It is implementation-defined whether realtime signal behavior is supported for other signals. //[Option End]

//The <signal.h> header shall define the following macros that are used to refer to the signals that occur in the system. Signals defined here begin with the letters SIG followed by an uppercase letter. The macros shall expand to positive integer constant expressions with type int and distinct values. The value 0 is reserved for use as the null signal (see kill()). Additional implementation-defined signals may occur in the system.

//The ISO C standard only requires the signal names SIGABRT, SIGFPE, SIGILL, SIGINT, SIGSEGV, and SIGTERM to be defined. An implementation need not generate any of these six signals, except as a result of explicit use of interfaces that generate signals, such as raise(), //[CX] [Option Start] kill(), the General Terminal Interface (see Special Characters), and the kill utility, unless otherwise stated (see, for example, XSH Memory Protection). [Option End]

//The following signals shall be supported on all implementations (default actions are explained below the table):

//[CX] [Option Start] The <signal.h> header shall define the mcontext_t type through typedef. [Option End]

//The <signal.h> header shall define the stack_t type as a structure, which shall include at least the following members:

typedef int mcontext_t;

typedef struct stack {
	void     *ss_sp       ;//Stack base or pointer. 
	size_t    ss_size     ;//Stack size. 
	int       ss_flags    ;//Flags. 
} stack_t;
//[CX] [Option Start] The <signal.h> header shall define the ucontext_t type as a structure that shall include at least the following members:

typedef struct ucontext {
	struct ucontext *uc_link    ; // Pointer to the context that is resumed 
	// when this context returns. 
	sigset_t    uc_sigmask ; // The set of signals that are blocked when this 
	// context is active. 
	stack_t     uc_stack   ; // The stack used by this context. 
	mcontext_t  uc_mcontext; // A machine-specific representation of the saved 
							 //context. 
} ucontext_t;

//[Option End]

//[CX] [Option Start] The <signal.h> header shall define the siginfo_t type as a structure, which shall include at least the following members: [Option End]
/* 
 * [CX][Option Start]
 * int           si_signo  Signal number. 
 * int           si_code   Signal code. 
 * [Option End]
 * [XSI][Option Start]
 * int           si_errno  If non-zero, an errno value associated with 
 *                       this signal, as described in <errno.h>. 
 * [Option End]
 * [CX][Option Start]
 * pid_t         si_pid    Sending process ID. 
 * uid_t         si_uid    Real user ID of sending process. 
 * void         *si_addr   Address of faulting instruction. 
 * int           si_status Exit value or signal. 
 * [Option End]
 * [OB XSR][Option Start]
 * long          si_band   Band event for SIGPOLL. 
 * [Option End]
 * [CX][Option Start]
 * union sigval  si_value  Signal value. 
 * [Option End]
 */

typedef struct siginfo {
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
} siginfo_t;

//[CX] [Option Start] The <signal.h> header shall declare the sigaction structure, which shall include at least the following members:

/* 
 * void   (*sa_handler)(int)  Pointer to a signal-catching function 
 *                            or one of the SIG_IGN or SIG_DFL. 
 * sigset_t sa_mask           Set of signals to be blocked during execution 
 *                            of the signal handling function. 
 * int      sa_flags          Special flags. 
 * void   (*sa_sigaction)(int, siginfo_t *, void *)
 *                            Pointer to a signal-catching function. 
 */
struct sigaction {
	union {
		void     (*sa_handler)(int);
		void     (*sa_sigaction)(int, siginfo_t *, void *);
	};
	sigset_t   sa_mask;
	int        sa_flags;
	void     (*sa_restorer)(void);
};


//[Option End]

//[CX] [Option Start] The storage occupied by sa_handler and sa_sigaction may overlap, and a conforming application shall not use both simultaneously. [Option End]

//The <signal.h> header shall define the following macros which shall expand to integer constant expressions that need not be usable in #if preprocessing directives:

#define SIG_BLOCK 42
    //[CX] [Option Start] The resulting set is the union of the current set and the signal set pointed to by the argument set. [Option End]
#define SIG_UNBLOCK 42
    //[CX] [Option Start] The resulting set is the intersection of the current set and the complement of the signal set pointed to by the argument set. [Option End]
#define SIG_SETMASK 42
    //[CX] [Option Start] The resulting set is the signal set pointed to by the argument set. [Option End]

//The <signal.h> header shall also define the following symbolic constants:

/* 
 * SA_NOCLDSTOP
 *     //[CX] [Option Start] Do not generate SIGCHLD when children stop [Option End]
 *     //[XSI] [Option Start]  or stopped children continue. [Option End]
 * SA_ONSTACK
 *     //[XSI] [Option Start] Causes signal delivery to occur on an alternate stack. [Option End]
 * SA_RESETHAND
 *     //[CX] [Option Start] Causes signal dispositions to be set to SIG_DFL on entry to signal handlers. [Option End]
 * SA_RESTART
 *     //[CX] [Option Start] Causes certain functions to become restartable. [Option End]
 * SA_SIGINFO
 *     //[CX] [Option Start] Causes extra information to be passed to signal handlers at the time of receipt of a signal. [Option End]
 * SA_NOCLDWAIT
 *     //[XSI] [Option Start] Causes implementations not to create zombie processes or status information on child termination. See sigaction. [Option End]
 * SA_NODEFER
 *     //[CX] [Option Start] Causes signal not to be automatically blocked on entry to signal handler. [Option End]
 * SS_ONSTACK
 *     //[XSI] [Option Start] Process is executing on an alternate signal stack. [Option End]
 * SS_DISABLE
 *     //[XSI] [Option Start] Alternate signal stack is disabled. [Option End]
 * MINSIGSTKSZ
 *     //[XSI] [Option Start] Minimum stack size for a signal handler. [Option End]
 * SIGSTKSZ
 *     //[XSI] [Option Start] Default size in bytes for the alternate signal stack. [Option End]
 */


//[CX] [Option Start] The <signal.h> header shall define the symbolic constants in the Code column of the following table for use as values of si_code that are signal-specific or non-signal-specific reasons why the signal was generated. [Option End]

//If si_code is equal to CLD_EXITED, then si_status holds the exit value of the process; otherwise, it is equal to the signal that caused the process to change state. The exit value in si_status shall be equal to the full exit value (that is, the value passed to _exit(), _Exit(), or exit(), or returned from main()); it shall not be limited to the least significant eight bits of the value.

//Band event for POLL_IN, POLL_OUT, or POLL_MSG.

//For some implementations, the value of si_addr may be inaccurate.

//The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.

//[CX][Option Start]
int    kill(pid_t, int);
//[Option End]
//[XSI][Option Start]
int    killpg(pid_t, int);
//[Option End]
//[CX][Option Start]
void   psiginfo(const siginfo_t *, const char *);
void   psignal(int, const char *);
int    pthread_kill(pthread_t, int);
int    pthread_sigmask(int, const sigset_t *restrict,
           sigset_t *restrict);
//[Option End]
int    raise(int);
//[CX][Option Start]
int    sigaction(int, const struct sigaction *restrict,
           struct sigaction *restrict);
int    sigaddset(sigset_t *, int);

//[XSI][Option Start]
int    sigaltstack(const stack_t *restrict, stack_t *restrict);
//[Option End]
//[CX][Option Start]
int    sigdelset(sigset_t *, int);
int    sigemptyset(sigset_t *);
int    sigfillset(sigset_t *);
//[Option End]
//[OB XSI][Option Start]
int    sighold(int);
int    sigignore(int);
int    siginterrupt(int, int);
//[Option End]
//[CX][Option Start]
int    sigismember(const sigset_t *, int);
//[Option End]
void (*signal(int, void (*)(int)))(int);
//[OB XSI][Option Start]
int    sigpause(int);
//[Option End]
//[CX][Option Start]
int    sigpending(sigset_t *);
int    sigprocmask(int, const sigset_t *restrict, sigset_t *restrict);
int    sigqueue(pid_t, int, union sigval);
//[Option End]
//[OB XSI][Option Start]
int    sigrelse(int);
void (*sigset(int, void (*)(int)))(int);
//[Option End]
//[CX][Option Start]
int    sigsuspend(const sigset_t *);
int    sigtimedwait(const sigset_t *restrict, siginfo_t *restrict, const struct timespec *restrict);
int    sigwait(const sigset_t *restrict, int *restrict);
int    sigwaitinfo(const sigset_t *restrict, siginfo_t *restrict);
//[Option End]

//[CX] [Option Start] Inclusion of the <signal.h> header may make visible all symbols from the <time.h> header. [Option End]

//TODO: check NON POSIX
#define NSIG 42

#endif
