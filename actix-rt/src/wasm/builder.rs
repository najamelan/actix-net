pub struct Builder{}

impl Builder
{
	pub(crate) fn new() -> Self
	{
		Builder {}
	}


	/// This function will start tokio runtime and will finish once the
	/// `System::stop()` message get called.
	/// Function `f` get called within tokio runtime context.
	pub fn run<F>(self, f: F) -> std::io::Result<()>
	where
	    F: FnOnce() + 'static,
	{
	    self.create_runtime(f).run()
	}


	fn create_runtime<F>(self, _f: F) -> SystemRunner
	where
	    F: FnOnce() + 'static,
	{
	    SystemRunner {  }
	}
}



pub struct SystemRunner{}


impl SystemRunner
{
	pub fn run( &self ) -> std::io::Result<()>
	{
		Ok(())
	}
}
