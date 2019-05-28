#include "unistd.h"

extern int user_fork();

int fork()
{
	return user_fork();
}
