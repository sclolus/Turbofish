#include <sys/stat.h>

// The umask() function shall set the file mode creation mask of the
// process to cmask and return the previous value of the mask. Only
// the file permission bits of cmask (see <sys/stat.h>) are used; the
// meaning of the other bits is implementation-defined.

// The file mode creation mask of the process is used to turn off
// permission bits in the mode argument supplied during calls to the
// following functions:

//     open(), openat(), creat(), mkdir(), mkdirat(), mkfifo(), and
//     mkfifoat()

//     [XSI] [Option Start] mknod(), mknodat() [Option End]

//     [MSG] [Option Start] mq_open() [Option End]

//     sem_open()

// Bit positions that are set in cmask are cleared in the mode of the
// created file.

#warning NOT IMPLEMENTED

mode_t umask(mode_t cmask)
{
	return 42;
}
