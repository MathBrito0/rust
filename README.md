# Rust Study

A repository focused on learning Rust from scratch!

# Achievements:

- [🔄] Learn basic concepts;
- [🔄] Learn Rust fundamentals;
- [ ] Ownership;
- [ ] Borrowing;
- [ ] Pattern Matching;


# Structure:

This is a **Cargo Workspace** with various exercises organized independently.

(rust/

├── exercicio-beta/ # First program (Hello, Rust!)

├── exercicio-1/ # [Simple calculator]

└── ...)

# Creating modules in Rust.

## 1. Create the structure via the terminal.

The "exercicios" folder contains independent Rust projects.
Each exercise has its own 'Cargo.toml' and its own 'main.rs'.

'''bash
cd /workspaces/rust
cargo new exercicios/exercicio-2

(Ensure the new exercise has been added to the root Cargo.toml

["workspace"]
members = [
    "exercicios/exercicio-0",
    "exercicios/exercicio-1",
    "exercicios/exercicio-2",
]
resolver = "3")

At the project root (`rust`), run:

```bash
git add .
git commit -m "Organiza exercícios em workspace"
git push origin main
```

If a remote repository isn't configured yet, add it first:

```bash
git remote add origin https://github.com/account-name/rust.git
git push -u origin main
```

To check if the remote already exists: 
```bash
git remote -v
```

`git add .` stages the changes, `git commit` creates a local record and `git push` sends it to GitHub.

# How to run and compile a project? 

/*cargo run -p exercise-1*/

# Error checking

/*cargo check*/