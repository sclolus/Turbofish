# export TURBOFISH_ROOT in you shell configuration file to execute this Makefile independently
include $(TURBOFISH_ROOT)/boilerplates.mk

DASH = dash-0.5.10

.PHONY: all clean fclean re

TARGET=$(SYSTEM_ROOT)/bin/dash

all: $(TARGET)

$(TARGET): $(LIBC_AR) $(LIBC_HEADERS)
	rm -rf $(DASH)
	wget -c 'http://gondor.apana.org.au/~herbert/dash/files/$(DASH).tar.gz'
	tar -xf '$(DASH).tar.gz'
	./tiny.sh
	cp -v $(DASH)/build/src/dash $@

clean:
	rm -rvf $(DASH)
	rm $(TARGET)

fclean: clean

re: fclean all
