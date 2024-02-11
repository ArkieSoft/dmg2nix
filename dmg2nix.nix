{ rustPlatform, lib, fetchCrate, pkgs, stdenv }:

rustPlatform.buildRustPackage rec {
  pname = "dmg2nix";
  version = "0.1.0";
  src = fetchCrate {
    inherit pname version;
    hash = "sha256-zdXSp+/MelAA0KO4utHGclCKLE0lEAysGeKwQZLhoD4=";
  };

  cargoHash = "sha256-zqFum1/bKRgPgT4udhYgEr3OrYkjZGfI8b+lXEu6Sdg=";
  cargoDepsName = pname;
 
  buildInputs = [pkgs.darwin.apple_sdk.frameworks.Security];
}
