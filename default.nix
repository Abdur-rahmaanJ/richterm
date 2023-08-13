let
    pkgs = import <nixpkgs> {};
in
pkgs.mkShell {
  name = "richterm-devshell";
  propagatedBuildInputs = with pkgs; [
    rustup
    cargo-make
    cargo
  ];
  src = null;
  shellHook = with pkgs; ''
   export PATH=$HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/:$PATH
   alias q=exit
  '';
}
