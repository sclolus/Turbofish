#ifndef __GRP_H__
# define __GRP_H__

#include <string.h>
#include <sys/types.h>

struct group {
	char   *gr_name;   // The name of the group.
	char   *gr_passwd; // group password
	gid_t   gr_gid;    // Numerical group ID.
	char  **gr_mem;    // Pointer to a null-terminated array of character pointers to member names.
};

// The <grp.h> header shall define the gid_t and size_t types as described in <sys/types.h>.

// The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.

struct group *getgrnam(const char *name);
struct group *getgrgid(gid_t gid);

void endgrent(void);
struct group *getgrent(void);
int getgrgid_r(gid_t, struct group *, char *, size_t, struct group **);
int getgrnam_r(const char *, struct group *, char *, size_t , struct group **);
void setgrent(void);

#endif
