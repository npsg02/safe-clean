# safe-clean

A safe disk cleanup CLI/TUI tool built with Rust that helps you identify and clean up unnecessary files on your system.

## Features

- **Interactive TUI Mode**: Browse and explore your disk usage with an intuitive terminal interface
- **Docker Cleanup**: Safely remove unused Docker containers, images, volumes, and networks
- **Temporary Files Cleanup**: Clean system temporary directories and files
- **Directory Analysis**: List directories by size to identify space usage
- **Large File Discovery**: Find files larger than a specified threshold
- **Development Artifacts Cleanup**: Discover and remove `node_modules`, `.venv`, `target`, and other development artifacts
- **Dry Run Mode**: Preview what would be cleaned without actually removing anything
- **Safety First**: All operations include confirmation prompts and safety checks

## Installation

### Using Nix Flakes

```bash
# Run directly without installing
nix run github:npsg02/safe-clean

# Install to your profile
nix profile install github:npsg02/safe-clean

# Try it out in a temporary shell
nix shell github:npsg02/safe-clean
```

### Using Nix (traditional)

```bash
# Install using nix-env
git clone https://github.com/npsg02/safe-clean.git
cd safe-clean
nix-env -if .
```

### NixOS Configuration

Add to your `configuration.nix`:

```nix
{
  inputs.safe-clean.url = "github:npsg02/safe-clean";
  # ... other inputs
}

# In your configuration
{ inputs, ... }:
{
  imports = [ inputs.safe-clean.nixosModules.default ];
  
  programs.safe-clean.enable = true;
}
```

Or add directly to `environment.systemPackages`:

```nix
{ pkgs, ... }:
{
  environment.systemPackages = [
    (pkgs.callPackage (pkgs.fetchFromGitHub {
      owner = "npsg02";
      repo = "safe-clean";
      rev = "main"; # or specific commit/tag
      sha256 = ""; # nix will tell you the correct hash
    }) {})
  ];
}
```

### Build from source

```bash
# Build from source with Cargo
git clone https://github.com/npsg02/safe-clean.git
cd safe-clean
cargo build --release
```

## Usage

### Interactive TUI Mode (Default)

Launch the interactive terminal interface:

```bash
safe-clean
# or explicitly
safe-clean tui
```

### CLI Commands

#### List Directories by Size
```bash
# List top 20 largest directories in current directory
safe-clean list

# List top 10 largest directories
safe-clean list --top 10

# Analyze specific path
safe-clean list /path/to/analyze --top 15
```

#### Find Large Files
```bash
# Find files larger than 100MB (default)
safe-clean large

# Find files larger than 1GB
safe-clean large --size 1GB

# Search in specific directory
safe-clean large /path/to/search --size 500MB
```

#### Development Artifacts Cleanup
```bash
# Find development artifacts (dry run)
safe-clean dev-clean --dry-run

# Actually remove development artifacts (with confirmation)
safe-clean dev-clean

# Search in specific directory
safe-clean dev-clean /path/to/projects --dry-run
```

#### Docker Cleanup
```bash
# Preview Docker cleanup
safe-clean docker --dry-run

# Clean up Docker resources (with confirmation)
safe-clean docker
```

#### Temporary Files Cleanup
```bash
# Preview temp files cleanup
safe-clean temp --dry-run

# Clean up temporary files (with confirmation)
safe-clean temp
```

### Command Options

- `--dry-run`: Preview what would be cleaned without actually removing anything
- `--top N`: Limit results to top N items (for list command)
- `--size SIZE`: Specify size threshold (e.g., "100MB", "1GB", "500KB")

## Safety Features

- **Confirmation Prompts**: All destructive operations require user confirmation
- **Dry Run Mode**: Preview operations before executing them
- **Safe File Detection**: Only removes files that match known safe patterns
- **Path Validation**: Prevents removal of system-critical directories
- **Detailed Reporting**: Shows exactly what will be or was cleaned

## Development Artifacts Detected

- `node_modules` (Node.js)
- `.venv`, `venv` (Python virtual environments)
- `__pycache__` (Python cache)
- `.tox` (Python testing)
- `target` (Rust)
- `build`, `dist` (General build artifacts)

## Size Format

Size values can be specified in various formats:
- `1024` (bytes)
- `1KB`, `1MB`, `1GB`, `1TB`
- `1.5GB`, `500MB`, etc.

## Example Output

```
🛠️  Development Artifacts Cleanup
=================================
Searching in: .

📊 Found development artifacts:
Path                                                                    Size      Items
-------------------------------------------------------------------------------------
./target                                                            484.2 MB       1220
./node_modules                                                       156.3 MB        892

📈 Summary:
   Total artifacts: 2
   Total size: 640.5 MB
   Total items: 2112

Remove 2 development artifacts (640.5 MB)? [y/N]
```

## Documentation

For complete documentation with examples and detailed usage instructions, visit our [documentation website](https://npsg02.github.io/safe-clean/).

## License

MIT License
