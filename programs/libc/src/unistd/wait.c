#include "unistd.h"

extern pid_t user_wait(int *stat_loc);

pid_t wait(int *stat_loc) {
	return user_wait(stat_loc);
}
