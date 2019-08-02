[BITS 32]

segment .text

extern main
extern user_exit

global _start ; must be declared for linker (ld)
_start:       ; tell linker entry point
	push ebp
	mov ebp, esp

	call main

	push 0
	call user_exit


; --- STRACE DUMP ---
; execve("./Charly", ["./Charly"], [ /*39 vars */]) = 0
; strace:	 [ Process PID=2165 runs in 32 bit mode. ]
; write(1, "O\303\271 est Charlie ?\n\0", 19Où est Charlie ?
; ) = 19
; exit(0)                                 = ?
; +++ exited with 0 +++


; Personnages à retrouver

; Charlie, un jeune homme grand et mince aux cheveux bruns portant des lunettes de vue.
; Il est habillé avec un bonnet et un sweat shirt à rayures blanches et rouges,
; un pantalon bleu et des chaussures marron.

; Ouaf, le chien de Charlie (dont on ne voit que la queue)

; Blanchebarbe, un mage en rouge avec chapeau de magicien bleu et dans sa main un
; bâton rayé bleu, blanc et rouge.

; Pouah, la version maléfique de Charlie, aux cheveux noirs, portant un bonnet jaune et noir,
; des lunettes de vue aux verres foncés, une moustache très brune, un costume de prisonnier
; à rayures jaunes et noires, des chaussettes jaunes et des chaussures noires.

; Félicie, amie de Charlie, habillée avec une jupe d'un bleu plus clair que le pantalon de Charlie,
; des collants à rayures rouges et blanches et des chaussures d'un marron plus clair que celles de Charlie.

; Les fans de Charlie : d'innombrables enfants habillés en Charlie disséminés dans toutes les pages.
