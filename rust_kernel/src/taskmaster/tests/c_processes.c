
void dummy_c_process(void)
{
	char *msg = (char*)0x400100;
	msg[0] = 'D';
	msg[1] = 'u';
	msg[2] = 'm';
	msg[3] = 'm';
	msg[4] = 'y';
	msg[5] = ' ';
	msg[6] = 'C';
	// msg[7] = '4';
	msg[8] = '\n';

	while(1) {
		msg[7] = '4';
		asm("mov $0x4, %%eax; mov $0x1, %%ebx; mov %0, %%ecx; mov $9, %%edx; int $0x80" :: "m" (msg));
		msg[7] = '8';
		asm("mov $0x4, %%eax; mov $0x1, %%ebx; mov %0, %%ecx; mov $9, %%edx; int $0x80" :: "m" (msg));
	}
}
