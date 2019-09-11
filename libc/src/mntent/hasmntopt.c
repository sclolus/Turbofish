#include <mntent.h>
#include <stdio.h>
#include <string.h>

/* The hasmntopt() function scans the mnt_opts field (see below) of the  mntent  struc‐ */
/* ture  mnt  for  a substring that matches opt.  See <mntent.h> and mount(8) for valid */
/* mount options. */
char *hasmntopt(const struct mntent *mnt, const char *opt)
{
	return strstr(mnt->mnt_opts, opt);
}
