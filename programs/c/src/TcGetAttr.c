#include <termios.h>
#include <unistd.h>
#include <stdio.h>

int main() {
	struct termios termios_p;

	tcgetattr(0, &termios_p);
	tcsetattr(0, TCSANOW, &termios_p);
}
