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

#endif
