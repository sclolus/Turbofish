for test in ${TESTS}; do \
	qemu-system-x86_64 --enable-kvm -cpu IvyBridge -m 64M -kernel ./integration_tests/"${test}" \
		-serial mon:stdio \
		-device isa-debug-exit,iobase=0xf4,iosize=0x04 \
		-display none
done
