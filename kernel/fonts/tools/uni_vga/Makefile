#
# Makefile
#     Contains commands to make specific-encoded screen fonts from
#     an iso10646-1-encoded bdf font.
#
# Usage:
#     make <encoding>-vga-8x16.psf
#
# Notes:
#     Requires GNU make (due to use of pattern rule)
#

BDF2PSF=	./bdf2psf.pl
FACE=		vga
SIZE=		16

%-$(FACE)-8x$(SIZE).psf:	u_$(FACE)$(SIZE).bdf %.sfm
		$(BDF2PSF) -s $*.sfm $< | psfaddtable -c $*.sfm -o $@ -

.PHONY:		default allfonts

default:
		@echo "Usage:"
		@echo "    make <encoding>-vga-8x16.bdf"
		@echo "where <encoding> is one of"
		@echo "    "`echo *.sfm|sed -e 's/.sfm//g'`

allfonts:	$(patsubst %.sfm, %-$(FACE)-8x$(SIZE).psf, $(wildcard *.sfm))
