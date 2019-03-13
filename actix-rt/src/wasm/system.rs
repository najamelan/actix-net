use
{
	crate :: { Arbiter, Builder, SystemRunner },
	std::cell::RefCell,
};


thread_local!
(
    static CURRENT: RefCell<Option<System>> = RefCell::new( None );
);


/// Spawns a future on the current arbiter.
///
/// # Panics
///
/// This function panics if actix system is not running.
///
pub fn spawn<F>(f: F)
where
    F: futures::Future<Item = (), Error = ()> + 'static,
{
    if !System::is_set() {
        panic!("System is not running");
    }

    Arbiter::spawn(f);
}



#[ derive( Clone, Debug ) ]
//
pub struct System
{
	arbiter: Arbiter,
	id     : usize  ,
}


impl System
{
	#[allow(clippy::new_ret_no_self)]
	/// Create new system.
	///
	/// This method panics if it can not create tokio runtime
	pub fn new<T: Into<String>>( _name: T ) -> SystemRunner
	{
		// Self::builder().name(name).build()

		SystemRunner {}
	}

	pub fn current() -> System
	{
	    CURRENT.with(|cell| match *cell.borrow() {
	        Some(ref sys) => sys.clone(),
	        None => panic!("System is not running"),
	    })
	}

	pub fn with_current<F, R>(f: F) -> R
	where
		F: FnOnce(&System) -> R,
	{
		CURRENT.with(|cell| match *cell.borrow() {
		    Some(ref sys) => f(sys),
		    None => panic!("System is not running"),
		})
	}

	/// Set current running system.
	#[doc(hidden)]
	pub fn set_current(sys: System) {
	    CURRENT.with(|s| {
	        *s.borrow_mut() = Some(sys);
	    })
	}

	pub fn is_set() -> bool
	{
		true
	}

	/// Build a new system with a customized tokio runtime.
	///
	/// This allows to customize the runtime. See struct level docs on
	/// `Builder` for more information.
	pub fn builder() -> Builder {
	    Builder::new()
	}

	// instance methods

	pub fn start( &self )
	{

	}

	pub fn stop( &self )
	{

	}

	pub fn id( &self ) -> usize
	{
		self.id
	}

	/// System arbiter
	pub fn arbiter( &self ) -> &Arbiter
	{
		&self.arbiter
	}

	/// This function will start tokio runtime and will finish once the
	/// `System::stop()` message get called.
	/// Function `f` get called within tokio runtime context.
	///
	pub fn run<F>(f: F) -> std::io::Result<()>
	where
	    F: FnOnce() + 'static,
	{
	    Self::builder().run(f)
	}
}
