#ifndef _SYS_TYPES_H
# define _SYS_TYPES_H

#define blkcnt_t int
//    Used for file block counts.
#define blksize_t int
//    Used for block sizes.
#define clock_t int
//    Used for system times in clock ticks or CLOCKS_PER_SEC; see <time.h>.
#define clockid_t int
//    Used for clock ID type in the clock and timer functions.
#define dev_t int
//    Used for device IDs.
#define fsblkcnt_t unsigned int
//    Used for file system block counts.
#define fsfilcnt_t unsigned int
//    Used for file system file counts.
typedef unsigned int gid_t;
//    Used for group IDs.
#define id_t int
//    Used as a general identifier; can be used to contain at least a pid_t, uid_t, or gid_t.
#define ino_t unsigned int
//    Used for file serial numbers.
#define key_t int
//    [XSI] [Option Start] Used for XSI interprocess communication. [Option End]
#define mode_t int
//    Used for some file attributes.
#define nlink_t int
//    Used for link counts.
#define off_t unsigned int
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
#define suseconds_t int
//    Used for time in microseconds.
#define time_t int
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
typedef unsigned int uid_t;
//    Used for user IDs.

#endif
