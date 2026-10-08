//! Lettras MCP server (Model Context Protocol, streamable HTTP, JSON-RPC over POST).
//!
//! The server embeds the compiled Lettras engine (WebAssembly, run by `wasmi`) and calls it in-process
//! through the [`PuzzleBackend`] trait: no network, no credentials, no environment variables.
//! It validates tool arguments, runs the engine, and formats the result for the model: a readable
//! grid plus the full structured puzzle. A free-tier [`Gate`] caps how many puzzles one client can create.

pub mod backend;
pub mod http;
pub mod limits;
pub mod protocol;
pub mod render;
pub mod tools;

pub use backend::{LocalEngine, PuzzleBackend};
pub use limits::{Gate, MemoryStore, RedisRestStore, UsageStore};
pub use protocol::handle_message;
