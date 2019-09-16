KERNEL            := rust
TARGET            := "i686-turbofish"
PATH              := /toolchain_turbofish/cross/bin/:$(PATH)
SHELL             := env PATH=$(PATH) /bin/bash

# Turbofish root must be defined as environement variable
SYSTEM_ROOT       := $(TURBOFISH_ROOT)/system_disk

TOOLCHAIN_SYSROOT := /toolchain_turbofish/sysroot
LIBC_AR           := $(TOOLCHAIN_SYSROOT)/usr/lib/libc.a
LIBC_HEADERS      := $(TOOLCHAIN_SYSROOT)/usr/include $(TOOLCHAIN_SYSROOT)/usr/include/sys
