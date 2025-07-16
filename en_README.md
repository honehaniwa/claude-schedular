# Claude Scheduler

A web-based scheduler for Claude AI commands with Git worktree support - Built with Dioxus

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org/)
[![Dioxus](https://img.shields.io/badge/dioxus-0.5-green.svg)](https://dioxuslabs.com/)

## Overview

Claude Scheduler is a desktop application that allows you to easily schedule and execute Claude AI commands and shell commands through a GUI. Built with Rust and using the Dioxus framework for modern web UI technology.

## Features

### ✅ Core Features
- **Claude Code Execution**: Send prompts to Claude AI and execute them
- **Shell Mode**: Execute shell commands directly
- **Time-based Scheduling**: Schedule execution for today/tomorrow at any time
- **Instant Execution**: Execute commands immediately

### ✅ Git Worktree Support
- **Parallel Execution**: Execute commands in different git branches simultaneously
- **Branch Selection**: Choose from available git worktree branches
- **Worktree Management**: Automatic worktree detection and management

### ✅ Scheduling Features
- **Minute-level Precision**: Set exact time (0-23 hours, 0-59 minutes)
- **Schedule Management**: Add, edit, delete, and view schedules
- **5-second Monitoring**: Automatic schedule monitoring
- **Status Management**: Track pending, completed, and failed executions

### ✅ Execution History & Results
- **Execution History**: Record all executions (manual, automatic, shell)
- **Detailed Results**: Display stdout/stderr output
- **Reuse Commands**: Reuse commands from history
- **Color-coded Results**: Visual distinction for success/failure

### ✅ UI/UX Features
- **Dark Mode**: Light/Dark theme switching
- **Responsive UI**: Adapts to screen size
- **Real-time Status**: Live execution status updates

## Installation

### Prerequisites
- Rust 1.70 or higher
- Git (for worktree functionality)
- Claude CLI (for Claude AI mode)

### Building from Source
```bash
# Clone the repository
git clone https://github.com/your-username/claude-scheduler.git
cd claude-scheduler

# Build the project
cargo build --release

# Run the application
cargo run
```

### Using Pre-built Binary
Download the latest release from the [releases page](https://github.com/your-username/claude-scheduler/releases).

## Usage

### 1. Claude Code Mode (Default)
1. Enter your Claude AI prompt in the text area
2. Click "▶️ Execute Immediately" or schedule for later execution

### 2. Shell Mode
1. Enable "💻 Shell Mode" checkbox
2. Enter shell commands (e.g., `ls -la`, `echo 'hello'`)
3. Execute commands directly

### 3. Git Worktree Mode
1. Enable "🌿 Git Worktree Parallel Execution"
2. Select target branch from dropdown
3. Commands will execute in the selected branch's worktree

### 4. Schedule Execution
1. Check "⏰ Enable Time-based Automatic Execution"
2. Select today/tomorrow
3. Set time (0-23 hours) and minutes (0-59 minutes)
4. Click "📅 Schedule Registration"

### 5. View Execution History
- Check "📊 Execution History & Results" section
- Green border indicates success, red indicates failure
- Use "🔄 Reuse" button to re-execute past commands

## Configuration

The application stores its configuration and history in memory during runtime. For persistent storage, modify the code to use a database or file system.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup
```bash
# Clone the repository
git clone https://github.com/your-username/claude-scheduler.git
cd claude-scheduler

# Install dependencies
cargo build

# Run in development mode
cargo run
```

## Technical Specifications

- **Language**: Rust 2021 Edition
- **UI Framework**: Dioxus 0.5 (Desktop)
- **Async Runtime**: Tokio
- **Date/Time**: Chrono
- **Architecture**: Modular design with clean separation of concerns

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Dioxus](https://dioxuslabs.com/) for the excellent UI framework
- [Claude AI](https://claude.ai/) for the AI capabilities
- The Rust community for the amazing ecosystem

## Support

If you encounter any issues or have questions, please open an issue on GitHub. 