#ifndef __SIGNAL_H__
# define __SIGNAL_H__

/* Signals.  */
enum Signum {
	SIGNULL     =  0 ,     /* NULL signal for logical raisons */

	SIGHUP      =  1 ,     /* Hangup (POSIX).  */
	SIGINT      =  2 ,     /* Interrupt (ANSI).  */
	SIGQUIT     =  3 ,     /* Quit (POSIX).  */
	SIGILL      =  4 ,     /* Illegal instruction (ANSI).  */
	SIGTRAP     =  5 ,     /* Trace trap (POSIX).  */
	SIGABRT     =  6 ,     /* Abort (ANSI).  */
	SIGIOT      =  6 ,     /* IOT trap (4.2 BSD).  */
	SIGBUS      =  7 ,     /* BUS error (4.2 BSD).  */
	SIGFPE      =  8 ,     /* Floating-point exception (ANSI).  */
	SIGKILL     =  9 ,     /* Kill, unblockable (POSIX).  */
	SIGUSR1     =  10,     /* User-defined signal 1 (POSIX).  */
	SIGSEGV     =  11,     /* Segmentation violation (ANSI).  */
	SIGUSR2     =  12,     /* User-defined signal 2 (POSIX).  */
	SIGPIPE     =  13,     /* Broken pipe (POSIX).  */
	SIGALRM     =  14,     /* Alarm clock (POSIX).  */
	SIGTERM     =  15,     /* Termination (ANSI).  */
	SIGSTKFLT   =  16,     /* Stack fault.  */
	SIGCHLD     =  17,     /* Child status has changed (POSIX).  */
	SIGCLD      =  SIGCHLD,/* Same as SIGCHLD (System V).  */
	SIGCONT     =  18,     /* Continue (POSIX).  */
	SIGSTOP     =  19,     /* Stop, unblockable (POSIX).  */
	SIGTSTP     =  20,     /* Keyboard stop (POSIX).  */
	SIGTTIN     =  21,     /* Background read from tty (POSIX).  */
	SIGTTOU     =  22,     /* Background write to tty (POSIX).  */
	SIGURG      =  23,     /* Urgent condition on socket (4.2 BSD).  */
	SIGXCPU     =  24,     /* CPU limit exceeded (4.2 BSD).  */
	SIGXFSZ     =  25,     /* File size limit exceeded (4.2 BSD).  */
	SIGVTALRM   =  26,     /* Virtual alarm clock (4.2 BSD).  */
	SIGPROF     =  27,     /* Profiling alarm clock (4.2 BSD).  */
	SIGWINCH    =  28,     /* Window size change (4.3 BSD, Sun).  */
	SIGIO       =  29,     /* I/O now possible (4.2 BSD).  */
	SIGPOLL     =  SIGIO,  /* Pollable event occurred (System V).  */
	SIGPWR      =  30,     /* Power failure restart (System V).  */
	SIGSYS      =  31,     /* Bad system call.  */
	SIGUNUSED   =  31
};

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
 * --- POSIX COMMENTARY ADDITION ---
 * SA_NOCLDSTOP
 *     [CX] [Option Start] Do not generate SIGCHLD when children stop [Option End]
 *     [XSI] [Option Start] or stopped children continue. [Option End]
 * SA_ONSTACK
 *     [XSI] [Option Start] Causes signal delivery to occur on an alternate stack. [Option End]
 * SA_RESETHAND
 *     [CX] [Option Start] Causes signal dispositions to be set to SIG_DFL on entry to signal handlers. [Option End]
 * SA_RESTART
 *     [CX] [Option Start] Causes certain functions to become restartable. [Option End]
 * SA_SIGINFO
 *     [CX] [Option Start] Causes extra information to be passed to signal handlers at the time of receipt of a signal. [Option End]
 * SA_NOCLDWAIT
 *     [XSI] [Option Start] Causes implementations not to create zombie processes or status information on child termination. See sigaction. [Option End]
 * SA_NODEFER
 *     [CX] [Option Start] Causes signal not to be automatically blocked on entry to signal handler. [Option End]
 * SS_ONSTACK
 *     [XSI] [Option Start] Process is executing on an alternate signal stack. [Option End]
 * SS_DISABLE
 *     [XSI] [Option Start] Alternate signal stack is disabled. [Option End]
 * MINSIGSTKSZ
 *     [XSI] [Option Start] Minimum stack size for a signal handler. [Option End]
 */
#define SIGSTKSZ       8192
  //     [XSI] [Option Start] Default size in bytes for the alternate signal stack. [Option End]

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

#define SIG_DFL ((sighandler_t)0)
//    Request for default signal handling.
#define SIG_ERR ((sighandler_t)-1)
//    Return value from signal() in case of error.
//SIG_HOLD
    //[OB XSI] [Option Start] Request that signal be held. [Option End]
#define SIG_IGN ((sighandler_t)1)
//   Request that signal be ignored.

//[CX] [Option Start] The <signal.h> header shall define the pthread_t, size_t, and uid_t types as described in <sys/types.h>.

#include <time.h>
//The <signal.h> header shall define the timespec structure as described in <time.h>. //[Option End]

//The <signal.h> header shall define the following data types:

#define sig_atomic_t int
//    Possibly volatile-qualified integer type of an object that can be accessed as an atomic entity, even in the presence of asynchronous interrupts.
#define sigset_t unsigned int
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

#define SIG_BLOCK 0
    //[CX] [Option Start] The resulting set is the union of the current set and the signal set pointed to by the argument set. [Option End]
#define SIG_UNBLOCK 1
    //[CX] [Option Start] The resulting set is the intersection of the current set and the complement of the signal set pointed to by the argument set. [Option End]
#define SIG_SETMASK 2
    //[CX] [Option Start] The resulting set is the signal set pointed to by the argument set. [Option End]

//The <signal.h> header shall also define the following symbolic constants:

//[CX] [Option Start] The <signal.h> header shall define the symbolic constants in the Code column of the following table for use as values of si_code that are signal-specific or non-signal-specific reasons why the signal was generated. [Option End]

/*
 * SIGILL si_codes
 */
#define ILL_ILLOPC	1	/* illegal opcode */
#define ILL_ILLOPN	2	/* illegal operand */
#define ILL_ILLADR	3	/* illegal addressing mode */
#define ILL_ILLTRP	4	/* illegal trap */
#define ILL_PRVOPC	5	/* privileged opcode */
#define ILL_PRVREG	6	/* privileged register */
#define ILL_COPROC	7	/* coprocessor error */
#define ILL_BADSTK	8	/* internal stack error */
#define NSIGILL		8

/*
 * SIGFPE si_codes
 */
#define FPE_INTDIV	1	/* integer divide by zero */
#define FPE_INTOVF	2	/* integer overflow */
#define FPE_FLTDIV	3	/* floating point divide by zero */
#define FPE_FLTOVF	4	/* floating point overflow */
#define FPE_FLTUND	5	/* floating point underflow */
#define FPE_FLTRES	6	/* floating point inexact result */
#define FPE_FLTINV	7	/* floating point invalid operation */
#define FPE_FLTSUB	8	/* subscript out of range */
#define NSIGFPE		8

/*
 * SIGSEGV si_codes
 */
#define SEGV_MAPERR	1	/* address not mapped to object */
#define SEGV_ACCERR	2	/* invalid permissions for mapped object */
#define SEGV_BNDERR	3	/* failed address bound checks */
#define SEGV_PKUERR	4	/* failed protection key checks */
#define NSIGSEGV	4

/*
 * SIGBUS si_codes
 */
#define BUS_ADRALN	1	/* invalid address alignment */
#define BUS_ADRERR	2	/* non-existent physical address */
#define BUS_OBJERR	3	/* object specific hardware error */
/* hardware memory error consumed on a machine check: action required */
#define BUS_MCEERR_AR	4
/* hardware memory error detected in process but not consumed: action optional*/
#define BUS_MCEERR_AO	5
#define NSIGBUS		5

/*
 * SIGTRAP si_codes
 */
#define TRAP_BRKPT	1	/* process breakpoint */
#define TRAP_TRACE	2	/* process trace trap */
#define TRAP_BRANCH     3	/* process taken branch trap */
#define TRAP_HWBKPT     4	/* hardware breakpoint/watchpoint */
#define NSIGTRAP	4

/*
 * SIGCHLD si_codes
 */
#define CLD_EXITED	1	/* child has exited */
#define CLD_KILLED	2	/* child was killed */
#define CLD_DUMPED	3	/* child terminated abnormally */
#define CLD_TRAPPED	4	/* traced child has trapped */
#define CLD_STOPPED	5	/* child has stopped */
#define CLD_CONTINUED	6	/* stopped child has continued */
#define NSIGCHLD	6

/*
 * SIGPOLL (or any other signal without signal specific si_codes) si_codes
 */
#define POLL_IN		1	/* data input available */
#define POLL_OUT	2	/* output buffers available */
#define POLL_MSG	3	/* input message available */
#define POLL_ERR	4	/* i/o error */
#define POLL_PRI	5	/* high priority input available */
#define POLL_HUP	6	/* device disconnected */
#define NSIGPOLL	6

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
