#ifndef __TIMES_H__
# define __TIMES_H__

//The <sys/times.h> header shall define the tms structure, which is returned by times() and shall include at least the following members:

#include <sys/types.h>

struct tms {
	clock_t  tms_utime ;// User CPU time. 
	clock_t  tms_stime ;// System CPU time. 
	clock_t  tms_cutime;// User CPU time of terminated child processes. 
	clock_t  tms_cstime;// System CPU time of terminated child processes. 
};

//The <sys/times.h> header shall define the clock_t type as described in <sys/types.h>.

//The following shall be declared as a function and may also be defined as a macro. A function prototype shall be provided.

clock_t times(struct tms *);

#endif
