#ifndef __MMAN_H__
# define __MMAN_H__

#include <stddef.h>

#define MAP_FAILED 0xFFFFFFFF

//    The <sys/mman.h> header shall define the following symbolic constants for use as protection options:
#define PROT_NONE 0
//    Page cannot be accessed.
#define PROT_READ (1 << 0)
//   Page can be read.
#define PROT_WRITE (1 << 1)
//   Page can be written.
#define PROT_EXEC (1 << 2)
//   Page can be executed.

//The <sys/mman.h> header shall define the following symbolic constants for use as flag options:

#define MAP_SHARED (1 << 0)
//Share changes.
#define MAP_PRIVATE (1 << 1)
//Changes are private.
#define MAP_FIXED (1 << 4)
//Interpret addr exactly.
#define MAP_ANONYMOUS (1 << 5)
#define MAP_ANON MAP_ANONYMOUS

//[XSI|SIO] [Option Start] The <sys/mman.h> header shall define the following symbolic constants for the msync() function:
//
//MS_ASYNC
//    Perform asynchronous writes.
//MS_INVALIDATE
//    Invalidate mappings.
//MS_SYNC
//    Perform synchronous writes.

//[Option End]
//
//[ML] [Option Start] The <sys/mman.h> header shall define the following symbolic constants for the mlockall() function:
//
//MCL_CURRENT
//    Lock currently mapped pages.
//MCL_FUTURE
//    Lock pages that become mapped.
//
//[Option End]
//
//The <sys/mman.h> header shall define the symbolic constant MAP_FAILED which shall have type void * and shall be used to indicate a failure from the mmap() function .
//
//[ADV] [Option Start] If the Advisory Information option is supported, the <sys/mman.h> header shall define symbolic constants for the advice argument to the posix_madvise() function as follows:
//
//POSIX_MADV_DONTNEED
//    The application expects that it will not access the specified range in the near future.
//POSIX_MADV_NORMAL
//    The application has no advice to give on its behavior with respect to the specified range. It is the default characteristic if no advice is given for a range of memory.
//POSIX_MADV_RANDOM
//    The application expects to access the specified range in a random order.
//POSIX_MADV_SEQUENTIAL
//    The application expects to access the specified range sequentially from lower addresses to higher addresses.
//POSIX_MADV_WILLNEED
//    The application expects to access the specified range in the near future.
//
//[Option End]
//
//[TYM] [Option Start] The <sys/mman.h> header shall define the following symbolic constants for use as flags for the posix_typed_mem_open() function:
//
//POSIX_TYPED_MEM_ALLOCATE
//    Allocate on mmap().
//POSIX_TYPED_MEM_ALLOCATE_CONTIG
//    Allocate contiguously on mmap().
//POSIX_TYPED_MEM_MAP_ALLOCATABLE
//    Map on mmap(), without affecting allocatability.
//
//[Option End]
//
//The <sys/mman.h> header shall define the mode_t, off_t, and size_t types as described in <sys/types.h>.

#include <sys/types.h>
//
//[TYM] [Option Start] The <sys/mman.h> header shall define the posix_typed_mem_info structure, which shall include at least the following member:
//size_t  posix_tmi_length  Maximum length which may be allocated 
//                          from a typed memory object. 
struct posix_typed_mem_info {
	size_t  posix_tmi_length;  //Maximum length which may be allocated from a typed memory object. 
};
//
//
//[Option End]
//
//The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.

//[MLR][Option Start]
int mlock(const void *, size_t);
//[Option End]
//[ML][Option Start]
int mlockall(int);
//[Option End]
void *mmap(void *, size_t, int, int, int, off_t);
int mprotect(void *, size_t, int);
//[XSI|SIO][Option Start]
int msync(void *, size_t, int);
//[Option End]
//[MLR][Option Start]
int munlock(const void *, size_t);
//[Option End]
//[ML][Option Start]
int munlockall(void);
//[Option End]
int munmap(void *, size_t);
//[ADV][Option Start]
int posix_madvise(void *, size_t, int);
//[Option End]
//[TYM][Option Start]
int posix_mem_offset(const void *restrict, size_t, off_t *restrict,
           size_t *restrict, int *restrict);
int posix_typed_mem_get_info(int, struct posix_typed_mem_info *);
int posix_typed_mem_open(const char *, int, int);
//[Option End]
//[SHM][Option Start]
int shm_open(const char *, int, mode_t);
int shm_unlink(const char *);
//[Option End]

#endif
