# 🌟 Project Stella: The Continuous-State Neural VM

**Objective:** To build a Virtual Machine protector in Rust that translates discrete assembly logic (`MOV`, `ADD`, `CMP`, `JMP`) into a continuous **Fixed-Point Matrix Multiplication** loop.

**The "Unhinged" Theory:**
Standard CPU: `NextState = Logic(CurrentState)`
Stella CPU: `NextState = Activation(WeightMatrix × CurrentState + Bias)`

We are not using Python/PyTorch. We are building a mathematical engine in pure Rust that *simulates* logic gates using linear algebra.

---

## 🛠 Tech Stack & Tools
*   **Language:** Rust (Strict `no_std` compatible core).
*   **Math:** Custom Fixed-Point Arithmetic (I64 representing Q32.32 format). *We avoid `f64` floats to guarantee bit-perfect determinism across architectures.*
*   **Parallelism:** `Rayon` (Matrix multiplication is trivially parallelizable).
*   **Serialization:** `Bincode` (To store the massive weight matrices).

---

## 📅 The Phases (Strict TDD)

### Phase 1: The Synaptic Kernel (The Math)
We need a bedrock of deterministic linear algebra. If $1 + 1$ equals $2.0000001$, the VM crashes.

*   **Goal:** Build a fixed-point Matrix engine.
*   **TDD Requirements:**
    *   `test_fixed_point_precision()`: Verify $1.0 + 2.0 == 3.0$ exactly.
    *   `test_matrix_multiplication()`: Verify $(A \times B) \times C == A \times (B \times C)$.
    *   `test_activation_function()`: Implement a "Hard Tanh" or "ReLU6" that clamps values cleanly to simulate binary logic states ($0$ or $1$).

### Phase 2: The "Gate" Compiler (Logic Synthesis)
We need to compose standard assembly instructions out of "Neurons."

*   **Concept:** A single instruction (like `ADD`) is a small pre-calculated Weight Matrix.
*   **Goal:** Implement the "NAND" gate using matrices. If you have NAND, you have everything.
*   **TDD Requirements:**
    *   `test_neuron_and_gate()`: Inputs $(1, 1) \rightarrow 1$, $(1, 0) \rightarrow 0$.
    *   `test_neuron_xor_gate()`: The classic ML problem. Needs a hidden layer (intermediate state) to solve.
    *   `test_register_storage()`: Can a neuron "hold" a value of `42` for 100 cycles without decaying? (Identity Matrix).

### Phase 3: The Graph Assembly (Control Flow)
This is the mind-bending part. How do we `JUMP` without a `JMP` instruction?
*   **Concept:** **Attractor Dynamics.**
    *   The "Instruction Pointer" is not an integer index. It is a "Context Vector" (a pattern of active neurons).
    *   Each Basic Block (function) has a specific "Signature."
    *   To jump from Block A to Block B, the matrix transforms the Context Vector from Pattern A to Pattern B over time.
*   **TDD Requirements:**
    *   `test_conditional_branching()`: Input `0` stays in Block A. Input `1` morphs state to Block B.
    *   `test_infinite_loop()`: A matrix configuration that oscillates the state vector stably forever.

### Phase 4: The Obfuscator (Basis Transformation)
Right now, the matrices are "sparse" (mostly zeros, easy to read). We need to explode them.
*   **Concept:** $W' = P^{-1} W P$.
    *   We generate a random invertible matrix $P$.
    *   We multiply our logic matrix by $P$.
    *   Now, "Register A" is no longer `Vector[0]`. It is spread across `Vector[0]...Vector[N]` as a linear combination.
*   **TDD Requirements:**
    *   `test_isomorphism()`: Run the VM with clean matrices. Run it with scrambled matrices. The Output (observed result) must be identical, but the internal state must be noise.

### Phase 5: The "Brain" Runtime
The final executable that loads the `.stella` file (the weights) and runs the inference loop.

---

## 🧠 Detailed Architecture

### The State Vector ($S$)
Instead of `RAX, RBX, RCX`, our machine state is a vector of 1024 Fixed-Point numbers.
$$S = [n_0, n_1, ..., n_{1023}]$$

*   $n_{0..15}$: **I/O Neurons** (The "Public" API).
*   $n_{16..1023}$: **Hidden State** (The obfuscated logic).

### The Execution Cycle
Unlike a CPU that fetches different instructions, Stella runs **the same equation** forever:

1.  **Input:** Load user input into $S_{0..15}$.
2.  **Propagate:** $S_{new} = \text{Clamp}(W \times S_{old} + B)$
3.  **Repeat:** $S_{old} \leftarrow S_{new}$

There is no "Fetch-Decode-Execute." There is only "Propagate." The specific "Instruction" being executed is encoded *inside* the state vector itself (The Context).

---

## 📝 Implementation Plan (Code Structure)

```text
stella_vm/
├── Cargo.toml
├── src/
│   ├── lib.rs           # The Core Library
│   ├── math/            # Fixed-Point Linear Algebra
│   │   ├── fix.rs       # I64 wrapper (Q32.32)
│   │   └── matrix.rs    # Matrix Mul, Add, Transpose
│   ├── layer/           # Neural Layers
│   │   └── dense.rs     # W*x + b
│   ├── compiler/        # The Logic Synthesizer
│   │   ├── gates.rs     # AND, OR, XOR matrices
│   │   ├── assembler.rs # ASM -> Matrix Converter
│   │   └── obfuscator.rs# Basis Change Logic
│   └── vm/              # The Runtime
│       └── engine.rs    # The infinite loop
└── tests/               # Strict TDD folder
    ├── logic_tests.rs
    └── integration_tests.rs
```

---

## 🧪 TDD Example: The "ADD" Gate

To prove this is possible without Python, imagine we want `C = A + B`.

**State Vector:** $[A, B, C]$
**Matrix ($W$):**
$$
\begin{bmatrix}
1 & 0 & 0 \\
0 & 1 & 0 \\
1 & 1 & 0 
\end{bmatrix}
$$

**Execution:**
$$
\begin{bmatrix}
1 & 0 & 0 \\
0 & 1 & 0 \\
1 & 1 & 0 
\end{bmatrix}
\times
\begin{bmatrix}
5 \\
3 \\
0 
\end{bmatrix}
=
\begin{bmatrix}
5 \\
3 \\
8 
\end{bmatrix}
$$

*   Row 1: $1*A + 0*B + 0*C = A$ (Preserve A)
*   Row 2: $0*A + 1*B + 0*C = B$ (Preserve B)
*   Row 3: $1*A + 1*B + 0*C = A + B$ (Compute Sum)

This is how Stella works. But imagine that matrix is $1000 \times 1000$, scrambled with random noise, and doing 50 instructions simultaneously.

---

### 🛡️ Why this is "Unhinged" for Reverse Engineers

1.  **No CFG:** Tools like IDA Pro generate graphs based on `JMP` instructions. Stella has no `JMP`. It has a Context Vector that "drifts" between states. The Control Flow Graph is a single node: The Loop.
2.  **No Symbolics:** Symbolic Execution engines (Angr, Triton) rely on discrete path constraints (`if x > 5`). Stella is continuous. To solve `if x > 5`, the solver has to solve a system of 1,000 linear equations simultaneously.
3.  **Polymorphism on Steroids:** Every time you compile, we generate a random $P$ matrix. This generates a **completely unique** weight set. No two binaries look alike, even if the source code is identical.
