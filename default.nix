with import <nixpkgs> {};

pkgsi686Linux.clangStdenv.mkDerivation {
  pname = "warzone";
  version = "0.1";
  preConfigure= "./autogen.sh";
  nativeBuildInputs = [ pkgsi686Linux.pkgconfig ];

  buildInputs = with pkgsi686Linux; [
    # Rust
    llvm clang clang-tools cmake openssl python37 rustup rustfmt rustc cargo
    llvmPackages.clang-unwrapped
    python37Packages.pip llvmPackages.libclang
    autoconf automake bison flex zlib libjpeg libpng physfs pkgconfig
    xorg.libX11
    zip unzip
    xorg.libXrandr
    SDL libtheora openal SDL_net
    glew fribidi 
    glibc.dev
    SDL.dev
    libglvnd
    xorg.xorgproto
  ];

  INCLUDES = with pkgsi686Linux; map (x: " -I " + x)
    [ (glibc.dev + "/include/") (SDL.dev + "/include/") (openal + "/include/") (xorg.libX11.dev + "/include/") (physfs + "/include/") (libglvnd.dev + "/include/") (xorg.xorgproto + "/include/") ];
  LDFLAGS = "-lX11 -L" + pkgsi686Linux.llvmPackages.clang-unwrapped + "/lib/";
  LIBCLANG_PATH = pkgsi686Linux.llvmPackages.libclang + "/lib/";
  #RUST_BACKTRACE = 1;

  src = if lib.inNixShell then null else ./.;
  hardeningDisable = [ "all" ];
  enableParallelBuilding = true;
}