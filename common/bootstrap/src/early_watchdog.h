
#ifndef __EARLY_WATCHDOG_H__
# define __EARLY_WATCHDOG_H__

#define LEN 3

// Early GDT
#define GDT_AREA 0x800
#define GDT_SIZE 0x400

// Early IDT
#define IDT_AREA 0x1000
#define IDT_SIZE 0x800

// IVT bios
#define IVT_AREA 0x0
#define IVT_SIZE 0x400

enum dog {
	gdt,
	idt,
	ivt,
};

void	dog_guard(enum dog dog);
void	dog_bark(enum dog dog);

void	guard_all(void);
void	check_all(void);

#endif
