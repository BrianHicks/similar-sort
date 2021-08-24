{
  description = "sort lines by their similarity to a candidate string";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-21.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in rec {
        # `nix build`
        packages.similar-sort = pkgs.stdenv.mkDerivation {
          name = "similar-sort";
          buildInputs = [ pkgs.go ];
          src = ./.;

          buildPhase = ''
            env HOME=$(pwd) GOPATH=$(pwd) go build similar-sort.go
          '';

          installPhase = ''
            mkdir -p $out/bin
            cp similar-sort $out/bin
          '';
        };
        defaultPackage = packages.similar-sort;

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
