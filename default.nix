{ lib
, rustPlatform
, pkg-config
, stdenv
}:

rustPlatform.buildRustPackage rec {
  pname = "safe-clean";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [
    pkg-config
  ];

  meta = with lib; {
    description = "Safe disk cleanup CLI/TUI tool";
    homepage = "https://github.com/npsg02/safe-clean";
    license = licenses.mit;
    maintainers = [ ];
    mainProgram = "safe-clean";
  };
}
