with import <nixpkgs> {};

pkgsi686Linux.clangStdenv.mkDerivation {
  pname = "warzone";
  version = "0.1";
  preConfigure= "./autogen.sh";
  nativeBuildInputs = [ pkgsi686Linux.pkgconfig ];

  buildInputs = with pkgsi686Linux; [
    # Rust
    llvm clang clang-tools cmake openssl python37 rustup rustfmt rustc cargo
    python37Packages.pip llvmPackages.libclang
    autoconf automake bison flex zlib libjpeg libpng physfs pkgconfig
    xorg.libX11
    zip unzip
    xorg.libXrandr
    SDL libtheora openal SDL_net
    glew fribidi 
  ];

  hardeningDisable = [ "all" ];

  LDFLAGS = "-lX11 -L" + pkgsi686Linux.llvmPackages.clang-unwrapped + "/lib/";
  #  + "-I" + pkgsi686Linux.glibc.dev + "/";
  LIBCLANG_PATH = pkgsi686Linux.llvmPackages.libclang + "/lib/";
  #LIBRARY_PATH = pkgsi686Linux.glibc + "/lib/";
  #CLANG_LIBDIR_SUFFIX = pkgsi686Linux.glibc.dev + "/";

  #configureFlags = [ "--libs" "x11" "xrandr" "xi" "xxf86vm" "glew" "glfw3"];
  #buildFlags = [ "-lX11" ];

  src = if lib.inNixShell then null else ./.;

  enableParallelBuilding = true;
}