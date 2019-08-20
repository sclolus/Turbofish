#ifndef __GRP_H__
# define __GRP_H__

#include <string.h>

struct group {
	char   *gr_name; // The name of the group. 
	gid_t   gr_gid ; // Numerical group ID. 
	char  **gr_mem ; // Pointer to a null-terminated array of character 
	//pointers to member names. 
};


//The <grp.h> header shall define the gid_t and size_t types as described in <sys/types.h>.

//The following shall be declared as functions and may also be defined as macros. Function prototypes shall be provided.

//[XSI][Option Start]
void endgrent(void);
struct group *getgrent(void);
//[Option End]
struct group *getgrgid(gid_t);
int getgrgid_r(gid_t, struct group *, char *,
                   size_t, struct group **);
struct group *getgrnam(const char *);
int getgrnam_r(const char *, struct group *, char *,
                   size_t , struct group **);
//[XSI][Option Start]
void setgrent(void);
//[Option End]

#endif
