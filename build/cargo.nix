```nix
# cargo.nix — воспроизводимая сборка через Nix
# Используйте: nix build -f build/cargo.nix

{ pkgs ? import <nixpkgs> {} }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "ontocoder-public";
  version = "2.0.0-aenga-public";

  src = pkgs.lib.cleanSource ./.;

  # Зависимости Cargo
  cargoSha256 = "sha256-XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"; # ← обновите после cargo update

  # Системные зависимости (для Go-компонентов sgRL)
  nativeBuildInputs = with pkgs; [
    pkg-config
    clang
    go
  ];

  # Патч для отключения сетевых вызовов во время сборки
  cargoBuildFlags = [ "--workspace" ];

  # Установка артефактов
  installPhase = ''
    runHook preInstall
    mkdir -p $out/bin
    cp target/release/ontoc $out/bin/
    cp target/release/onto-runtime $out/bin/
    cp target/release/ontoreg $out/bin/
    runHook postInstall
  '';

  meta = with pkgs.lib; {
    description = "AENGA-guaranteed ontological compiler";
    license = licenses.mpl20;
    maintainers = [ ];
    platforms = platforms.all;
  };
}
```