#ifndef __RESOURCE_H__
# define __RESOURCE_H__

#include <sys/types.h>
#include <sys/time.h>
//The <sys/resource.h> header shall define the following symbolic constants as possible values of the which argument of getpriority() and setpriority():

//PRIO_PROCESS
//    Identifies the who argument as a process ID.
//PRIO_PGRP
//    Identifies the who argument as a process group ID.
//PRIO_USER
//    Identifies the who argument as a user ID.
//
//The <sys/resource.h> header shall define the following type through typedef:
//
typedef int rlim_t;
//    Unsigned integer type used for limit values.
//
//The <sys/resource.h> header shall define the following symbolic constants, which shall have values suitable for use in #if preprocessing directives:
//
//RLIM_INFINITY
//    A value of rlim_t indicating no limit.
//RLIM_SAVED_MAX
//    A value of type rlim_t indicating an unrepresentable saved hard limit.
//RLIM_SAVED_CUR
//    A value of type rlim_t indicating an unrepresentable saved soft limit.
//
//On implementations where all resource limits are representable in an object of type rlim_t, RLIM_SAVED_MAX and RLIM_SAVED_CUR need not be distinct from RLIM_INFINITY.
//
//The <sys/resource.h> header shall define the following symbolic constants as possible values of the who parameter of getrusage():
//
#define RUSAGE_SELF 42
//    Returns information about the current process.
#define RUSAGE_CHILDREN 42
//    Returns information about children of the current process.
//
//The <sys/resource.h> header shall define the rlimit structure, which shall include at least the following members:
//
struct rlimit {
	rlim_t rlim_cur; //The current (soft) limit. 
	rlim_t rlim_max; //The hard limit. 
};
//
//The <sys/resource.h> header shall define the rusage structure, which shall include at least the following members:
//
struct rusage {
	struct timeval ru_utime; //  User time used. 
	struct timeval ru_stime; //  System time used. 
};
//
//The <sys/resource.h> header shall define the timeval structure as described in <sys/time.h>.
//
//The <sys/resource.h> header shall define the following symbolic constants as possible values for the resource argument of getrlimit() and setrlimit():
//
#define RLIMIT_CORE 0
//    Limit on size of core file.
#define RLIMIT_CPU  1
//    Limit on CPU time per process.
#define RLIMIT_DATA 2
//    Limit on data segment size.
#define RLIMIT_FSIZE 3
//    Limit on file size.
#define RLIMIT_NOFILE 4
//    Limit on number of open files.
#define RLIMIT_STACK 5
//    Limit on stack size.
#define RLIMIT_AS 6
//    Limit on address space size.
//
//The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.

int getpriority(int, id_t);
int getrlimit(int, struct rlimit *);
int getrusage(int, struct rusage *);
int setpriority(int, id_t, int);
int setrlimit(int, const struct rlimit *);

//The <sys/resource.h> header shall define the id_t type through typedef, as described in <sys/types.h>.
//
//Inclusion of the <sys/resource.h> header may also make visible all symbols from <sys/time.h>.

#endif
