#![ allow( unused_imports ) ]

//! A runtime implementation that runs everything on the current thread.

#[ cfg( feature = "tokio" ) ] mod arbiter;
#[ cfg( feature = "tokio" ) ] pub mod blocking;
#[ cfg( feature = "tokio" ) ] mod builder;
#[ cfg( feature = "tokio" ) ] mod runtime;
#[ cfg( feature = "tokio" ) ] mod system;

#[ cfg( feature = "tokio" ) ]
//
pub use
{
	arbiter :: { Arbiter               } ,
	builder :: { Builder, SystemRunner } ,
	runtime :: { Runtime               } ,
	system  :: { System, spawn         } ,
};


// This will export exactly the same things as above.
//
#[ cfg( feature = "wasm" ) ] mod wasm;
#[ cfg( feature = "wasm" ) ] pub use wasm::*;


