<div align="center">

![Iris Banner](https://svg-banners.vercel.app/api?type=luminance&text1=Project%20Stella&width=800&height=200&color=FA7AEB)

**A continuous-state neural virtual machine executing Turing-complete logic via encrypted geometric noise.**

[Features](#core-capabilities) • [Architecture](#technical-deep-dive) • [Installation](#quick-start) • [Usage](#usage-examples) • [Security](#spatial-obfuscation--security)

</div>

---

## Overview

**Project Stella** is a highly experimental Proof of Concept (PoC) for a fundamentally new paradigm in software execution. Instead of executing discrete, sequential instructions moving through a traditional von Neumann memory space, Stella compiles standard assembly logic into a massive, multi-dimensional **neural matrix**.

During runtime, the execution state (variables, instruction pointers, and logic branches) exists purely as "activation energy" bouncing around an unhinged linear algebra system. To an outside observer dumping the VM's memory, the program looks like a chaotic mess of floating-point noise. Conceptually, Stella acts as a "Neural Fully Homomorphic Encryption (FHE)" environment, enabling the host processor to perform deterministic logic without ever comprehending the semantics, spatial layout, or sequential flow of the code it is running.

## Core Capabilities

### 🧠 Pure Matrix Execution (Continuous State)

All arithmetic (`ADD`, `SUB`), logic (`AND`, `OR`, `NAND`, `NOT`), and memory (`LATCH`) operations are superimposed onto a unified weights matrix. The entire Virtual Machine has no traditional CPU loop—it simply multiplies the state vector by the matrix on every tick.

### ♾️ Attractor Control Flow

Standard branching (`JMP`, `JEQ`) is implemented via continuous attractor dynamics. Execution energy physically drains from one region of the matrix and pulses into another based on neural activation thresholds.

* Instructions on the same matrix row execute simultaneously in superposition via **Additive Accumulation**.

### 🧮 Fixed-Point Determinism

To prevent the "Vanishing Gradient" problem common in recurrent neural systems, Stella uses a custom **Q32.32 Fixed-Point Math Engine**. This guarantees 0.000000000 rounding error, allowing infinite loops and stable latches to sustain execution for billions of cycles without floating-point degradation.

### 🔐 Spatial Obfuscation (Polymorphism)

Every compiled program is pad-injected with "junk nodes" and cryptographically shuffled using an $N \times N$ Permutation Matrix. No two compiled binaries look alike, and the logical data inside the VM registers is geometrically scrambled, defeating memory scanners and traditional reverse engineering.

### 🌪️ Chaotic Side-Channel Defense

The "junk nodes" are not simply zeroed out; they are woven into **Irrational Rotational Matrices** that force the CPU to compute non-repeating orbital trajectories bounded by the activation function. This boils the runtime state in active entropy, blinding Differential Power Analysis (DPA) and electromagnetic side-channel attacks.

---

## Technical Deep Dive

### The Execution Engine

The `stella_vm` binary contains no parsers or interpreters. It evaluates the entire program simultaneously via a single non-linear mathematical step:

$$S_{t+1} = \text{clamp}_{0}^{1}(W \cdot S_t + B)$$

* $W$ = The dense Weight Matrix (Logic/Routing).
* $B$ = The Bias Vector (Constant loads/Thresholds).
* $S_t$ = The State Vector (Registers + Instruction Pointer).
* $\text{clamp}_{0}^{1}$ = The activation function preventing unbounded energy growth.

### The Compiler & Spatial Cryptography

When you compile a `.asm` script, the `compile_demo` assembler generates the true logic matrices ($W_{clean}$, $B_{clean}$). It then generates a random orthogonal permutation matrix ($P$) and its inverse ($P^{-1}$) to scramble the coordinate space:

$$W_{obfuscated} = P^{-1} \times W_{clean} \times P$$

This spatial transformation is mathematically isomorphic—the continuous logic executes perfectly inside the encrypted basis, and is only decoded back to physical reality by the legitimate client holding the private `.key` file.

To generate this permutation matrix, the compiler initializes a **Hénon Map Strange Attractor** seeded by nanosecond system time, extracting cryptographic entropy from a non-linear chaotic orbit.

### Execution Semantics: Hardware over Software

Because Stella evaluates all rows simultaneously, it fundamentally behaves like a **Field Programmable Gate Array (FPGA)** rather than a sequential CPU. Signals experience physical *propagation delay* (e.g., a `NOT` gate takes exactly one matrix multiplication cycle to invert its output). As a result, developers must account for real-world electrical race conditions by using "Buffer Nodes" to hold execution energy while logic gates settle.

---

## Quick Start

### Requirements

* **Rust** 1.70+
* **Cargo** package manager

### Installation

```bash
# Clone the repository
git clone https://github.com/SSL-ACTX/Stella.git
cd Stella

# Run the test suite to verify the fixed-point math and neural parser
cargo test

```

---

## Usage Examples

Stella operates in a disconnected workflow. You compile the matrix, encode your inputs using a client tool, run the matrix blindly in the VM, and decode the output.

### 1. Write the Assembly

Create a file called `program.asm`. The first line must define the required logical `.SIZE`.

```nasm
.SIZE 4

; Constants
MOV N0, 1.0      
MOV N1, 1.0      

; Logic
AND N2, N0, N1   ; N2 = 1.0 AND 1.0 = 1.0

; Control Flow / Memory
JEQ N2, N0, N3   ; If N2 is active, pulse N0 into N3
LATCH N3, N3     ; Trap the energy in N3 permanently

```

### 2. Compile Polymorphic Payload

This will pad the state size (e.g., to 16 nodes), scramble the matrix geometrically, and output `my_app.stella` (the binary) and `my_app.key` (your decoder ring).

```bash
cargo run --bin compile_demo -- program.asm my_app

```

### 3. Encode User Input

Pass the logical inputs to the `stella_client`. It maps your logical bits into the physical obfuscated locations within the padded 16-dimensional vector.

```bash
cargo run --bin stella_client -- encode my_app.key 1.0 1.0 0.0 0.0
# Output: [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, ...] (Copy this payload)

```

### 4. Execute the Unhinged Matrix

Paste the encoded payload directly into the Stella VM. The runtime has no concept of what it is executing; it simply circulates the noise for $N$ cycles.

```bash
cargo run --bin stella_vm -- my_app.stella 0.0 1.0 0.0 0.0 0.0 0.0 1.0 0.0 ...
# Output: Final I/O State: [0.0, 0.0, 1.0, 0.0, 1.0, 0.0, ...] (Copy this output)

```

### 5. Decode the Result

Feed the VM's raw output back into the client to collapse the superposition back into readable discrete logic.

```bash
cargo run --bin stella_client -- decode my_app.key 0.0 0.0 1.0 0.0 1.0 0.0 ...
# Output: [1.0, 0.0, 1.0, 1.0, 0.0, ...] (N3 Successfully Latched!)

```

---

## Spatial Obfuscation & Security

To prove the robustness of the matrix compilation, this repository includes a Known-Plaintext Attack Proof of Concept (`stella_dis`).

If an attacker captures both your un-padded `source.asm` and your `app.stella` binary, they can theoretically brute-force the permutation matrices:

```bash
cargo run --bin stella_dis source.asm app.stella
# [!] SUCCESS: Obfuscation key recovered! Mapping: [1, 2, 3, 0]

```

To defend against this, the production compiler pads the `.SIZE` to $16+$ nodes. A 16-node state vector yields **$16! = 20,922,789,888,000$ possible permutations**, making algebraic brute-forcing computationally unfeasible for average attackers. The addition of active rotational entropy within these padded nodes further obscures the operational payload from side-channel observers.

---

## Disclaimer

> [!WARNING]
> **Experimental PoC Status:** Project Stella is a research endeavor into continuous-state execution and neural obfuscation. It is strictly an alpha-stage Proof of Concept.
> * **Not for Production:** The assembler does not currently implement syntax checking, and the linear constraints mean multi-layer programs require manual "Energy Register" management to prevent matrix row collisions. Furthermore, code must be engineered with hardware-like propagation delays in mind to prevent race conditions.
> * **Performance:** Simulating sequential logic via $O(N^2)$ matrix multiplications is inherently slower than standard native execution.
> 
> 

---

<div align="center">

**Author:** Seuriin ([SSL-ACTX](https://github.com/SSL-ACTX))

*Licensed under the [AGPL 3.0](LICENSE) License.*

</div>
