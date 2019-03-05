# Evaluation guidelines

These are the guidelines used by the Resources team to evaluate project
submissions.

## Hard requirements

The project must fulfill these requirements or it will be rejected.

- It's an application (binary crate) that runs on embedded hardware (Single
  Board Computer, development board or custom PCB).

- The main logic of the application must be written in Rust.

- The source code must be public. Licensing terms are unimportant.

## Bonus points

> N.B. A project repository may contain more than one crate. In those cases, one
> (binary) crate will be the application, and the other crates will be libraries
> and / or tools. In the review consider only the crates that are compiled for
> the target device. One of these crates will be the application ("application
> code") and the rest will be libraries ("support code").

Award points if

- The project is unlike the projects currently on display.

- The documentation includes instructions on how to build the program, or the
  firmware can be built with just `cargo build`.

- The repository has a CI setup.

- The application compiles on stable.

- The application code is free from `unsafe` code and all `unsafe` code has been
  pushed to libraries that only expose a safe API.

- Code that's not target specific, if any, has unit tests.

- Support code, if any, is documented.

## Penalties

Subtract points if

- The application contains potential soundness issues. For example, using
  `mem::transmute`, unchecked creation of singletons and unchecked use of
  `static mut` variables are huge red flags.

- Explicit panicking (e.g. `unwrap`) is used instead of proper error handling.
  However, note that `unreachable!` is OK if properly used.
