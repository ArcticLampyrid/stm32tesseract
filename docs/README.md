# STM32Tesseract
STM32Tesseract is a utility designed to seamlessly integrate STM32CubeMX-generated code with contemporary IDEs and build systems.

## Motivation
While STM32CubeMX is an excellent tool for initializing STM32 projects, it often falls short in supporting modern, user-friendly toolchains. STM32Tesseract aims to bridge this gap, enhancing the development experience by facilitating integration with advanced IDEs and build systems.

## Features
- **CMake Integration**: Effortlessly integrate with CMake to streamline your build process.
- **Build Environment Setup**: Provides a straightforward approach to configuring your build environment.
- **VSCode Project Setup**: Simplifies the process of setting up a project in Visual Studio Code.
- **CLion Project Setup**: Enables easy configuration of CLion projects.

## Usage
For detailed instructions, refer to the [Tutorial](Tutorial.md).

## Build
*Note: The following instructions are intended solely for STM32Tesseract developers.*

STM32Tesseract utilizes Cargo as its build system. To build the project, execute `cargo build` in the root directory of the project.