.SIZE 4
; Initialize our inputs
MOV N0, 1.0
MOV N1, 1.0

; Perform some logic
NAND N2, N0, N1  ; N2 = 1 NAND 1 (Should be 0.0)
NOT N3, N2       ; N3 = NOT 0    (Should be 1.0)
