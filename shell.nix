# shell.nix - For traditional nix-shell users
# For flake users, use: nix develop
(import (
  fetchTarball {
    url = "https://github.com/edolstra/flake-compat/archive/master.tar.gz";
  }
) {
  src = ./.;
}).shellNix
