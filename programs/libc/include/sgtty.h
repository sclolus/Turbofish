#ifndef SGTTY_H
# define SGTTY_H

//NON POSIX
struct sgttyb {
	int sg_flags;
	int sg_erase;
	int	sg_kill;
};

/* 
 * #define ECHOCTL 42
 * #define CRMOD 42
 * #define TIOCGETP 42
 * #define TIOCSETN 42
 * #define CBREAK 42
 * #define ECHO 42
 * #define ANYP 42
 */

#endif
