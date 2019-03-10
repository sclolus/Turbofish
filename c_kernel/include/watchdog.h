
#ifndef __WATCHDOG_H__
# define __WATCHDOG_H__

enum dog {
	gdt,
	idt,
	idt_bios,
};

void	dog_guard(enum dog dog);
void	dog_bark(enum dog dog);

#endif
