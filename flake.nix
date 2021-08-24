{
  description = "sort lines by their similarity to a candidate string";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=release-21.05";
    gitignore = {
      url = "github:hercules-ci/gitignore";
      flake = false;
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs:
    inputs.flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = inputs.nixpkgs.legacyPackages.${system};
        gitignore = pkgs.callPackage inputs.gitignore { };
      in rec {
        # `nix build`
        packages.similar-sort = pkgs.stdenv.mkDerivation {
          name = "similar-sort";
          buildInputs = [ pkgs.go ];
          src = gitignore.gitignoreSource ./.;

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
          inputs.flake-utils.lib.mkApp { drv = packages.similar-sort; };
        defaultApp = apps.similar-sort;

        # `nix develop`
        devShell = pkgs.mkShell { nativeBuildInputs = [ pkgs.go ]; };
      });
}
