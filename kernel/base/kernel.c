
#include "stdarg.h"
#include "../../kernel/cmd/cmd.h"
#include "../../kernel/graphic/graphic.h"
#include "../../kernel/system/system.h"
#include "../../kernel/system/vesa.h"

#define vesa_Info_Location  0x00032000
#define old_Cursor_Location 0x00032200


int main();

void _start(void)
{    


	query_old_cursor_position(old_Cursor_Location);
    setTextColor(3);    print("Execution of 32bits Kernel: ");
	
    setTextColor(2);    print("SUCCESS\n");
    


	setTextColor(3);    print("Query VESA informations: ");
	copy_Vesa_Info(vesa_Info_Location,&screen);
	setTextColor(2);    print("width %i heigth %i bits/pixels %i granularity %i Linear Frame Buffer at %x\n",screen.XResolution,screen.YResolution,screen.BitsPerPixel,screen.WinGranularity,screen.FlatMemory);

	setTextColor(3);    print("Creating new GDT for main Kernel: ");

	while (1);

	init_GDT(screen.FlatMemory);
    
    
    
	/* Initialisation du pointeur de pile %esp */

	asm("   movw $0x20, %ax     \n \
		  movw %ax, %ss       \n \
		  movl $0x20000, %esp  ");                    // La pile ne doit pas pouvoir corrompre le code, il faut faire attention là ou on la place.

	setTextColor(2); 	print("SUCCESS\n");
	setTextColor(3);	print("Initialize PIC (master and slave): ");
	init_PIC();
	setTextColor(2);    print("SUCCESS\n");
	setTextColor(3); 	print("Initialize IDT table: ");
	init_IDT();
	setTextColor(2);    print("SUCCESS\n");
    main();
}

void out(u16,u32);
u32 in(u16);

char* assign_pci_class(u32);
char* assign_pci_subclass(u32);

struct PCI_SUMMARY {
	u16 bus;
	u8  slot;	
	u8  function;
	u32 PCI_REG_VENDOR_ID;
	u32 PCI_REG_DEVICE_ID;
	char* PCI_CLASS;
	char* PCI_SUBCLASS;
};


struct PCI_SUMMARY pci[50];
struct PCI_SUMMARY* pci_ptr;


int main()
{
    va_list ap;
    
    (void)ap;
    
	setTextColor(15);   print("MAIN SYSTEM INITIALISED - INTERRUPT ENABLED\n");
	setTextColor(20);
	getCursorPosition(&cursor.X,&cursor.Y);
	setCursorPosition(0,36);
	print("   .-,        <,`-.__.-'>    \n");
	print("  / |          )       (     \n");
	print("  | ;         /_   ^4^  \\_   \n");
	print("  \\  \\.-~~-.__\\=_  -'- _/=)\n");
	print("   \\             `---;`      \n");
	print("   /     |           |__     \n");
	print("  /   /    ,    |   /  `~.   \n");
	print("  |    .'~..__|    /.' `'~,_)\n");
	print("  T  (`  | (  ;   |          \n");
	print("   \\  \\   '._)  \\  \\_      \n");
	print("    '._)         ',__)         \n");
	setCursorPosition(cursor.X,cursor.Y);
	setTextColor(62);

    	u16 bus;
	u8  slot;	
	u8  function;
	
	u32 val32;
	u32 output;
	
	u8 i;
	
	i=0;
	pci_ptr = &pci[i];
	
	for (bus=0; bus<0x100; bus++)
	{
		for (slot=0; slot<0x20; slot++)
		{
			for (function=0; function<0x8; function++)
			{
				val32 = 0x80000000 + (bus << 16) + (slot << 11) + (function << 8);
				out(0x0CF8,val32);
				output = in(0x0CFC);
				
				if (output == 0xFFFFFFFF) 
				{
					if (function == 0)		break;
					else					continue;
				}
				pci_ptr->PCI_REG_VENDOR_ID = output;
				pci_ptr->bus 	= bus;
				pci_ptr->slot	= slot;
				pci_ptr->function	= function;
				
				val32 += 0x9;
				out(0x0CF8,val32);
				//pci_ptr->PCI_REG_DEVICE_ID = in(0x0CFC) >> 16;
				pci_ptr->PCI_REG_DEVICE_ID = in(0x0CFC);
				
				//pci_ptr->PCI_CLASS = assign_pci_class(pci_ptr->PCI_REG_DEVICE_ID >> 28);
				//pci_ptr->PCI_SUBCLASS = assign_pci_subclass(pci_ptr->PCI_REG_DEVICE_ID >> 16);
				
				//print("PCI Bus %i slot %i function %i vendor-id 0x%h device-id 0x%x Class:%s SubClass:%s\n",pci_ptr->bus,pci_ptr->slot,pci_ptr->function,pci_ptr->PCI_REG_VENDOR_ID,pci_ptr->PCI_REG_DEVICE_ID,pci_ptr->PCI_CLASS,pci_ptr->PCI_SUBCLASS);
				print("PCI Bus %i slot %i function %i vendor-id 0x%x device-id 0x%x\n",pci_ptr->bus,pci_ptr->slot,pci_ptr->function,pci_ptr->PCI_REG_VENDOR_ID,pci_ptr->PCI_REG_DEVICE_ID);

				i++; 
				pci_ptr = &pci[i];
	}}}
    
/*
;val32 =
;      0x80000000
;    | bus << 16
;    | slot << 11
;    | function << 8
;    | register << 2
;
;
;                         |  |      PCI BUS       |  |                  32 bits
;                    __________________________________________
;                   | |||| |||| |||| |||| |||| |||| |||| |||| |
;                     |||| |||| |||| |||| |||| |||| |||| ||||
; 0x8000        -> 0x 1000 0000 ---- ---- ---- ---- ---- ----
; bus << 16     -> 0x ---- ---- BBBB BBBB ---- ---- ---- ----   -> 256 values   0x00 -> 0xFF
; slot << 11    -> 0x ---- ---- ---- ---- BBBB B--- ---- ----   ->  32 values   0x00 -> 0x20
; function << 8 -> 0x ---- ---- ---- ---- ---- -BBB ---- ----   ->   7 values   0x00 -> 0x08
; register << 2 -> 0x ---- ---- ---- ---- ---- ---- BBBB BB--   ->  64 values   0x00 -> 0x40
*/


	
	asm(" sti ");
	iddle_mode();
    return(0);
}
