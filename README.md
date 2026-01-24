# Steamdl

[![SkillIcons](https://skillicons.dev/icons?i=rust,react,tauri,vite)](https://skillicons.dev)

A simple, efficient desktop application for downloading Steam Workshop content without requiring a Steam account login. The tool leverages an embedded SteamCMD instance in anonymous mode to fetch and organize workshop items automatically.

## Releases

Ready-to-use executables are available in the **[Releases](https://github.com/Azertyuiop442/Steamdl/releases)** section of this repository.

- **[Download the Latest Installer (Windows)](https://github.com/Azertyuiop442/Steamdl/releases/latest)** - Includes all necessary SteamCMD dependencies.

## Core Functionality

- **Anonymous Downloads**: Fetches content via SteamCMD without needing user credentials.
- **Automated Organization**: Parses the workshop page to retrieve the actual mod name and organizes files into descriptive directories instead of raw IDs.
- **Smart Directory Hoisting**: Automatically detects and flattens `mods/` sub-folders to ensure compatibility with standard installation structures.
- **Tauri 2.0 Backend**: Built with Rust for safe, high-performance file operations and a React frontend for state management.

## Getting Started (For Developers)

### Prerequisites

- [Bun](https://bun.sh/) (recommended) or Node.js.
- Rust toolchain (for building from source).

### Development Environment

1. Clone the repository.
2. Install dependencies:
   ```bash
   bun install
   ```
3. Boot the environment:
   ```bash
   bun tauri dev
   ```

### Building

To create a production-ready installer:
```bash
bun tauri build
```
The installer will be generated in `src-tauri/target/release/bundle/nsis/`.

## How it works

1. **Metadata Fetching**: When a URL is entered, the app fetches the Steam Workshop page to extract the Title and AppID.
2. **Download Execution**: SteamCMD is invoked as a sidecar process with a generated runscript.
3. **Post-Processing**: Once the download completes, the backend moves the files from the SteamCMD cache to the `download/` folder, sanitizing the name and flattening the directory structure where necessary.

## License

This project is licensed under the **GNU General Public License v3.0**. See the [LICENSE](LICENSE) file for details.
