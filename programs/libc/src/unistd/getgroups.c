#include <unistd.h>
#include <errno.h>

// The getgroups() function shall fill in the array grouplist with the
// current supplementary group IDs of the calling process. It is
// implementation-defined whether getgroups() also returns the
// effective group ID in the grouplist array.

// The gidsetsize argument specifies the number of elements in the
// array grouplist. The actual number of group IDs stored in the array
// shall be returned. The values of array entries with indices greater
// than or equal to the value returned are undefined.

// If gidsetsize is 0, getgroups() shall return the number of group
// IDs that it would otherwise return without modifying the array
// pointed to by grouplist.

// If the effective group ID of the process is returned with the
// supplementary group IDs, the value returned shall always be greater
// than or equal to one and less than or equal to the value of
// {NGROUPS_MAX}+1.

#warning NOT IMPLEMENTED

int getgroups(int gidsetsize, gid_t grouplist[])
{
	errno = ENOSYS;
	return -1;
}
