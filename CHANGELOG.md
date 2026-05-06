# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2025-07-20

### Changed

- Make `pnet` a feature (enabled by default). This can be turned off if there's
  platform problems or people want to bring their own function to generate a
  machine id and want to save a dependency.
- Upgrade crate to Rust Edition 2024
