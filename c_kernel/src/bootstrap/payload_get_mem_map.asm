[BITS 16]
segment .text
GLOBAL payload_get_mem_map
payload_get_mem_map:
	jmp $
	nop
	nop
	nop
	nop
.end:

[BITS 32]
segment .data
GLOBAL payload_get_mem_map_len
payload_get_mem_map_len: dd (payload_get_mem_map.end - payload_get_mem_map)
