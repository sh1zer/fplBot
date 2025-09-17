//! Discord slash command implementations for FPL functionality
//!
//! This module contains all the Discord slash command handlers that provide
//! Fantasy Premier League functionality to Discord users.
//!
//! # Available Commands
//!
//! * [`standings`] - Display league standings with interactive pagination
//! * [`manager`] - Manager-related commands for viewing and updating manager information
//! * [`player`] - Player statistics and information commands
//! * [`track_fixture`] - Track and get notifications for specific fixtures
//! * [`fixtures`] - Display gameweek fixtures with scores and details
//!
//! Each command module provides:
//! - Command registration function for Discord
//! - Main command execution handler
//! - Helper functions for data processing and response formatting

pub mod standings;
pub mod manager;
pub mod player;
pub mod track_fixture;
pub mod fixtures;
