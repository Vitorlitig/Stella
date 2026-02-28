.SIZE 13

; --- 1. Constants ---
MOV N0, 1.0      ; Drives 1.0 continuously
MOV N1, 0.0      ; Drives 0.0 continuously

; --- 2. Arithmetic ---
ADD N2, N0, N1   ; N2 = 1.0 + 0.0 = 1.0
SUB N3, N0, N1   ; N3 = 1.0 - 0.0 = 1.0

; --- 3. Logic Gates ---
AND N4, N0, N0   ; N4 = 1.0 AND 1.0 = 1.0
OR  N5, N0, N1   ; N5 = 1.0 OR 0.0  = 1.0
NAND N6, N0, N0  ; N6 = 1.0 NAND 1.0 = 0.0
NOT N7, N6       ; N7 = NOT 0.0     = 1.0

; --- 4. Control Flow (The Energy Bouncer) ---
; We will manually inject 1.0 into N8 via the CLI input.

JMP N8, N9       ; Cycle 1: Energy unconditional jump from N8 -> N9
JEQ N7, N9, N10  ; Cycle 2: If N7 is 1.0 (it is!), pulse energy N9 -> N10

; --- 5. Memory (The Traps) ---
LATCH N11, N10   ; Traps the fleeting 1-cycle pulse from N10 forever
LATCH N12, N8    ; Traps the initial energy from N8 before it jumps away
