#include <mntent.h>
#include <stdio.h>
#include <string.h>
#include <tools.h>
#include <stdlib.h>

static int32_t	parse_mnt_entry(char *line, struct mntent *entry)
{
	char	**fields = strsplit(line, ' ');

	if (!fields) {
		return -1;
	}
	size_t	nbr_fields = array_size((void **)fields);

	if (nbr_fields != 6) {
		free_array((void **)fields);
		return -1;
	}

	entry->mnt_fsname = fields[0];
	entry->mnt_dir = fields[1];
	entry->mnt_type = fields[2];
	entry->mnt_opts = fields[3];
	entry->mnt_freq = atoi(fields[4]);
	free(fields[4]);

	entry->mnt_passno = atoi(fields[5]);
	free(fields[5]);
	free(fields);
	return 0;
}

/* The getmntent() function reads the next line of the filesystem description file from */
/* stream and returns a pointer to a structure containing the broken out fields from  a */
/* line  in the file.  The pointer points to a static area of memory which is overwrit‐ */
/* ten by subsequent calls to getmntent(). */
struct mntent	 *getmntent(FILE *stream)
{
	static struct mntent	entry;
	char			*line = NULL;
	size_t			size = 0;

	int ret = ft_getline(&line, &size, stream);

	if (-1 == ret)
		return NULL;

	if (-1 == parse_mnt_entry(line, &entry)) {
		free(line);
		return NULL;
	}
	free(line);
	return &entry;
}

#ifdef UNIT_TESTS
# include <criterion/criterion.h>

Test(_parse_mnt_entry, basic_valid) {
	char		*entry = "sysfs /sys sysfs rw,nosuid,nodev,noexec,relatime 0 0";
	struct mntent	mntent;

	cr_assert(0 == parse_mnt_entry(entry, &mntent));
	cr_assert_str_eq(mntent.mnt_fsname, "sysfs");
	cr_assert_str_eq(mntent.mnt_dir, "/sys");
	cr_assert_str_eq(mntent.mnt_type, "sysfs");
	cr_assert_str_eq(mntent.mnt_opts, "rw,nosuid,nodev,noexec,relatime");
	cr_assert_eq(mntent.mnt_freq, 0);
	cr_assert_eq(mntent.mnt_passno, 0);

	free(mntent.mnt_fsname);
	free(mntent.mnt_dir);
	free(mntent.mnt_type);
	free(mntent.mnt_opts);
}

Test(_parse_mnt_entry, basic_invalid) {
	char		*entry = "sysfs /sys sysfs 0 0";
	struct mntent	mntent;

	cr_assert_eq(-1, parse_mnt_entry(entry, &mntent));

	entry = " 0 ";
	cr_assert_eq(-1, parse_mnt_entry(entry, &mntent));

	entry = "";
	cr_assert_eq(-1, parse_mnt_entry(entry, &mntent));

	entry = "sysfs /sys sysfs";
	cr_assert_eq(-1, parse_mnt_entry(entry, &mntent));
}

#endif
