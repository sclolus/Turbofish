#ifndef __WAIT_H__
# define __WAIT_H__

#include "i386.h"

typedef int pid_t;

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

pid_t wait(int *wstatus);

pid_t waitpid(pid_t pid, int *wstatus, int options);

/* 
 * returns the exit status of the child.  This consists of the least
 * significant 8 bits of the status argu‐ ment that the child specified
 * in a call to exit(3) or _exit(2) or as the argument for a return
 * statement in main().  This macro should be employed only if WIFEXITED
 * returned true.
 */
#define	WEXITSTATUS(status)	(((status) & 0xff00) >> 8)

/* returns true if the child process was terminated by a signal. */
#define WIFSIGNALED(status) \
  (((signed char) (((status) & 0x7f) + 1) >> 1) > 0)

/* 
 * returns the number of the signal that caused the child process to
 * terminate.  This macro should be employed only if WIFSIGNALED returned
 * true.
 */
#define	WTERMSIG(status)	((status) & 0x7f)


/* If WIFSTOPPED(STATUS), the signal that stopped the child.  */
#define	WSTOPSIG(status)	WEXITSTATUS(status)

/* Nonzero if STATUS indicates normal termination.  */
#define	WIFEXITED(status)	(WTERMSIG(status) == 0)

/* Nonzero if STATUS indicates the child is stopped.  */
#define	WIFSTOPPED(status)	(((status) & 0xff) == 0x7f)

#endif
