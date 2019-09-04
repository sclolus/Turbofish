#include <unistd.h>
#include <user_syscall.h>
#include <errno.h>

/// The chown() function shall change the user and group ownership of
/// a file.
///
/// The path argument points to a pathname naming a file. The user ID
/// and group ID of the named file shall be set to the numeric values
/// contained in owner and group, respectively.
///
/// Only processes with an effective user ID equal to the user ID of
/// the file or with appropriate privileges may change the ownership
/// of a file. If _POSIX_CHOWN_RESTRICTED is in effect for path:
///
///     Changing the user ID is restricted to processes with
///     appropriate privileges.
///
///     Changing the group ID is permitted to a process with an
///     effective user ID equal to the user ID of the file, but
///     without appropriate privileges, if and only if owner is equal
///     to the file's user ID or (uid_t)-1 and group is equal either
///     to the calling process' effective group ID or to one of its
///     supplementary group IDs.
///
/// If the specified file is a regular file, one or more of the
/// S_IXUSR, S_IXGRP, or S_IXOTH bits of the file mode are set, and
/// the process does not have appropriate privileges, the set-user-ID
/// (S_ISUID) and set-group-ID (S_ISGID) bits of the file mode shall
/// be cleared upon successful return from chown(). If the specified
/// file is a regular file, one or more of the S_IXUSR, S_IXGRP, or
/// S_IXOTH bits of the file mode are set, and the process has
/// appropriate privileges, it is implementation-defined whether the
/// set-user-ID and set-group-ID bits are altered. If the chown()
/// function is successfully invoked on a file that is not a regular
/// file and one or more of the S_IXUSR, S_IXGRP, or S_IXOTH bits of
/// the file mode are set, the set-user-ID and set-group-ID bits may
/// be cleared.
///
/// If owner or group is specified as (uid_t)-1 or (gid_t)-1,
/// respectively, the corresponding ID of the file shall not be
/// changed.
///
/// Upon successful completion, chown() shall mark for update the last
/// file status change timestamp of the file, except that if owner is
/// (uid_t)-1 and group is (gid_t)-1, the file status change timestamp
/// need not be marked for update.
int chown(const char *path, uid_t owner, gid_t group)
{
	int ret = _user_syscall(CHOWN, 3, path, owner, group);
	set_errno_and_return(ret);
}
