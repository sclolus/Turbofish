#include "unistd.h"

extern int user_exit(int status);

void exit(int status)
{
	user_exit(status);
}
