
#include <pwd.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <tools.h>

struct passwd *getpw_common(void *cmp, int fn(struct passwd *ref, void *cmp));
int f_get_passwd(char **fields, void *_pentry);
void print_passwd(struct passwd *pentry);
void free_passwd(struct passwd *pentry);

static int f_comp(struct passwd *ref, void *other)
{
	uid_t uid = (uid_t)other;
	if (uid == ref->pw_uid) {
		return 0;
	} else {
		return 1;
	}
}

/*
 * get password file entry
 */
struct passwd *getpwuid(uid_t uid)
{
	return getpw_common((void *)uid, &f_comp);
}

struct passwd *getpw_common(void *cmp, int f_comp(struct passwd *ref, void *cmp))
{
	static struct passwd static_area = {0};

	struct passwd **a = (struct passwd **)parse_2d_file("/etc/passwd", '\n', ':', sizeof(struct passwd), &f_get_passwd);
	if (a == NULL) {
		return NULL;
	}

	bool founded = false;
	for (int i = 0; a[i] != NULL; i++) {
		print_passwd(a[i]);
		if (f_comp(a[i], cmp) == 0) {
			if (static_area.pw_name != NULL) {
				free_passwd(&static_area);
			}
			/* Store content into the static area: Do not pass the returned pointer to free */
			memcpy(&static_area, a[i], sizeof(struct passwd));
			founded = true;
		} else {
			free_passwd(a[i]);
		}
		free(a[i]);
	}
	free(a);
	/*
	 * The getpwnam() and getpwuid() functions return a pointer to a passwd structure,
	 * or NULL if the matching entry is not found or an error occurs. If an error occurs,
	 * errno is set appropriately. If one wants to check errno after the call, it should
	 * be set to zero before the call.
	 */
	return (founded) ? &static_area : NULL;
}

int f_get_passwd(char **fields, void *_pentry)
{
	struct passwd *pentry = _pentry;

	uint32_t n_fields = array_size((void **)fields);
	if (n_fields != ENTRY_NB_FIELDS) {
		dprintf(2, "File Bad Formated\n");
		free_array((void **)fields);
		return -1;
	}
	pentry->pw_name = fields[0];
	pentry->pw_passwd = fields[1];
	pentry->pw_uid = atoi(fields[2]);
	pentry->pw_gid = atoi(fields[3]);
	pentry->pw_dir = fields[4];
	pentry->pw_gecos = fields[5];
	pentry->pw_shell = fields[6];
	free(fields[2]);
	free(fields[3]);
	return 0;
}

void free_passwd(struct passwd *pentry)
{
	free(pentry->pw_name);
	free(pentry->pw_passwd);
	free(pentry->pw_dir);
	free(pentry->pw_gecos);
	free(pentry->pw_shell);
}

void print_passwd(struct passwd *pentry)
{
	char	*pw_name = pentry->pw_name ?: "";
	char	*pw_passwd = pentry->pw_passwd ?: "";
	char	*pw_dir = pentry->pw_dir ?: "";
	char	*pw_gecos = pentry->pw_gecos ?: "";
	char	*pw_shell = pentry->pw_shell ?: "";
	printf("%s:%s:%u:%u:%s:%s:%s\n", pw_name,
	       pw_passwd,
	       pentry->pw_uid,
	       pentry->pw_gid,
	       pw_dir,
	       pw_gecos,
	       pw_shell);
}
