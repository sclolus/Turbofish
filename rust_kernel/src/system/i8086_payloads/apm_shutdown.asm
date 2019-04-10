[BITS 16]

; for APM shutdown, see https://wiki.osdev.org/APM
segment .text

GLOBAL payload_apm_shutdown
payload_apm_shutdown:
	; put stack on 0x84000
	mov ax, 0x8000
	mov ss, ax
	mov bp, 0x4000
	mov sp, bp

	; disconnect from any APM interface
	mov ah, 53h                 ; this is an APM command
	mov al, 04h                 ; interface disconnect command
	xor bx, bx                  ; device id (0 = APM BIOS)
	int 15h                     ; call the BIOS function through interrupt 15h
	jc .disconnect_error        ; if the carry flag is set see what the fuss is about.
	jmp .no_error

.disconnect_error:              ; the error code is in ah.
	cmp ah, 03h                 ; if the error code is anything but 03h there was an error.
	jne .APM_error              ; the error code 03h means that no interface was connected in the first place.

.no_error:
	; the function was successful
	; Nothing is returned.

	; connect to an APM interface
	mov ah, 53h                 ; this is an APM command
	mov al, 1                   ; see above description
	xor bx, bx                  ; device id (0 = APM BIOS)
	int 15h                     ; call the BIOS function through interrupt 15h
	jc .APM_error               ; if the carry flag is set there was an error
	; the function was successful
	; The return values are different for each interface.
	; The Real Mode Interface returns nothing.
	; See the official documentation for the
	; return values for the protected mode interfaces.

	; Enable power management for all devices
	mov ah, 53h                 ; this is an APM command
	mov al, 08h                 ; Change the state of power management...
	mov bx, 0001h               ; ...on all devices to...
	mov cx, 0001h               ; ...power management on.
	int 15h                     ; call the BIOS function through interrupt 15h
	jc .APM_error               ; if the carry flag is set there was an error

	; Set the power state for all devices
	mov ah, 53h                 ; this is an APM command
	mov al, 07h                 ; Set the power state...
	mov bx, 0001h               ; ...on all devices to...
	mov cx, 3                   ; see above
	int 15h                     ; call the BIOS function through interrupt 15h
	jc .APM_error               ; if the carry flag is set there was an error
	xor eax, eax
	jmp .end
.APM_error:
	mov eax, -1
.end:

segment .data

GLOBAL payload_apm_shutdown_len
payload_apm_shutdown_len: dd (payload_apm_shutdown.end - payload_apm_shutdown)
