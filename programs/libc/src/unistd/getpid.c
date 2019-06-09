#include "signal.h"

extern pid_t user_getpid(void);

pid_t getpid(void) {
	return user_getpid();
}
