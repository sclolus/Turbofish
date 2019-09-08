
#include <ltrace.h>
#include <grp.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <tools.h>

struct group *getgr_common(void *cmp, int fn(struct group *ref, void *cmp));
int f_get_group(char **fields, void *_group);
void print_group(struct group *group);
void free_group(struct group *group);

struct s_gid {
	uid_t gid;
};

static int f_comp(struct group *ref, void *other)
{
	TRACE
	gid_t gid  = ((struct s_gid *)other)->gid;
	if (gid == ref->gr_gid) {
		return 0;
	} else {
		return 1;
	}
}

/*
 * get group file entry
 */
struct group *getgrgid(gid_t gid)
{
	TRACE
	struct s_gid s_gid;

	s_gid.gid = gid;
	return getgr_common((void *)&s_gid, &f_comp);
}

void print_group(struct group *group)
{
	TRACE
	printf("%s:%s:%u\n", group->gr_name,
	       group->gr_passwd,
	       group->gr_gid);
	int i = 0;
	while (group->gr_mem[i] != NULL) {
		printf("%s ", group->gr_mem[i]);
		i++;
	}
	printf("\n");

}

void free_group(struct group *group)
{
	TRACE
	free(group->gr_name);
	free(group->gr_passwd);
	free_array((void **)group->gr_mem);
}

struct group *getgr_common(void *cmp, int f_comp(struct group *ref, void *cmp))
{
	TRACE
	static struct group static_area = {0};

	struct group **a = (struct group **)parse_2d_file("/etc/group", '\n', ':', sizeof(struct group), &f_get_group);
	if (a == NULL) {
		return NULL;
	}

	bool founded = false;
	for (int i = 0; a[i] != NULL; i++) {
		print_group(a[i]);
		if (f_comp(a[i], cmp) == 0) {
			if (static_area.gr_name != NULL) {
				free_group(&static_area);
			}
			/* Store content into the static area: Do not pass the returned pointer to free */
			memcpy(&static_area, a[i], sizeof(struct group));
			founded = true;
		} else {
			free_group(a[i]);
		}
		free(a[i]);
	}
	free(a);
	/*
	 * The getgrnam() and getgrgid() functions return a pointer to a group structure, or NULL
	 * if the matching entry is not found or an error occurs.  If an error occurs, errno is set
	 * appropriately.  If one wants to check errno after the call, it should be set to zero before the call.
	 */
	return (founded) ? &static_area : NULL;
}

int f_get_group(char **fields, void *_group)
{
	TRACE
	struct group *group = _group;

	uint32_t n_fields = array_size((void **)fields);
	if (n_fields != ENTRY_NB_FIELDS) {
		dprintf(2, "File Bad Formated\n");
		free_array((void **)fields);
		return -1;
	}
	group->gr_name = fields[0];
	group->gr_passwd = fields[1];
	group->gr_gid = atoi(fields[2]);
	group->gr_mem = strsplit(fields[3], ',');
	free(fields[2]);
	free(fields[3]);
	return 0;
}
