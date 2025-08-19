# Minimal Rust Godot Template

A clean, minimal template for Godot 4 development with Rust using gdext. Perfect for both advanced users starting a new project and beginners who want to see the minimal working result.

## 📁 Project Structure

```
minimal-rust-godot-template/
├── godot/              # Clean Godot 4 project with .gdextension linker
└── rust/               # Rust code with basic MyClass example
    └── src/lib.rs      # Static method and constructor examples
```

## 🚀 Quick Start

1. Clone the repository
2. Build the Rust library:
   ```bash
   cd rust
   cargo build
   ```
3. Open the `godot/` project in Godot 4.4.1+ (works with 4.5+)
4. Find `MyClass` in the Create Node dialog

![Screenshot](<a href="https://ibb.co/1YyYt6Nf"><img src="https://i.ibb.co/CpCp32y5/Untitled.png" alt="Untitled" border="0"></a>)