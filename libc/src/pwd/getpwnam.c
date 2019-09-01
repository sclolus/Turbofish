
#include <pwd.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>
#include <tools.h>

struct passwd *getpw_common(void *cmp, int f_comp(struct passwd *ref, void *cmp));
int f_get_passwd(char **fields, void *_pentry);
void print_passwd(struct passwd *pentry);
void free_passwd(struct passwd *pentry);

static int f_comp(struct passwd *ref, void *other)
{
	return strcmp((char *)other, ref->pw_name);
}

/*
 * get password file entry
 */
struct passwd *getpwnam(const char *name)
{
	return getpw_common((void *)name, &f_comp);
}
