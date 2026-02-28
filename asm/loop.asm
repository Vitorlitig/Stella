.SIZE 3

; We start by manually injecting energy into N1 via N0
MOV N0, 1.0      ; The "Trigger" neuron

; --- The Infinite Attractor Loop ---
; Cycle A: If N1 is active, move energy to N2 and kill N1
JMP N1, N2       ; N2 = N1, then N1 = 0

; Cycle B: If N2 is active, move energy back to N1 and kill N2
JMP N2, N1       ; N1 = N2, then N2 = 0
