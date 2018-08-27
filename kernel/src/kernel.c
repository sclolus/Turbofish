
extern void scrollup(unsigned int);
extern void print(char *);

extern kY;
extern kattr;

void _start(void)
{
	kY = 18;
	kattr = 0x5E;
	print("un message\n");

	kattr = 0x4E;
	print("un autre message\n");


	kattr = 0x4E;
	print("et un dernier...\n");

	scrollup(2);

	while (1);
}
