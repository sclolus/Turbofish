[BITS 32]

; Calling convention: Platform System V i386
; ------------------------------------------
; Return Value | Parameter Registers | Additional Parameters | Stack Alignment
; eax, edx       none                  stack (right to left)   none
;
; Scratch Registers | Preserved Registers       | Call List
; eax, ecx, edx       ebx, esi, edi, ebp, esp     ebp

segment .text

global _start ; must be declared for linker (ld)
_start:       ; tell linker entry point
	push ebp
	mov ebp, esp

	push dword len
	push dword msg
	push 1

	call user_write

	add esp, 12

	push 0
	call user_exit

user_write:
	push ebp
	mov ebp, esp

	push ebx

	mov edx, [ebp + 16]
	mov ecx, [ebp + 12]
	mov ebx, [ebp + 8]

	mov eax, 4 ; system call number (sys_write)
	int 80h

	pop ebx

	pop ebp
	ret

user_exit:
	push ebp
	mov ebp, esp

	mov ebx, [ebp + 8]

	mov eax, 1 ; system call number (sys_exit)
	int 80h

segment .data

msg db  'Les carottes sont cuites, je répète, les carottes sont cuites.', 0xa
len equ $ - msg


; --- STRACE DUMP ---
; execve("./Charles", ["./Charles"], [/* 39 vars */]) = 0
; strace: [ Process PID=11626 runs in 32 bit mode. ]
; write(1, "les carottes sont cuites !\n", 27les carottes sont cuites !
; ) = 27
; exit(0)                                 = ?
; +++ exited with 0 +++


; Étrangement, lorsqu’il n’y a plus aucun espoir, nous utilisons souvent des expressions inspirées des légumes.
; Ces expressions, qui n’ont a priori aucun sens, trouvent leur origine dans la vie domestique des siècles passés,
; souvent au sein de foyers très modestes.

; Dans la locution « les carottes sont cuites », quel est donc le rapport entre ce légume et l’espoir perdu ?
; Au XVIIe siècle, la carotte était un légume bon marché, souvent présenté comme un aliment du pauvre.
; Idéalement, on faisait cuire les carottes avec la viande, mais on les mangeait seules en temps de disette.

; On ne parlait pas alors de « vivre d’amour et d’eau fraîche » mais plutôt de ne « vivre que de carottes »,
; ce qui signifiait « vivre avec très peu de moyens ». Par la ressemblance phonétique et par la forme,
; on associait aussi la carotte à la « crotte ». Ainsi, les esprits bien tournés imaginèrent une expression
; pour désigner la constipation : « chier des carottes ».

; On comprend donc que la carotte était entourée d’un imaginaire assez péjoratif. Vers le XIXe siècle,
; une autre expression a fait référence au légume. Pour parler d’une personne mourante, on disait :
; « avoir ses carottes cuites ». La carotte en l’état ne signifie donc pas la fin, mais c’est sa
; cuisson qui contribue à donner toute sa portée à l’expression.

; Celle-ci est restée dans la mémoire collective puisqu’elle fut reprise durant la Seconde Guerre mondiale
; comme code utilisé à la radio depuis Londres. « Les carottes sont cuites, je répète, les carottes sont cuites »
; était le signal pour déclencher des opérations dans les territoires occupés.
