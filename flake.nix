{
  description = "sort lines by their similarity to a candidate string";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/release-21.05";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        naersk-lib = naersk.lib.${system};
      in rec {
        # `nix build`
        packages.similar-sort = naersk-lib.buildPackage ./.;
        defaultPackage = packages.similar-sort;

        overlay = final: prev: {
          similar-sort = packages.similar-sort;
        };

        # `nix run`
        apps.similar-sort =
          flake-utils.lib.mkApp { drv = packages.similar-sort; };
        defaultApp = apps.similar-sort;

        # `nix develop`
        devShell = pkgs.mkShell {
          nativeBuildInputs = [
            pkgs.cargo
            pkgs.cargo-edit
            pkgs.cargo-watch
            pkgs.rustPackages.clippy
            pkgs.rustc
            pkgs.rustfmt

            # for some reason this is always needed when building Rust stuff, at
            # least on macOS
            pkgs.libiconv
          ];
        };
      });
}
