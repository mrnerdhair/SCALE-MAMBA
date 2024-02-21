{ sources ? import ./sources.nix }:

let
  pkgs =
    import sources.nixpkgs { overlays = [ (import sources.nixpkgs-mozilla) ]; };
  channel = "nightly";
  date = "2024-02-20";
  targets = [ "wasm32-unknown-unknown" ];
  chan = pkgs.rustChannelOfTargets channel date targets;
in chan
