.SIZE 4
; N0 = Condition (User Input)
; N1 = Entry Block (Starting Energy)
; N2 = Temporary Pulse Wire
; N3 = Permanent Final State (Latched)

; The JEQ will drain N1 and pulse N2 for exactly 1 cycle if N0 is 1.0
JEQ N0, N1, N2

; The LATCH will constantly monitor N2. If N2 ever fires, N3 turns on forever.
LATCH N3, N2
