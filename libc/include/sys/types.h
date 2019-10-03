#ifndef _SYS_TYPES_H
# define _SYS_TYPES_H

typedef unsigned int blkcnt_t;
//    Used for file block counts.
typedef unsigned int blksize_t;
//    Used for block sizes.
#define clock_t int
//    Used for system times in clock ticks or CLOCKS_PER_SEC; see <time.h>.
#define clockid_t int
//    Used for clock ID type in the clock and timer functions.
typedef int dev_t;
//    Used for device IDs.
#define fsblkcnt_t unsigned int
//    Used for file system block counts.
#define fsfilcnt_t unsigned int
//    Used for file system file counts.
typedef unsigned short gid_t;
//    Used for group IDs.
#define id_t int
//    Used as a general identifier; can be used to contain at least a pid_t, uid_t, or gid_t.
typedef unsigned int ino_t;
//    Used for file serial numbers.
#define key_t int
//    [XSI] [Option Start] Used for XSI interprocess communication. [Option End]
typedef unsigned short mode_t;
//    Used for some file attributes.
typedef unsigned short nlink_t;
//    Used for link counts.
typedef long long off_t ;
//    Used for file sizes.
#define pid_t int
//    Used for process IDs and process group IDs.
#define pthread_attr_t int
//    Used to identify a thread attribute object.
#define pthread_barrier_t int
//    Used to identify a barrier.
#define pthread_barrierattr_t int
//    Used to define a barrier attributes object.
#define pthread_cond_t int
//    Used for condition variables.
#define pthread_condattr_t int
//    Used to identify a condition attribute object.
#define pthread_key_t int
//    Used for thread-specific data keys.
#define pthread_mutex_t int
//    Used for mutexes.
#define pthread_mutexattr_t int
//    Used to identify a mutex attribute object.
#define pthread_once_t int
//    Used for dynamic package initialization.
#define pthread_rwlock_t int
//    Used for read-write locks.
#define pthread_rwlockattr_t int
//    Used for read-write lock attributes.
#define pthread_spinlock_t int
//    Used to identify a spin lock.
#define pthread_t int
//    Used to identify a thread.
typedef long unsigned int size_t;
//    Used for sizes of objects.
typedef long int ssize_t;
//    Used for a count of bytes or an error indication.
typedef unsigned int suseconds_t;
//    Used for time in microseconds.
typedef signed int time_t;
//    Used for time in seconds.
#define timer_t int
//    Used for timer ID returned by timer_create().
#define trace_attr_t int
////    [OB TRC] [Option Start] Used to identify a trace stream attributes object [Option End]
//trace_event_id_t;
////    [OB TRC] [Option Start] Used to identify a trace event type. [Option End]
//trace_event_set_t;
////    [OB TEF] [Option Start] Used to identify a trace event type set. [Option End]
//trace_id_t;
////    [OB TRC] [Option Start] Used to identify a trace stream. [Option End]
typedef unsigned short uid_t;
//    Used for user IDs.

/* These were defined by ISO C without the first `_'.  */
typedef	unsigned char u_int8_t;
typedef	unsigned short int u_int16_t;
typedef	unsigned int u_int32_t;

#endif
