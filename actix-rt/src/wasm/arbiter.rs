use futures::{future, Async, Future, IntoFuture, Poll, Stream};
use futures::sync::oneshot::{channel, Canceled, Sender};


#[ derive( Clone, Debug ) ]
//
pub struct Arbiter
{

}


impl Arbiter
{
	pub fn new() -> Self
	{
		Self{}
	}

	pub fn spawn<F>(_future: F)
	where
	  F: Future<Item = (), Error = ()> + 'static,
	{

	}


	// instance methods


	/// Send a function to the arbiter's thread, exeute and return result.
	///
	pub fn exec<F, R>(&self, _f: F) -> impl Future<Item = R, Error = Canceled>
	where
	    F: FnOnce() -> R + Send + 'static,
	    R: Send + 'static,
	{
	    let (_tx, rx) = channel();


	    // let _ = self
	    //     .0
	    //     .unbounded_send(ArbiterCommand::ExecuteFn(Box::new(move || {
	    //         if !tx.is_canceled() {
	    //             let _ = tx.send(f());
	    //         }
	    //     })));
	    rx
	}

	pub fn exec_fn<F>(&self, _f: F)
	where
	    F: FnOnce() + Send + 'static,
	{
	}
}
