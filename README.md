Markdown
# Rust Game of Life

A terminal-based implementation of Conway's Game of Life, written in Rust. This project demonstrates efficient data structures, concurrency, and terminal manipulation to create a dynamic and engaging simulation.

## Features

* **Dynamic Visualization:** Watch the Game of Life unfold in your terminal, using the `termion` library for clear and responsive rendering.
* **Graceful Interruption:** Press Ctrl+C (SIGINT) to gracefully exit the simulation without leaving artifacts in your terminal.
* **Glider Pattern:** The simulation starts with a classic Glider pattern, showcasing the emergent behavior of cellular automata.
* **Efficient Implementation:**  The code utilizes a `HashSet` for efficient cell storage and manipulation, ensuring smooth performance even with large populations.
* **Multithreading:**  A dedicated thread handles signal detection, ensuring the simulation remains responsive to interrupts.

## How to Run

1. **Prerequisites:** Ensure you have Rust and Cargo installed on your system.

2. **Clone the Repository:**
```bash
git clone https://github.com/boorboor/cgl
cd cgl
```

3. **Clone the Repository:**
```bash
cargo run
```

## Controls
**Ctrl+C:** Exit the simulation.
