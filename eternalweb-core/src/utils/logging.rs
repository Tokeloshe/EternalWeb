//! Logging utilities

/// Log event types
#[derive(Debug, Clone)]
pub enum LogEvent {
    NetworkEvent,
    ArchiveEvent,
    SecurityEvent,
    ErrorEvent,
}
