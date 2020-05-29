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
    libglvnd
    xorg.xorgproto
    libGLU
    libogg
    libvorbis
    libpng_apng
  ];

  INCLUDES = with pkgsi686Linux; map (x: " -I " + x + "/include/")
    [ glibc.dev SDL.dev openal xorg.libX11.dev physfs libglvnd.dev xorg.xorgproto libGLU.dev libjpeg.dev SDL_net
      libogg.dev libvorbis.dev libpng_apng.dev
    ];
  LDFLAGS = "-lX11 -L" + pkgsi686Linux.llvmPackages.clang-unwrapped + "/lib/";
  LIBCLANG_PATH = pkgsi686Linux.llvmPackages.libclang + "/lib/";
  #RUST_BACKTRACE = 1;

  src = if lib.inNixShell then null else ./.;
  hardeningDisable = [ "all" ];
  enableParallelBuilding = true;
}