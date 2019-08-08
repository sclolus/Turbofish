#include <termios.h>
#include <stdio.h>

void set_raw_mode(void) {
	struct termios termios_p;
	int ret = tcgetattr(0, &termios_p);
	if(ret == -1) {
		perror("tcgetattr failed");
	}

	termios_p.c_lflag &= (~(ICANON | ECHO | ISIG));
	ret = tcsetattr(0, TCSANOW, &termios_p);
	if( ret == -1) {
		perror("tcsetattr failed");
	}
}

void set_cooked_mode(void) {
	struct termios termios_p;
	int ret =  tcgetattr(0, &termios_p);
	if(ret == -1) {
		perror("tcgetattr failed");
	}

	termios_p.c_lflag |= (ICANON | ECHO | ISIG);
	ret = tcsetattr(0, TCSANOW, &termios_p);
	if( ret == -1) {
		perror("tcsetattr failed");
	}
}
