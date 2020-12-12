use super::*;
use gl_safe::*;
use std::marker::PhantomData;

pub struct Buffer<T: Sized + Copy + 'static> {
	handle: GLuint,
	len: u32,
	_type: PhantomData<T>,
}

impl<T> Buffer<T>
where
	T: Sized + Copy + 'static,
{
	pub fn new(data: &[T], flags: GLbitfield) -> Self {
		let mut s = Self::create();
		s.storage(data, flags);
		s
	}

	/// Create a buffer object.
	/// http://docs.gl/gl4/glCreateBuffers
	pub fn create() -> Self {
		Self {
			handle: glCreateBuffer(),
			len: 0,
			_type: PhantomData,
		}
	}

	/// Creates and initializes a buffer object's immutable data store.
	/// http://docs.gl/gl4/glBufferStorage
	pub fn storage(&mut self, data: &[T], flags: GLbitfield) {
		glNamedBufferStorage(self.handle, data, flags);
		self.len = data.len() as u32;
	}

	/// Returns the number of elements in the buffer.
	pub fn len(&self) -> u32 {
		self.len
	}

	pub fn handle(&self) -> GLuint {
		self.handle
	}

	/// Copy the buffer's contents into `data`,
	/// which must match in size.
	pub fn copy_data(&self, data: &mut [T]) {
		if data.len() != self.len() as usize {
			panic!("Buffer::get_data: size mismatch: buffer len {} != argument len {}", self.len(), data.len())
		}
		glGetNamedBufferSubData(self.handle, 0 /*offset*/, data)
	}

	/// Returns a copy the buffer's contents.
	pub fn get_data(&self) -> Vec<T> {
		let mut data = Vec::<T>::with_capacity(self.len() as usize);
		unsafe { data.set_len(self.len() as usize) };
		glGetNamedBufferSubData(self.handle, 0 /*offset*/, &mut data);
		data
	}
}

impl<T> Drop for Buffer<T>
where
	T: Sized + Copy + 'static,
{
	fn drop(&mut self) {
		println!("TODO: should we be dropping buffer {}?", self.handle);
		//glDeleteBuffer(self.handle)
	}
}

impl<T> Into<GLuint> for Buffer<T>
where
	T: Sized + Copy + 'static,
{
	fn into(self) -> GLuint {
		self.handle
	}
}
