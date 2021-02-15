{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs {
         inherit system;
         overlays = [
           rust-overlay.overlay
           (self: super: {
             rustc = self.latest.rustChannels.nightly.rust;
             cargo = self.latest.rustChannels.nightly.rust;
           })
         ];
       };
      naersk-lib = naersk.lib."${system}".override {
        cargo = pkgs.cargo;
        rustc = pkgs.rustc;
      };
    in rec {
      packages.filehost = naersk-lib.buildPackage {
        pname = "filehost";
        root = ./.;
        /*buildInputs = with pkgs; [];*/
      };
      defaultPackage = packages.filehost;

      apps.filehost = utils.lib.mkApp {
        drv = packages.filehost;
      };
      defaultApp = apps.filehost;

      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ rustc cargo ];
      };
    });
}
