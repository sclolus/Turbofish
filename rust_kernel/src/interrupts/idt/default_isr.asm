[BITS 32]

extern generic_handler
extern wrapper_generic_handler
extern _align_stack
extern _pic_time

;; This file contains the default Interrupt Service routine.

;; default ISR for all IDT entries
global _default_isr
_default_isr:
	iret

global _isr_timer_handler:
_isr_timer_handler:
	push eax
	lock inc dword [_pic_time]
	; send EOI master pic, irq0

	mov al, 0x20
	out 0x20, al
	pop eax
	ret


%macro CREATE_ISR 3
segment .text
global _isr_%1_%3
_isr_%1_%3:
	push ebp
	mov ebp, esp
	pushad
	push %3
	push 4
	push %2
	call _align_stack
	add esp, 12 ;pop interrupt string
	popad
	pop ebp
	iret
%endmacro

	CREATE_ISR generic_handler, generic_handler, 0
	CREATE_ISR generic_handler, generic_handler, 1
	CREATE_ISR generic_handler, generic_handler, 2
	CREATE_ISR generic_handler, generic_handler, 3
	CREATE_ISR generic_handler, generic_handler, 4
	CREATE_ISR generic_handler, generic_handler, 5
	CREATE_ISR generic_handler, generic_handler, 6
	CREATE_ISR generic_handler, generic_handler, 7
	CREATE_ISR generic_handler, generic_handler, 8
	CREATE_ISR generic_handler, generic_handler, 9
	CREATE_ISR generic_handler, generic_handler, 10
	CREATE_ISR generic_handler, generic_handler, 11
	CREATE_ISR generic_handler, generic_handler, 12
	CREATE_ISR generic_handler, generic_handler, 13
	CREATE_ISR generic_handler, generic_handler, 14
	CREATE_ISR generic_handler, generic_handler, 15
	CREATE_ISR generic_handler, generic_handler, 16
	CREATE_ISR generic_handler, generic_handler, 17
	CREATE_ISR generic_handler, generic_handler, 18
	CREATE_ISR generic_handler, generic_handler, 19
	CREATE_ISR generic_handler, generic_handler, 20
	CREATE_ISR generic_handler, generic_handler, 21
	CREATE_ISR generic_handler, generic_handler, 22
	CREATE_ISR generic_handler, generic_handler, 23
	CREATE_ISR generic_handler, generic_handler, 24
	CREATE_ISR generic_handler, generic_handler, 25
	CREATE_ISR generic_handler, generic_handler, 26
	CREATE_ISR generic_handler, generic_handler, 27
	CREATE_ISR generic_handler, generic_handler, 28
	CREATE_ISR generic_handler, generic_handler, 29
	CREATE_ISR generic_handler, generic_handler, 30
	CREATE_ISR generic_handler, generic_handler, 31
	CREATE_ISR generic_handler, generic_handler, 32
	CREATE_ISR generic_handler, generic_handler, 33
	CREATE_ISR generic_handler, generic_handler, 34
	CREATE_ISR generic_handler, generic_handler, 35
	CREATE_ISR generic_handler, generic_handler, 36
	CREATE_ISR generic_handler, generic_handler, 37
	CREATE_ISR generic_handler, generic_handler, 38
	CREATE_ISR generic_handler, generic_handler, 39
	CREATE_ISR generic_handler, generic_handler, 40
	CREATE_ISR generic_handler, generic_handler, 41
	CREATE_ISR generic_handler, generic_handler, 42
	CREATE_ISR generic_handler, generic_handler, 43
	CREATE_ISR generic_handler, generic_handler, 44
	CREATE_ISR generic_handler, generic_handler, 45
	CREATE_ISR generic_handler, generic_handler, 46
	CREATE_ISR generic_handler, generic_handler, 47
	CREATE_ISR generic_handler, generic_handler, 48
	CREATE_ISR generic_handler, generic_handler, 49
	CREATE_ISR generic_handler, generic_handler, 50
	CREATE_ISR generic_handler, generic_handler, 51
	CREATE_ISR generic_handler, generic_handler, 52
	CREATE_ISR generic_handler, generic_handler, 53
	CREATE_ISR generic_handler, generic_handler, 54
	CREATE_ISR generic_handler, generic_handler, 55
	CREATE_ISR generic_handler, generic_handler, 56
	CREATE_ISR generic_handler, generic_handler, 57
	CREATE_ISR generic_handler, generic_handler, 58
	CREATE_ISR generic_handler, generic_handler, 59
	CREATE_ISR generic_handler, generic_handler, 60
	CREATE_ISR generic_handler, generic_handler, 61
	CREATE_ISR generic_handler, generic_handler, 62
	CREATE_ISR generic_handler, generic_handler, 63
	CREATE_ISR generic_handler, generic_handler, 64
	CREATE_ISR generic_handler, generic_handler, 65
	CREATE_ISR generic_handler, generic_handler, 66
	CREATE_ISR generic_handler, generic_handler, 67
	CREATE_ISR generic_handler, generic_handler, 68
	CREATE_ISR generic_handler, generic_handler, 69
	CREATE_ISR generic_handler, generic_handler, 70
	CREATE_ISR generic_handler, generic_handler, 71
	CREATE_ISR generic_handler, generic_handler, 72
	CREATE_ISR generic_handler, generic_handler, 73
	CREATE_ISR generic_handler, generic_handler, 74
	CREATE_ISR generic_handler, generic_handler, 75
	CREATE_ISR generic_handler, generic_handler, 76
	CREATE_ISR generic_handler, generic_handler, 77
	CREATE_ISR generic_handler, generic_handler, 78
	CREATE_ISR generic_handler, generic_handler, 79
	CREATE_ISR generic_handler, generic_handler, 80
	CREATE_ISR generic_handler, generic_handler, 81
	CREATE_ISR generic_handler, generic_handler, 82
	CREATE_ISR generic_handler, generic_handler, 83
	CREATE_ISR generic_handler, generic_handler, 84
	CREATE_ISR generic_handler, generic_handler, 85
	CREATE_ISR generic_handler, generic_handler, 86
	CREATE_ISR generic_handler, generic_handler, 87
	CREATE_ISR generic_handler, generic_handler, 88
	CREATE_ISR generic_handler, generic_handler, 89
	CREATE_ISR generic_handler, generic_handler, 90
	CREATE_ISR generic_handler, generic_handler, 91
	CREATE_ISR generic_handler, generic_handler, 92
	CREATE_ISR generic_handler, generic_handler, 93
	CREATE_ISR generic_handler, generic_handler, 94
	CREATE_ISR generic_handler, generic_handler, 95
	CREATE_ISR generic_handler, generic_handler, 96
	CREATE_ISR generic_handler, generic_handler, 97
	CREATE_ISR generic_handler, generic_handler, 98
	CREATE_ISR generic_handler, generic_handler, 99
	CREATE_ISR generic_handler, generic_handler, 100
	CREATE_ISR generic_handler, generic_handler, 101
	CREATE_ISR generic_handler, generic_handler, 102
	CREATE_ISR generic_handler, generic_handler, 103
	CREATE_ISR generic_handler, generic_handler, 104
	CREATE_ISR generic_handler, generic_handler, 105
	CREATE_ISR generic_handler, generic_handler, 106
	CREATE_ISR generic_handler, generic_handler, 107
	CREATE_ISR generic_handler, generic_handler, 108
	CREATE_ISR generic_handler, generic_handler, 109
	CREATE_ISR generic_handler, generic_handler, 110
	CREATE_ISR generic_handler, generic_handler, 111
	CREATE_ISR generic_handler, generic_handler, 112
	CREATE_ISR generic_handler, generic_handler, 113
	CREATE_ISR generic_handler, generic_handler, 114
	CREATE_ISR generic_handler, generic_handler, 115
	CREATE_ISR generic_handler, generic_handler, 116
	CREATE_ISR generic_handler, generic_handler, 117
	CREATE_ISR generic_handler, generic_handler, 118
	CREATE_ISR generic_handler, generic_handler, 119
	CREATE_ISR generic_handler, generic_handler, 120
	CREATE_ISR generic_handler, generic_handler, 121
	CREATE_ISR generic_handler, generic_handler, 122
	CREATE_ISR generic_handler, generic_handler, 123
	CREATE_ISR generic_handler, generic_handler, 124
	CREATE_ISR generic_handler, generic_handler, 125
	CREATE_ISR generic_handler, generic_handler, 126
	CREATE_ISR generic_handler, generic_handler, 127
	CREATE_ISR generic_handler, generic_handler, 128
	CREATE_ISR generic_handler, generic_handler, 129
	CREATE_ISR generic_handler, generic_handler, 130
	CREATE_ISR generic_handler, generic_handler, 131
	CREATE_ISR generic_handler, generic_handler, 132
	CREATE_ISR generic_handler, generic_handler, 133
	CREATE_ISR generic_handler, generic_handler, 134
	CREATE_ISR generic_handler, generic_handler, 135
	CREATE_ISR generic_handler, generic_handler, 136
	CREATE_ISR generic_handler, generic_handler, 137
	CREATE_ISR generic_handler, generic_handler, 138
	CREATE_ISR generic_handler, generic_handler, 139
	CREATE_ISR generic_handler, generic_handler, 140
	CREATE_ISR generic_handler, generic_handler, 141
	CREATE_ISR generic_handler, generic_handler, 142
	CREATE_ISR generic_handler, generic_handler, 143
	CREATE_ISR generic_handler, generic_handler, 144
	CREATE_ISR generic_handler, generic_handler, 145
	CREATE_ISR generic_handler, generic_handler, 146
	CREATE_ISR generic_handler, generic_handler, 147
	CREATE_ISR generic_handler, generic_handler, 148
	CREATE_ISR generic_handler, generic_handler, 149
	CREATE_ISR generic_handler, generic_handler, 150
	CREATE_ISR generic_handler, generic_handler, 151
	CREATE_ISR generic_handler, generic_handler, 152
	CREATE_ISR generic_handler, generic_handler, 153
	CREATE_ISR generic_handler, generic_handler, 154
	CREATE_ISR generic_handler, generic_handler, 155
	CREATE_ISR generic_handler, generic_handler, 156
	CREATE_ISR generic_handler, generic_handler, 157
	CREATE_ISR generic_handler, generic_handler, 158
	CREATE_ISR generic_handler, generic_handler, 159
	CREATE_ISR generic_handler, generic_handler, 160
	CREATE_ISR generic_handler, generic_handler, 161
	CREATE_ISR generic_handler, generic_handler, 162
	CREATE_ISR generic_handler, generic_handler, 163
	CREATE_ISR generic_handler, generic_handler, 164
	CREATE_ISR generic_handler, generic_handler, 165
	CREATE_ISR generic_handler, generic_handler, 166
	CREATE_ISR generic_handler, generic_handler, 167
	CREATE_ISR generic_handler, generic_handler, 168
	CREATE_ISR generic_handler, generic_handler, 169
	CREATE_ISR generic_handler, generic_handler, 170
	CREATE_ISR generic_handler, generic_handler, 171
	CREATE_ISR generic_handler, generic_handler, 172
	CREATE_ISR generic_handler, generic_handler, 173
	CREATE_ISR generic_handler, generic_handler, 174
	CREATE_ISR generic_handler, generic_handler, 175
	CREATE_ISR generic_handler, generic_handler, 176
	CREATE_ISR generic_handler, generic_handler, 177
	CREATE_ISR generic_handler, generic_handler, 178
	CREATE_ISR generic_handler, generic_handler, 179
	CREATE_ISR generic_handler, generic_handler, 180
	CREATE_ISR generic_handler, generic_handler, 181
	CREATE_ISR generic_handler, generic_handler, 182
	CREATE_ISR generic_handler, generic_handler, 183
	CREATE_ISR generic_handler, generic_handler, 184
	CREATE_ISR generic_handler, generic_handler, 185
	CREATE_ISR generic_handler, generic_handler, 186
	CREATE_ISR generic_handler, generic_handler, 187
	CREATE_ISR generic_handler, generic_handler, 188
	CREATE_ISR generic_handler, generic_handler, 189
	CREATE_ISR generic_handler, generic_handler, 190
	CREATE_ISR generic_handler, generic_handler, 191
	CREATE_ISR generic_handler, generic_handler, 192
	CREATE_ISR generic_handler, generic_handler, 193
	CREATE_ISR generic_handler, generic_handler, 194
	CREATE_ISR generic_handler, generic_handler, 195
	CREATE_ISR generic_handler, generic_handler, 196
	CREATE_ISR generic_handler, generic_handler, 197
	CREATE_ISR generic_handler, generic_handler, 198
	CREATE_ISR generic_handler, generic_handler, 199
	CREATE_ISR generic_handler, generic_handler, 200
	CREATE_ISR generic_handler, generic_handler, 201
	CREATE_ISR generic_handler, generic_handler, 202
	CREATE_ISR generic_handler, generic_handler, 203
	CREATE_ISR generic_handler, generic_handler, 204
	CREATE_ISR generic_handler, generic_handler, 205
	CREATE_ISR generic_handler, generic_handler, 206
	CREATE_ISR generic_handler, generic_handler, 207
	CREATE_ISR generic_handler, generic_handler, 208
	CREATE_ISR generic_handler, generic_handler, 209
	CREATE_ISR generic_handler, generic_handler, 210
	CREATE_ISR generic_handler, generic_handler, 211
	CREATE_ISR generic_handler, generic_handler, 212
	CREATE_ISR generic_handler, generic_handler, 213
	CREATE_ISR generic_handler, generic_handler, 214
	CREATE_ISR generic_handler, generic_handler, 215
	CREATE_ISR generic_handler, generic_handler, 216
	CREATE_ISR generic_handler, generic_handler, 217
	CREATE_ISR generic_handler, generic_handler, 218
	CREATE_ISR generic_handler, generic_handler, 219
	CREATE_ISR generic_handler, generic_handler, 220
	CREATE_ISR generic_handler, generic_handler, 221
	CREATE_ISR generic_handler, generic_handler, 222
	CREATE_ISR generic_handler, generic_handler, 223
	CREATE_ISR generic_handler, generic_handler, 224
	CREATE_ISR generic_handler, generic_handler, 225
	CREATE_ISR generic_handler, generic_handler, 226
	CREATE_ISR generic_handler, generic_handler, 227
	CREATE_ISR generic_handler, generic_handler, 228
	CREATE_ISR generic_handler, generic_handler, 229
	CREATE_ISR generic_handler, generic_handler, 230
	CREATE_ISR generic_handler, generic_handler, 231
	CREATE_ISR generic_handler, generic_handler, 232
	CREATE_ISR generic_handler, generic_handler, 233
	CREATE_ISR generic_handler, generic_handler, 234
	CREATE_ISR generic_handler, generic_handler, 235
	CREATE_ISR generic_handler, generic_handler, 236
	CREATE_ISR generic_handler, generic_handler, 237
	CREATE_ISR generic_handler, generic_handler, 238
	CREATE_ISR generic_handler, generic_handler, 239
	CREATE_ISR generic_handler, generic_handler, 240
	CREATE_ISR generic_handler, generic_handler, 241
	CREATE_ISR generic_handler, generic_handler, 242
	CREATE_ISR generic_handler, generic_handler, 243
	CREATE_ISR generic_handler, generic_handler, 244
	CREATE_ISR generic_handler, generic_handler, 245
	CREATE_ISR generic_handler, generic_handler, 246
	CREATE_ISR generic_handler, generic_handler, 247
	CREATE_ISR generic_handler, generic_handler, 248
	CREATE_ISR generic_handler, generic_handler, 249
	CREATE_ISR generic_handler, generic_handler, 250
	CREATE_ISR generic_handler, generic_handler, 251
	CREATE_ISR generic_handler, generic_handler, 252
	CREATE_ISR generic_handler, generic_handler, 253
	CREATE_ISR generic_handler, generic_handler, 254
	CREATE_ISR generic_handler, generic_handler, 255
