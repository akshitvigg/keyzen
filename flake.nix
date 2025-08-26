{
  description = "A terminal-based typing test game written in Rust.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = cargoToml.package.name;
          inherit (cargoToml.package) version;
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          meta = {
            inherit (cargoToml.package) description;
            inherit (cargoToml.package) license;
          };

          nativeBuildInputs = with pkgs; [ pkg-config ];

          buildInputs = with pkgs; [ alsa-lib ];
        };
      }
    );
}
