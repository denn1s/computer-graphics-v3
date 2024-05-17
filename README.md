# Computer Graphics Course

Welcome to the Computer Graphics Course repository! This repository contains code examples organized by branch, each illustrating different concepts and techniques in computer graphics using the Rust programming language.

## Table of Contents

- [Introduction](#introduction)
- [Getting Started](#getting-started)
- [Branches](#branches)
- [Running the Examples](#running-the-examples)
- [Contributing](#contributing)
- [License](#license)

## Introduction

This repository is designed to help you learn and explore various aspects of computer graphics through practical examples. Each branch of this repository corresponds to a different topic or module in the course, providing you with code and explanations to deepen your understanding.

## Getting Started

To get started with these examples, you will need to have Rust installed on your machine. If you don't have Rust installed, you can install it by following the instructions on the [official Rust website](https://www.rust-lang.org/).

### Prerequisites

- Rust: Install Rust by following the instructions [here](https://www.rust-lang.org/learn/get-started).

### Installation

1. Clone the repository:

    ```bash
    git clone git@github.com:denn1s/computer-graphics-v3.git 
    cd computer-graphics-v3
    ```

2. Checkout the branch you are interested in:

    ```bash
    git checkout branch-name
    ```

## Branches

Each branch in this repository corresponds to a specific topic covered in the course. The name of the branch starts with the name of the module followed by the expected order of the implementation.

- `SR-`: Software rendering examples 
- `RT-`: Ray tracer examples 

To switch to a branch, use the `git checkout` command as shown above.

## Running the Examples

Each branch contains a standalone Rust project. You can use Cargo to build and run the examples. Here are the steps:

1. Ensure you are in the root directory of the repository.
2. Checkout the desired branch:

    ```bash
    git checkout branch-name
    ```

3. Build and run the project using Cargo:

    ```bash
    cargo run
    ```

This command will compile the code and run the resulting binary. You should see the output in your terminal or a window will open displaying the rendered graphics, depending on the example.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Happy coding and enjoy your journey into computer graphics with Rust!

