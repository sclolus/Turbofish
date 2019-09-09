#include <ltrace.h>
#include <sys/stat.h>
#include <user_syscall.h>
#include <errno.h>

/// The chmod() function shall change S_ISUID, S_ISGID, [XSI] [Option
/// Start] S_ISVTX, [Option End] and the file permission bits of the
/// file named by the pathname pointed to by the path argument to the
/// corresponding bits in the mode argument. The application shall
/// ensure that the effective user ID of the process matches the owner
/// of the file or the process has appropriate privileges in order to
/// do this.
///
/// S_ISUID, S_ISGID, [XSI] [Option Start] S_ISVTX, [Option End] and
/// the file permission bits are described in <sys/stat.h>.
///
/// If the calling process does not have appropriate privileges, and
/// if the group ID of the file does not match the effective group ID
/// or one of the supplementary group IDs and if the file is a regular
/// file, bit S_ISGID (set-group-ID on execution) in the file's mode
/// shall be cleared upon successful return from chmod().
///
/// Additional implementation-defined restrictions may cause the
/// S_ISUID and S_ISGID bits in mode to be ignored.
///
/// Upon successful completion, chmod() shall mark for update the last
/// file status change timestamp of the file.
int chmod(const char *path, mode_t mode)
{
	TRACE
	int ret = _user_syscall(CHMOD, 2, path, mode);
	set_errno_and_return(ret);
}
