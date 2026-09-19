//! `coreai` is the Rust binding and model-kit crate for Apple's Core AI
//! runtime (`.aimodel`) — the 1:1 counterpart of
//! [`coremlit`](https://crates.io/crates/coremlit), which plays the same
//! role for CoreML. It will expose the same kit vocabulary (a safe runtime
//! core plus opt-in, feature-gated model kits); the concrete backend a
//! downstream consumer (for example `mediagraph`) runs is chosen by feature.
//!
//! # Platform
//!
//! Core AI is only available on macOS 27+ and iOS 27+; this crate targets
//! Apple platforms only.
//!
//! # Status
//!
//! `0.0.0` reserves the name on crates.io. No API is implemented yet.

#![deny(missing_docs)]
