{
  description = "no";
  inputs.nixpkgs.url = github:NixOS/nixpkgs;
  outputs = { self, nixpkgs }:
    let pkgs = import nixpkgs { system = "x85_64-linux"; }; in
    {
      devShell = import ./shell.nix { inherit pkgs; };
    };
}
# {
#   description = "no";
#   inputs.nixpkgs.url = github:NixOS/nixpkgs;
#   inputs.flake-utils.url = github:numtide/flake-utils;
#   outputs = { self, nixpkgs, flake-utils }:
#   flake-utils.lib.eachDefaultSystem (system:
#     let pkgs = nixpkgs.legacyPackages.${system}; in
#     {
#       devShell = import ./shell.nix { inherit pkgs; };
#     }
#   );
# }
