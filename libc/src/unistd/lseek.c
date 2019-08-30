#include <unistd.h>
#include <errno.h>
#include <user_syscall.h>

/// The lseek() function shall set the file offset for the open file
/// description associated with the file descriptor fildes, as
/// follows:
///     If whence is SEEK_SET, the file offset shall be set to offset
///     bytes.
///     If whence is SEEK_CUR, the file offset shall be set to its
///     current location plus offset.
///     If whence is SEEK_END, the file offset shall be set to the
///     size of the file plus offset.
/// The symbolic constants SEEK_SET, SEEK_CUR, and SEEK_END are
/// defined in <unistd.h>.
/// The behavior of lseek() on devices which are incapable of seeking
/// is implementation-defined. The value of the file offset associated
/// with such a device is undefined.
/// The lseek() function shall allow the file offset to be set beyond
/// the end of the existing data in the file. If data is later written
/// at this point, subsequent reads of data in the gap shall return
/// bytes with the value 0 until data is actually written into the
/// gap.
///
/// The lseek() function shall not, by itself, extend the size of a
/// file.
/// Upon successful completion, the resulting offset, as measured in
/// bytes from the beginning of the file, shall be
/// returned. Otherwise, -1 shall be returned, errno shall be set to
/// indicate the error, and the file offset shall remain unchanged.
///ORS
/// [EBADF]
///     The fildes argument is not an open file descriptor.
/// [EINVAL]
///     The whence argument is not a proper value, or the resulting
///     file offset would be negative for a regular file, block
///     special file, or directory.
/// [EOVERFLOW]
///     The resulting file offset would be a value which cannot be
///     represented correctly in an object of type off_t.
/// [ESPIPE]
///     The fildes argument is associated with a pipe, FIFO, or
///     socket.

#include <custom.h>
#warning NOT COMPLETLY IMPLEMENTED

off_t lseek(int fildes, off_t offset, int whence)
{
	DUMMY
	off_t ret;
	// 4 argument since off_t is a 8 bytes type
	_user_syscall(LSEEK, 5, &ret, fildes, offset, whence);
	set_errno_and_return(ret);
}
