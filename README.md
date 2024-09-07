# R.I.D.E. - Rust Integrated Development Environment

R.I.D.E. is a **GUI-focused** Rust Integrated Development Environment built using **Bevy**, **egui**, and **winit**. The IDE is designed with modularity and extensibility in mind, allowing developers to customize and extend its features. Although primarily a graphical environment, the project is structured to support future expansions such as plugin systems and cross-platform integration.

## Project Structure

R.I.D.E. is organized to separate concerns cleanly, with a strong focus on modularity and ease of future development.

### Directory Overview:

- **`Cargo.toml`** – Rust’s package manager file for managing dependencies like `bevy`, `egui`, and `winit`.
- **`src/main.rs`** – The entry point for the IDE, initializing the Bevy app.
- **`src/core/`** – Core modules and functionality of R.I.D.E.:
  - **`mod.rs`** – Core module handler.
  - **`ui/`** – User interface logic and systems:
    - **`initiate_window.rs`** – Handles window creation and initialization using Bevy and winit.
    - **`modules/`** – Contains key components of the UI, such as:
      - **`code_editor.rs`** – A text editor for writing code.
      - **`file_explorer.rs`** – A file explorer for navigating and managing project files.
      - **`terminal.rs`** – A terminal interface integrated into the UI for command-line operations.
    - **`mod.rs`** – UI module handler, coordinating the different components of the user interface.

## Key Technologies

- **Bevy**: A flexible, open-source game engine that is used here for window management and rendering.
- **egui**: An immediate mode GUI library used for creating interactive graphical interfaces within the IDE.
- **winit**: A cross-platform window creation and event loop management library that provides support for windowed applications.

## Features (In Progress)

- **Modular GUI Interface**: The project currently supports a flexible and modular graphical user interface that includes a file explorer, code editor, and integrated terminal.
- **Bevy-Based Rendering**: R.I.D.E. leverages Bevy for window management and graphics rendering, ensuring smooth and efficient performance.
- **Extensible Design**: The codebase is designed to allow future extensions, including language-specific plugins, advanced code analysis tools, and additional UI features.
- **Cross-Platform Potential**: While the initial development is focused on desktop platforms, the modular architecture lays the groundwork for easy adaptation to other platforms.

## Installation

To run R.I.D.E. on your local machine, you need to have **Rust** installed. Follow these steps:

1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/ride.git
    ```

2. Navigate to the project directory:
    ```bash
    cd ride
    ```

3. Build the project using Cargo:
    ```bash
    cargo build
    ```

4. Run the application:
    ```bash
    cargo run
    ```

## Roadmap

- **Plugin System**: Future updates will include a plugin system for language support and additional IDE features.
- **Advanced Code Editor**: Enhance the code editor with syntax highlighting, autocompletion, and language server support.
- **Cross-Platform Support**: Extend support to platforms such as macOS and Linux.
- **Theming**: Enable customizable theming for the GUI interface.

## Contributing

Contributions are welcome! If you'd like to contribute, please fork the repository and create a pull request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License.
