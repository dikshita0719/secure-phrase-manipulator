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

## Installation & Usage
1. Clone the repository:
   ```bash
   git clone https://github.com/dikshita0719/spm.git
   cd spm
