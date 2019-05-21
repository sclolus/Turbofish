
#include "minilibc.h"

const char c[] = "Où est Charlie ?\n";

int main(void)
{
	write(1, c, sizeof(c));
	return 0;
}
