# FPL Bot

A Discord bot designed to integrate with the Fantasy Premier League (FPL) API, providing users with quick access to league standings, user picks, fixtures, and more. Mainly made so me and my friends can look up stats mid conversation.

## Overview

By utilizing the [FPL API](https://github.com/sh1zer/fpl_api), FPL Bot allows users to track their mini-leagues, view upcoming fixtures, and check manager picks without leaving Discord. The bot also retains user and channel information to provide a seamless, persistent experience across gameweeks.

## Features

### Currently Implemented
*   **League Standings:** Fetch and display current standings for classic mini-leagues.
*   **Manager Picks:** View the current squad selections and formations for specific managers.
*   **Upcoming Fixtures:** Retrieve fixture schedules filtered by gameweek or specific teams.
*   **Data Retention:** Saves user preferences and channel configurations for streamlined command usage.

### In Development (WIP)
*   **Live Data Tracking:** Real-time updates for matchdays, including live points, bonus points, and player statistics as matches are being played.


## Commands

FPL Bot currently supports the following slash commands within Discord:

*   `/hello` - A simple command to say hello to the bot and verify it's responsive.
*   `/standings` - Displays the current classic mini-league standings with pagination.
*   `/fixtures` - Retrieves upcoming Premier League fixtures.
*   `/check_team` - Looks up a specific manager's current team selections and formation.
*   `/update_manager_id` - Links or updates your personal FPL Manager ID to your Discord account.
*   `/check_manager_id` - Checks which FPL Manager ID is currently linked to your Discord account.
*   `/update_channel_league_id` - Sets or updates the default FPL League ID for the current Discord channel.
*   `/check_channel_league_id` - Displays the current default FPL League ID bound to the Discord channel.


## Installation and Setup

### A Discord Bot Token is required. [A tutorial](https://discordpy.readthedocs.io/en/stable/discord.html)

### Instructions
1. **Clone the repository:**
```bash
git clone https://github.com/sh1zer/fplBot.git
cd fplBot
```
2. Set your .env variables
```
BOT_TOKEN=your_bot_token_here
```
3. Run the bot
```
cargo run --release
```
