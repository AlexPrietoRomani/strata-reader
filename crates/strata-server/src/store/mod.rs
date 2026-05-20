//! [`JobStore`] backends.
//!
//! - [`memory::MemoryJobStore`] — default, lock-free reads via parking_lot.
//! - [`sqlite::SqliteJobStore`] — restart-recovery store (Plan §14.T9.2).
//! - Redis backend is on the optional roadmap and not shipped yet.

pub mod memory;
pub mod sqlite;

pub use memory::MemoryJobStore;
pub use sqlite::SqliteJobStore;
