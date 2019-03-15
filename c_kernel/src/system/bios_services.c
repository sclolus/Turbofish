
#include "system.h"

/*
 * XXX This shutdown method is only valid for BIOS with APM
 */
void	bios_shutdown_computer(void)
{
	struct base_registers reg;

	reg.eax = 0x5301;
	reg.ebx = 0x0;
	_int8086(&reg, 0x15);

	/* Try to set apm version (to 1.2). */
	reg.eax = 0x530E;
	reg.ebx = 0;
	reg.ecx = 0x102;
	_int8086(&reg, 0x15);

	/* Turn off the system. */
	reg.eax = 0x5307;
	reg.ebx = 0x1;
	reg.ecx = 0x3;
	_int8086(&reg, 0x15);
}

void	bios_wait(u32 sec)
{
	struct base_registers reg;

	reg.eax = 0x8600;
	reg.ecx = sec * 10;
	reg.edx = 0;
	_int8086(&reg, 0x15);
}
