
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

static int f_comp(struct group *ref, void *other)
{
	TRACE
	return strcmp(ref->gr_name, (char *)other);
}

/*
 * get group file entry
 */
struct group *getgrnam(const char *name)
{
	TRACE
	return getgr_common((void *)name, &f_comp);
}
