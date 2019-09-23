#ifndef __UTMP_H__
# define __UTMP_H__

#include <sys/time.h>
#include <sys/types.h>
#include <bits/wordsize.h>
#include <stdint.h>

# define UT_LINESIZE 32
# define UT_NAMESIZE 32
# define UT_HOSTSIZE 256

/* Backwards compatibility hacks.  */
#define ut_name		ut_user
#ifndef _NO_UT_TIME
/* We have a problem here: `ut_time' is also used otherwise.  Define
   _NO_UT_TIME if the compiler complains.  */
# define ut_time	ut_tv.tv_sec
#endif
#define ut_xtime	ut_tv.tv_sec
#define ut_addr		ut_addr_v6[0]

/* Values for the `ut_type' field of a `struct utmp'.  */
#define EMPTY		0	/* No valid user accounting information.  */

#define RUN_LVL		1	/* The system's runlevel.  */
#define BOOT_TIME	2	/* Time of system boot.  */
#define NEW_TIME	3	/* Time after system clock changed.  */
#define OLD_TIME	4	/* Time when system clock changed.  */

#define INIT_PROCESS	5	/* Process spawned by the init process.  */
#define LOGIN_PROCESS	6	/* Session leader of a logged in user.  */
#define USER_PROCESS	7	/* Normal process.  */
#define DEAD_PROCESS	8	/* Terminated process.  */

#define ACCOUNTING	9

/* Old Linux name for the EMPTY type.  */
#define UT_UNKNOWN	EMPTY

/* The structure describing the status of a terminated process.  This
   type is used in `struct utmp' below.  */
struct exit_status
  {
    short int e_termination;	/* Process termination status.  */
    short int e_exit;		/* Process exit status.  */
  };

struct utmp
{
	short int ut_type;		/* Type of login.  */
	pid_t ut_pid;			/* Process ID of login process.  */
	char ut_line[UT_LINESIZE];
	char ut_id[4];		/* Inittab ID.  */
	char ut_user[UT_NAMESIZE];
	char ut_host[UT_HOSTSIZE];
	/* Hostname for remote login.  */
	struct exit_status ut_exit;	/* Exit status of a process marked
					   as DEAD_PROCESS.  */
/* The ut_session and ut_tv fields must be the same size when compiled
   32- and 64-bit.  This allows data files and shared memory to be
   shared between 32- and 64-bit applications.  */
	long int ut_session;		/* Session ID, used for windowing.  */
	struct timeval ut_tv;		/* Time entry was made.  */

	int32_t ut_addr_v6[4];	/* Internet address of remote host.  */
	char __glibc_reserved[20];		/* Reserved for future use.  */
};

#define USER_PROCESS	7	/* Normal process.  */

#endif /* __UTMP_H__ */
