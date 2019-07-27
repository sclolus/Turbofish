#ifndef __WAIT_H__
# define __WAIT_H__

#include <i386.h>

#include <sys/types.h>
/*
 * Option bits for the second argument of waitpid.  WNOHANG causes the
 * wait to not hang if there are no stopped or terminated processes, rather
 * returning an error indication in this case (pid==0).  WUNTRACED
 * indicates that the caller should receive status about untraced children
 * which stop due to signals.  If children are stopped and a wait without
 * this option is done, it is as though they were still running... nothing
 * about them is returned.
 */
#define WNOHANG       1 /* dont hang in wait */
#define WUNTRACED     2 /* tell about stopped, untraced children */

/* 
 * returns the exit status of the child.  This consists of the least
 * significant 8 bits of the status argu‐ ment that the child specified
 * in a call to exit(3) or _exit(2) or as the argument for a return
 * statement in main().  This macro should be employed only if WIFEXITED
 * returned true.
 */
#define	WEXITSTATUS(status)	((status) & 0xff)

/* returns true if the child process was terminated by a signal. */
#define WIFSIGNALED(status) \
	(((signed char) (status & 0x7f00)) > 0)

/* 
 * returns the number of the signal that caused the child process to
 * terminate.  This macro should be employed only if WIFSIGNALED returned
 * true.
 */
#define	WTERMSIG(status)	((status) & 0x7f00 >> 8)


/* If WIFSTOPPED(STATUS), the signal that stopped the child.  */
#define	WSTOPSIG(status)	WEXITSTATUS(status)

/* Nonzero if STATUS indicates normal termination.  */
#define	WIFEXITED(status)	(WTERMSIG(status) == 0)

/* 
 * /\* Nonzero if STATUS indicates the child is stopped.  *\/
 * #define	WIFSTOPPED(status)	(((status) & 0xff) == 0x7f)
 */

//The <sys/wait.h> header shall define the following symbolic constants for use with waitpid():
//
//WCONTINUED
//    [XSI] [Option Start] Report status of continued child process. [Option End]
//WNOHANG
//    Do not hang if no status is available; return immediately.
//WUNTRACED
//    Report status of stopped child process.
//
//The <sys/wait.h> header shall define the following macros for analysis of process status values:
//
//WEXITSTATUS
//    Return exit status.
//WIFCONTINUED
//    [XSI] [Option Start] True if child has been continued. [Option End]
//WIFEXITED
//    True if child exited normally.
//WIFSIGNALED
//    True if child exited due to uncaught signal.
//WIFSTOPPED
//    True if child is currently stopped.
//WSTOPSIG
//    Return signal number that caused process to stop.
//WTERMSIG
//    Return signal number that caused process to terminate.
//
//The <sys/wait.h> header shall define the following symbolic constants as possible values for the options argument to waitid():
//
//WEXITED
//    Wait for processes that have exited.
//WNOWAIT
//    Keep the process whose status is returned in infop in a waitable state.
//WSTOPPED
//    Status is returned for any child that has stopped upon receipt of a signal.
//
//The [XSI] [Option Start]  WCONTINUED [Option End] and WNOHANG constants, described above for waitpid(), can also be used with waitid().

//The type idtype_t shall be defined as an enumeration type whose possible values shall include at least the following: P_ALL P_PGID P_PID
typedef idtype_t;

//The <sys/wait.h> header shall define the id_t and pid_t types as described in <sys/types.h>.

#include <sys/types.h>

//The <sys/wait.h> header shall define the siginfo_t type and the sigval union as described in <signal.h>.
#include <signal.h>


//Inclusion of the <sys/wait.h> header may also make visible all symbols from <signal.h>.

//The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.

pid_t  wait(int *);
int    waitid(idtype_t, id_t, siginfo_t *, int);
pid_t  waitpid(pid_t, int *, int);


//TODO: check that
/* #define WAIT int; */


#endif
