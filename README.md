# rust-bitcoin Integration Tests

This repository provides interoperability and integration tests for the rust-bitcoin ecosystem.

Its purpose is to verify compatibility between rust-bitcoin and external libraries without introducing additional development dependencies into the core rust-bitcoin workspace.

The project focuses on:

- Serde interoperability
- Serializer compatibility
- Regression reproduction
- Cross-version compatibility
- Public API contract validation

# Goals
✔ Test rust-bitcoin with multiple serializers

✔ Discover interoperability bugs

✔ Produce minimal reproductions

✔ Feed fixes upstream

✔ Maintain compatibility matrix

✔ Provide regression corpus

# None-goals
This project is NOT:

- a fork of rust-bitcoin

- a replacement for rust-bitcoin tests

- an official rust-bitcoin repository

- a place for changing rust-bitcoin APIs

Instead:

- Its purpose is to improve rust-bitcoin through ecosystem-level testing.
