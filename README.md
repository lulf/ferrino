# ferrino

Ferrino is an Arduino-like framework for writing embedded applications that makes use of the Rust
type system and build system to build embedded applications.

Ferrino allows you to:

* Write a single application targeting multiple physical boards
* Define capabilities for boards and
* Depend on capabilities in your application

Ferrino wires the board you have selected for your application, if it supports the required
capabilities of the application. If not, you get a compile time error.
