{ pkgs }: {
    deps = [
        # Compiler
        pkgs.rustc
        # Project / library manager
        pkgs.cargo
        # Formats your code
        pkgs.rustfmt
        # Lints your code
        pkgs.clippy
    ];
}
