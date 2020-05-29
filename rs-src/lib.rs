#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_transmute)]
#![feature(extern_types)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]


#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;



pub mod src {
pub mod lib {
pub mod framework {
pub mod SDL_framerate;
pub mod block;
pub mod configfile;
pub mod debug;
pub mod frame;
pub mod frameresource;
pub mod heap;
pub mod ignorecase;
pub mod input;
pub mod mem;
pub mod resource_lexer;
pub mod resource_parser;
pub mod strres;
pub mod strres_lexer;
pub mod strres_parser;
pub mod treap;
pub mod trig;
} // mod framework
pub mod gamelib {
pub mod anim;
pub mod animobj;
pub mod audp_lexer;
pub mod audp_parser;
pub mod gtime;
pub mod hashtabl;
pub mod pqueue;
pub mod ptrlist;
} // mod gamelib
pub mod ivis_common {
pub mod bitimage;
pub mod bug;
pub mod imd;
pub mod imdload;
pub mod pcx;
pub mod pieclip;
pub mod piestate;
} // mod ivis_common
pub mod ivis_opengl {
pub mod bspimd;
pub mod ivi;
pub mod pieblitfunc;
pub mod piedraw;
pub mod piefunc;
pub mod piematrix;
pub mod piemode;
pub mod piepalette;
pub mod piestate;
pub mod pietexture;
pub mod rendfunc;
pub mod rendmode;
pub mod screen;
pub mod tex;
pub mod textdraw;
} // mod ivis_opengl
pub mod netplay {
pub mod netaudio_stub;
pub mod netcrypt;
pub mod netjoin_stub;
pub mod netlog;
pub mod netplay;
pub mod netusers_stub;
} // mod netplay
pub mod script {
pub mod codeprint;
pub mod event;
pub mod evntsave;
pub mod interp;
pub mod script;
pub mod script_lexer;
pub mod script_parser;
pub mod stack;
} // mod script
pub mod sequence {
pub mod adpcm;
pub mod dec130;
pub mod rpl_reader;
pub mod sequence_stub;
} // mod sequence
pub mod sound {
pub mod audio;
pub mod cdaudio;
pub mod mixer_stub;
pub mod openal_track;
pub mod playlist;
pub mod track;
} // mod sound
pub mod widget {
pub mod bar;
pub mod button;
pub mod editbox;
pub mod form;
pub mod label;
pub mod scrap;
pub mod slider;
pub mod tip;
pub mod widget;
} // mod widget
} // mod lib
pub mod src {
pub mod action;
pub mod advvis;
pub mod ai;
pub mod aiexperience;
pub mod ani;
pub mod arrow;
pub mod astar;
pub mod atmos;
pub mod aud;
pub mod audio_id;
pub mod bridge;
pub mod bucket3d;
pub mod buildpos;
pub mod cdspan;
pub mod cheat;
pub mod clparse;
pub mod cluster;
pub mod cmddroid;
pub mod combat;
pub mod component;
pub mod configuration;
pub mod console;
pub mod csnap;
pub mod data;
pub mod design;
pub mod difficulty;
pub mod disp2d;
pub mod display;
pub mod display3d;
pub mod drive;
pub mod droid;
pub mod e3demo;
pub mod edit2d;
pub mod edit3d;
pub mod effects;
pub mod environ;
pub mod feature;
pub mod findpath;
pub mod formation;
pub mod fpath;
pub mod frontend;
pub mod function;
pub mod game;
pub mod gateway;
pub mod gatewayroute;
pub mod gatewaysup;
pub mod geometry;
pub mod group;
pub mod hci;
pub mod ingameop;
pub mod init;
pub mod intdisplay;
pub mod intelmap;
pub mod intimage;
pub mod intorder;
pub mod keybind;
pub mod keyedit;
pub mod keymap;
pub mod level_lexer;
pub mod levels;
pub mod lighting;
pub mod loadsave;
pub mod map;
pub mod mapdisplay;
pub mod mapgrid;
pub mod mechanics;
pub mod message;
pub mod miscimd;
pub mod mission;
pub mod multibot;
pub mod multigifts;
pub mod multiint;
pub mod multijoin;
pub mod multilimit;
pub mod multimenu;
pub mod multiopt;
pub mod multiplay;
pub mod multistat;
pub mod multistruct;
pub mod multisync;
pub mod objects;
pub mod objmem;
pub mod oprint;
pub mod optimisepath;
pub mod order;
pub mod player;
pub mod power;
pub mod powercrypt;
pub mod projectile;
pub mod r#loop;
pub mod r#move;
pub mod radar;
pub mod raycast;
pub mod research;
pub mod scores;
pub mod scriptai;
pub mod scriptcb;
pub mod scriptextern;
pub mod scriptfuncs;
pub mod scriptobj;
pub mod scripttabs;
pub mod scriptvals;
pub mod scriptvals_lexer;
pub mod scriptvals_parser;
pub mod selection;
pub mod seqdisp;
pub mod stats;
pub mod structure;
pub mod target;
pub mod text;
pub mod texture;
pub mod transporter;
pub mod version;
pub mod visibility;
pub mod warcam;
pub mod warzoneconfig;
pub mod wrappers;
} // mod src
} // mod src

