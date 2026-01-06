# Secure Phrase Manipulator (SPM)

## Project Overview
The goal of this project is to build a simple command-line utility that takes a phrase, transforms it multiple times, analyzes the resulting text, and securely tracks the history of all transformations.
This project emphasizes Rust’s **ownership**, **borrowing**, and **mutability** concepts while working with `String` and `&str`. Completing this project demonstrates a solid understanding of Rust’s memory safety rules, type conversions, and control flow.

---

## Features
- **Interactive Input:** Continuously prompts the user for phrases until the exit command (`q`) is entered.
- **Command-Line Arguments:** Initial input can also be provided via command-line arguments.
- **Numeric Transformations:** Extracts numbers from input and performs bitwise and shift operations on them.
- **Phrase Transformation:** Appends keys and suffixes to phrases, demonstrating move semantics and tuple returns.
- **Text Analysis:** Iterates over transformed phrases, counts characters, extracts words using slicing, and demonstrates nested loops.
- **History Tracking:** Maintains a dynamic history of all transformations, enforcing borrowing rules to prevent data races.

---

## Project Phases

The project  has been divided into 4 phases to simplify the work load and increase productivity.

### Phase 1: Setup & Control Flow
- Program initialization with `main()`.
- Infinite loop for user input with exit on `"q"`.
- Optional command-line input via `std::env::args()`.
- Input validation and skipping empty entries.
- calculate_transform_key() performs Bitwise Operations for the numbers from the user input

### Phase 2: String Manipulation & Ownership
- `transform_phrase()` moves `String` ownership into the function.
- Returns modified `String` and derived metrics as a tuple.
- Demonstrates **move semantics** vs **copy semantics** for primitives.
---
### Phase 3: Analysis, Slicing, and Nested Loops (E02)
- `analyze_phrase()`, that takes the `transformed phrase` as an immutable string slice (&str).
- must iterate through the slice character by character using **.chars().enumerate()**.
- Uses  loop to simulate complex processing.
- Inside the loop, **if/else** logic implemented to classify and count the characters based on a condition (counting vowels).
---

## Installation & Usage
1. Clone the repository:
   ```bash
   git clone https://github.com/dikshita0719/spm.git
   cd spm
