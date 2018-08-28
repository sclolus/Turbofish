
extern void scrollup(unsigned int);
extern void print(char *);

extern void asm_print(void);
extern void asm_print_2(char *);

extern kY;
extern kattr;

void _start(void)
{
	kY = 18;
	kattr = 0x5E;

	asm_print_2("Les sangliers sont partis\n");

	asm_print();

	print("un message\n");

	kattr = 0x4E;
	print("un autre message\n");


	kattr = 0x4E;
	print("et un dernier...\n");

	scrollup(2);

	while (1);
}
