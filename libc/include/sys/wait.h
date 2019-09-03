#ifndef __WAIT_H__
# define __WAIT_H__

#include <i386.h>

#include <sys/types.h>
/*
 * The <sys/wait.h> header shall define the following symbolic constants for use with waitpid():
 */

/*
 * Overview of exit status in turbofish:
 * bit 0..7   : basic return value
 * bit 8..12  : signal exit value
 * bit 13     : signal stoped state    (with WUNTRACED)
 * bit 14     : signal continue state  (with WCONTINUED)
 */

#define EXITED_STATUS_BITS      0x00ff
#define SIGNALED_STATUS_BITS    0x1f00
#define STOPPED_STATUS_BIT      0x2000
#define CONTINUED_STATUS_BIT    0x4000

#define SIGNALED_STATUS_SHIFT   8

/*
 * Option bits for the second argument of waitpid.  WNOHANG causes the
 * wait to not hang if there are no stopped or terminated processes, rather
 * returning an error indication in this case (pid==0).  WUNTRACED
 * indicates that the caller should receive status about untraced children
 * which stop due to signals.  If children are stopped and a wait without
 * this option is done, it is as though they were still running... nothing
 * about them is returned.
 */
#define WNOHANG       0x1 /* dont hang in wait */
#define WUNTRACED     0x2 /* tell about stopped, untraced children */
#define WCONTINUED    0x4 /* tell me about continued */

/* 
 * returns the exit status of the child.  This consists of the least
 * significant 8 bits of the status argument that the child specified
 * in a call to exit(3) or _exit(2) or as the argument for a return
 * statement in main().  This macro should be employed only if WIFEXITED
 * returned true.
 * We can interpret the return value as unsigned [0;+255] or as signed [-128;+127]
 */
#define	WEXITSTATUS(status)	((status) & EXITED_STATUS_BITS)

/* returns true if the child process was terminated by a signal. */
#define WIFSIGNALED(status) \
	((((status) & SIGNALED_STATUS_BITS) > 0))

/* 
 * returns the number of the signal that caused the child process to
 * terminate. This macro should be employed only if WIFSIGNALED returned
 * true.
 * As unsigned, the range is [0;+31]
 */
#define	WTERMSIG(status)	(((status) & SIGNALED_STATUS_BITS) >> SIGNALED_STATUS_SHIFT)

/*
 * TRUE if STATUS indicates normal termination by exit(n) or return(n) from main.
 * In the others cases, by a signal terminaison for exemple, this macro returns FALSE
 */
#define	WIFEXITED(status)	(WTERMSIG(status) == 0)

/*
 * returns true if the child process was stopped by delivery of a signal;
 * this is possible only if the call was done using WUNTRACED or when the
 * child is being traced (see ptrace(2)).
 */
#define	WIFSTOPPED(status)	((status) & STOPPED_STATUS_BIT)

/*
 * returns the number of the signal which caused the child to stop.
 * This macro should be employed only if WIFSTOPPED returned true.
 */
#define	WSTOPSIG(status)	WTERMSIG(status)

/*
 * Since turbofish 0.5: returns true if the child process was resumed by delivery of SIGCONT
 */
#define WIFCONTINUED(wstatus)   ((status) & CONTINUED_STATUS_BIT)

// The <sys/wait.h> header shall define the following symbolic constants as possible values for the options argument to waitid():
//
// WEXITED
//    Wait for processes that have exited.
// WNOWAIT
//    Keep the process whose status is returned in infop in a waitable state.
// WSTOPPED
//    Status is returned for any child that has stopped upon receipt of a signal.
//
// The [XSI] [Option Start]  WCONTINUED [Option End] and WNOHANG constants, described above for waitpid(), can also be used with waitid().

// The type idtype_t shall be defined as an enumeration type whose possible values shall include at least the following: P_ALL P_PGID P_PID
typedef enum idtype {
	P_ALL,
	P_PGID,
	P_PID
} idtype_t;

// The <sys/wait.h> header shall define the id_t and pid_t types as described in <sys/types.h>.
#include <sys/types.h>

// The <sys/wait.h> header shall define the siginfo_t type and the sigval union as described in <signal.h>.
// Inclusion of the <sys/wait.h> header may also make visible all symbols from <signal.h>.
#include <signal.h>

// The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.

pid_t   wait(int *status);

int     waitid(idtype_t idtype, id_t id, siginfo_t *infop, int options);

pid_t   waitpid(pid_t pid, int *status, int options);

struct rusage;

pid_t wait3(int *wstatus, int options, struct rusage *rusage);

pid_t wait4(pid_t pid, int *wstatus, int options, struct rusage *rusage);

#endif
