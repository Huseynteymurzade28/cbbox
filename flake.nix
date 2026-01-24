{
  description = "NixOS Rust/SDL2 Dev Environment";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {

        nativeBuildInputs = with pkgs; [ pkg-config ];


        buildInputs = with pkgs; [
          SDL2
          SDL2_image
          SDL2_ttf
          wayland 
          libGL
        ];

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
          SDL2
          SDL2_image
          SDL2_ttf
          libGL
          wayland
        ]);
      };
    };
}