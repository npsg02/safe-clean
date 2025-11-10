# Nix and NixOS Installation Guide

This guide covers various ways to install and use `safe-clean` on Nix and NixOS systems.

## Prerequisites

- Nix package manager installed (for non-NixOS systems)
- Nix flakes enabled (recommended)

To enable flakes, add this to your `~/.config/nix/nix.conf` or `/etc/nix/nix.conf`:
```
experimental-features = nix-command flakes
```

## Installation Methods

### 1. Run Without Installing (Nix Flakes)

The quickest way to try `safe-clean`:

```bash
nix run github:npsg02/safe-clean
```

This will download, build, and run the latest version without installing it permanently.

### 2. Install to User Profile (Nix Flakes)

To install `safe-clean` to your user profile:

```bash
nix profile install github:npsg02/safe-clean
```

After installation, you can run it directly:
```bash
safe-clean
```

To update to the latest version:
```bash
nix profile upgrade safe-clean
```

To remove:
```bash
nix profile remove safe-clean
```

### 3. Temporary Shell (Nix Flakes)

Create a temporary shell with `safe-clean` available:

```bash
nix shell github:npsg02/safe-clean
```

This is useful for one-off usage or testing.

### 4. Traditional Nix Installation

For systems not using flakes:

```bash
git clone https://github.com/npsg02/safe-clean.git
cd safe-clean
nix-env -if .
```

### 5. NixOS System-Wide Installation

#### Using the NixOS Module

Add `safe-clean` as a flake input in your `flake.nix`:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    safe-clean.url = "github:npsg02/safe-clean";
  };

  outputs = { self, nixpkgs, safe-clean, ... }: {
    nixosConfigurations.your-hostname = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        safe-clean.nixosModules.default
        {
          programs.safe-clean.enable = true;
        }
      ];
    };
  };
}
```

#### Direct Package Installation

In your `configuration.nix`:

```nix
{ config, pkgs, ... }:

{
  environment.systemPackages = [
    (pkgs.callPackage (builtins.fetchGit {
      url = "https://github.com/npsg02/safe-clean.git";
      ref = "main";
    }) {})
  ];
}
```

Or using `fetchFromGitHub`:

```nix
{ config, pkgs, ... }:

{
  environment.systemPackages = [
    (pkgs.callPackage (pkgs.fetchFromGitHub {
      owner = "npsg02";
      repo = "safe-clean";
      rev = "main"; # or a specific commit hash
      sha256 = ""; # Run once to get the correct hash
    }) {})
  ];
}
```

After modifying your configuration:
```bash
sudo nixos-rebuild switch
```

## Development Environment

### Using the Development Shell

The flake provides a development shell with all necessary tools:

```bash
# Enter the development shell
nix develop github:npsg02/safe-clean

# Or from a local clone
git clone https://github.com/npsg02/safe-clean.git
cd safe-clean
nix develop
```

The development shell includes:
- Rust stable toolchain
- cargo-watch for automatic rebuilds
- rust-analyzer for IDE support
- pkg-config and other build dependencies

### Building from Source with Nix

```bash
git clone https://github.com/npsg02/safe-clean.git
cd safe-clean
nix build
./result/bin/safe-clean
```

The built binary will be available in the `result` symlink.

### Direnv Integration

For automatic environment loading, create a `.envrc` file in your clone:

```bash
use flake
```

Then run:
```bash
direnv allow
```

Now the development environment will be automatically loaded when you enter the directory.

## Pinning to a Specific Version

To use a specific version or commit:

```bash
# Using a specific tag
nix run github:npsg02/safe-clean/v0.1.0

# Using a specific commit
nix run github:npsg02/safe-clean/abc1234
```

## Troubleshooting

### Flakes Not Enabled

If you get an error about experimental features:
```
error: experimental Nix feature 'flakes' is disabled
```

Enable flakes by adding to `~/.config/nix/nix.conf`:
```
experimental-features = nix-command flakes
```

### Build Failures

If the build fails, try updating your nixpkgs:
```bash
nix flake update
```

### Binary Not Found

After installation, if the binary is not found, ensure your PATH includes:
- `~/.nix-profile/bin` (for user installations)
- `/run/current-system/sw/bin` (for NixOS system installations)

## Advanced Usage

### Using in Other Nix Expressions

You can use `safe-clean` as a dependency in your own Nix projects:

```nix
{
  inputs = {
    safe-clean.url = "github:npsg02/safe-clean";
  };

  outputs = { self, safe-clean, ... }: {
    # Your outputs here
    packages.x86_64-linux.my-package = pkgs.stdenv.mkDerivation {
      buildInputs = [ safe-clean.packages.x86_64-linux.default ];
    };
  };
}
```

### Overlays

To add `safe-clean` to your nixpkgs overlay:

```nix
{ config, pkgs, ... }:

{
  nixpkgs.overlays = [
    (final: prev: {
      safe-clean = prev.callPackage (builtins.fetchGit {
        url = "https://github.com/npsg02/safe-clean.git";
        ref = "main";
      }) {};
    })
  ];

  environment.systemPackages = [ pkgs.safe-clean ];
}
```

## Getting Help

For issues specific to Nix packaging:
- Open an issue at https://github.com/npsg02/safe-clean/issues
- Check the Nix documentation at https://nixos.org/manual/nix/stable/

For general `safe-clean` usage:
- See the main README: https://github.com/npsg02/safe-clean
- Visit the documentation: https://npsg02.github.io/safe-clean/
