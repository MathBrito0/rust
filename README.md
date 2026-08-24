# Rust Study

A repository focused on learning Rust from scratch!

# Achievements:

- [🔄] Learn basic concepts;
- [ ] Learn Rust fundamentals;
- [ ] Ownership;
- [ ] Borrowing;
- [ ] Pattern Matching;


# Structure:

This is a **Cargo Workspace** with various exercises organized independently.

(rust/

├── exercise-beta/ # First program (Hello, Rust!)

├── exercise-1/ # [Simple calculator]

└── ...)

# How to run a project? 

/*cargo run -p exercise-1*/

# How to compile the project?

/*cargo run -p exercise-name*/

# Error checking

/*cargo check*/

Na raiz do projeto (`rust`), execute:

```bash
git add .
git commit -m "Organiza exercícios em workspace"
git push origin main
```

Se ainda não existir um repositório remoto configurado, adicione-o antes:

```bash
git remote add origin https://github.com/MathBrito0/rust.git
git push -u origin main
```

Para conferir se o remoto já existe:

```bash
git remote -v
```

O `git add .` adiciona as mudanças, o `git commit` cria um registro local e o `git push` envia para o GitHub.