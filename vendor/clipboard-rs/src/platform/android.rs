use crate::{
	common::Result, Clipboard, ClipboardContent, ClipboardHandler, ClipboardWatcher, ContentFormat,
};

pub struct ClipboardContext;

impl ClipboardContext {
	pub fn new() -> Result<Self> {
		Err("Android clipboard is not supported yet".into())
	}
}

impl Clipboard for ClipboardContext {
	fn available_formats(&self) -> Result<Vec<String>> {
		Err("Android clipboard is not supported yet".into())
	}

	fn has(&self, _format: ContentFormat) -> bool {
		false
	}

	fn clear(&self) -> Result<()> {
		Err("Android clipboard is not supported yet".into())
	}

	fn get_buffer(&self, _format: &str) -> Result<Vec<u8>> {
		Err("Android clipboard is not supported yet".into())
	}

	fn get_text(&self) -> Result<String> {
		Err("Android clipboard is not supported yet".into())
	}

	fn get_rich_text(&self) -> Result<String> {
		Err("Android clipboard is not supported yet".into())
	}

	fn get_html(&self) -> Result<String> {
		Err("Android clipboard is not supported yet".into())
	}

	#[cfg(feature = "image")]
	fn get_image(&self) -> Result<crate::common::RustImageData> {
		Err("Android clipboard is not supported yet".into())
	}

	fn get_files(&self) -> Result<Vec<String>> {
		Err("Android clipboard is not supported yet".into())
	}

	fn get(&self, _formats: &[ContentFormat]) -> Result<Vec<ClipboardContent>> {
		Err("Android clipboard is not supported yet".into())
	}

	fn set_buffer(&self, _format: &str, _buffer: Vec<u8>) -> Result<()> {
		Err("Android clipboard is not supported yet".into())
	}

	fn set_text(&self, _text: String) -> Result<()> {
		Err("Android clipboard is not supported yet".into())
	}

	fn set_rich_text(&self, _text: String) -> Result<()> {
		Err("Android clipboard is not supported yet".into())
	}

	fn set_html(&self, _html: String) -> Result<()> {
		Err("Android clipboard is not supported yet".into())
	}

	#[cfg(feature = "image")]
	fn set_image(&self, _image: crate::common::RustImageData) -> Result<()> {
		Err("Android clipboard is not supported yet".into())
	}

	fn set_files(&self, _files: Vec<String>) -> Result<()> {
		Err("Android clipboard is not supported yet".into())
	}

	fn set(&self, _contents: Vec<ClipboardContent>) -> Result<()> {
		Err("Android clipboard is not supported yet".into())
	}
}

pub struct ClipboardWatcherContext;

impl ClipboardWatcherContext {
	pub fn new() -> Result<Self> {
		Err("Android clipboard watcher is not supported yet".into())
	}

	pub fn add_handler<T: ClipboardHandler>(&mut self, _handler: T) -> &mut Self {
		self
	}

	pub fn start_watch(&mut self) {
		// no-op
	}

	pub fn get_shutdown_channel(&self) -> WatcherShutdown {
		WatcherShutdown
	}
}

impl<T: ClipboardHandler> ClipboardWatcher<T> for ClipboardWatcherContext {
	fn add_handler(&mut self, _handler: T) -> &mut Self {
		ClipboardWatcherContext::add_handler(self, _handler)
	}

	fn start_watch(&mut self) {
		ClipboardWatcherContext::start_watch(self)
	}

	fn get_shutdown_channel(&self) -> WatcherShutdown {
		ClipboardWatcherContext::get_shutdown_channel(self)
	}
}

pub struct WatcherShutdown;

impl Drop for WatcherShutdown {
	fn drop(&mut self) {
		// no-op
	}
}
