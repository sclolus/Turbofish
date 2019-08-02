#include <signal.h>
#include <errno.h>

/* 
 * If pgrp is greater than 1, killpg(pgrp, sig) shall be equivalent to
 * kill(-pgrp, sig). If pgrp is less than or equal to 1, the behavior
 * of killpg() is undefined.
 */

int killpg(pid_t pgrp, int sig) {
	if (pgrp > 1) {
		return kill(-pgrp, sig);
	}
	else {
		errno = EINVAL;
		return -1;
	}
}
