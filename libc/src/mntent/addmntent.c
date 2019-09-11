#include <mntent.h>
#include <stdio.h>

/* The addmntent() function adds the mntent structure  mnt  to  the  end  of  the  open */
/* stream. */

int addmntent(FILE *stream, const struct mntent *mnt)
{
	// buffer for integer conversion...
	char	buf[11];

	if (-1 == fseek(stream, 0, SEEK_END)) {
		// addmntent() returns 1 on failure...
		return 1;
	}

	if (-1 == fputs(mnt->mnt_fsname, stream)) {
		return 1;
	}

	if (-1 == fputs(" ", stream)) {
		return 1;
	}

	if (-1 == fputs(mnt->mnt_dir, stream)) {
		return 1;
	}

	if (-1 == fputs(" ", stream)) {
		return 1;
	}


	if (-1 == fputs(mnt->mnt_type, stream)) {
		return 1;
	}

	if (-1 == fputs(" ", stream)) {
		return 1;
	}


	if (-1 == fputs(mnt->mnt_opts, stream)) {
		return 1;
	}

	snprintf(buf, sizeof(buf), "%d", mnt->mnt_freq);

	if (-1 == fputs(buf, stream)) {
		return 1;
	}

	snprintf(buf, sizeof(buf), "%d", mnt->mnt_passno);

	if (-1 == fputs(buf, stream)) {
		return 1;
	}

	return 0;
}
