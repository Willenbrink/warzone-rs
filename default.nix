with import <nixpkgs> {};

pkgsi686Linux.stdenv.mkDerivation {
  pname = "warzone";
  version = "0.1";
  preConfigure= "./autogen.sh";
  nativeBuildInputs = [ pkgsi686Linux.pkgconfig ];

  buildInputs = with pkgsi686Linux; [
    autoconf automake bison flex zlib libjpeg libpng physfs pkgconfig
    xorg.libX11
    xorg.libXrandr
    SDL libtheora openal SDL_net
    glew fribidi 
  ];

  hardeningDisable = [ "all" ];

  #configureFlags = [ "--libs" "x11" "xrandr" "xi" "xxf86vm" "glew" "glfw3"];
  #buildFlags = [ "-lX11" ];

  src = if lib.inNixShell then null else ./.;

  enableParallelBuilding = true;
}