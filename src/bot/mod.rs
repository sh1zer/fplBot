//! Discord bot module for FPL (Fantasy Premier League) functionality
//!
//! This module contains the Discord bot implementation including command handlers,
//! event handlers, and the core bot logic for interacting with Discord users.
//!
//! The bot provides FPL-related commands like viewing league standings, fixtures,
//! and manager information through Discord slash commands.
//!
//! # Modules
//!
//! * [`commands`] - Slash command implementations for FPL functionality
//! * [`handlers`] - Discord event handlers and interaction processing

pub mod commands;
pub mod handlers;