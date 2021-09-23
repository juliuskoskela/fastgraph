///////////////////////////////////////////////////////////////////////////////
///
/// INCLUDES

use std:: {
	fmt:: {
		Debug,
		Display,
	},
	hash::Hash,
};

use crate::global::*;
use rayon::prelude::*;

///
///////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////
///
/// EdgeList

#[derive(Debug, Clone)]
pub struct EdgeList<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	pub list: Vec<EdgeRef<K, N, E>>,
	iterator: usize,
}

///////////////////////////////////////////////////////////////////////////////
///
/// EdgeList: Implementations

impl<K, N, E> EdgeList<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
	pub fn new() -> Self {
		Self {
			list: Vec::new(),
			iterator: 0,
		}
	}

	pub fn add(&mut self, edge: EdgeRef<K, N, E>) {
		self.list.push(edge);
	}

	pub fn find_index(&self, edge: EdgeRef<K, N, E>) -> Option<usize> {
		let mut i = 0;
		for e in self.list.iter() {
			if e.target() == edge.target() && e.source() == edge.source() {
				return Some(i);
			}
			i += 1;
		}
		None
	}

	pub fn iter(&self) -> std::slice::Iter<EdgeRef<K, N, E>> {
		self.list.iter()
	}

	pub fn par_iter(&self) -> rayon::slice::Iter<EdgeRef<K, N, E>> {
		self.list.par_iter()
	}

	pub fn del(&mut self, edge: EdgeRef<K, N, E>) {
		let index = self.find_index(edge);
		match index {
			Some(i) => {
				self.list.remove(i);
			},
			None => return,
		}
	}

	pub fn del_index(&mut self, index: usize) {
		if index < self.list.len() {
			self.list.remove(index);
		}
	}

	pub fn open_all(&self) -> &Self {
		for edge in self.list.iter() {
			edge.open();
			edge.target().open();
			edge.source().open();
		}
		self
	}

	pub fn open_nodes(&self) -> &Self {
		for edge in self.list.iter() {
			edge.target().open();
			edge.source().open();
		}
		self
	}

	pub fn open_edges(&self) -> &Self {
		for edge in self.list.iter() {
			edge.open();
		}
		self
	}

	pub fn close_all(&self) -> &Self {
		for edge in self.list.iter() {
			edge.close();
			edge.target().close();
			edge.source().close();
		}
		self
	}

	pub fn close_nodes(&self) -> &Self {
		for edge in self.list.iter() {
			edge.target().close();
			edge.source().close();
		}
		self
	}

	pub fn close_edges(&self) -> &Self {
		for edge in self.list.iter() {
			edge.close();
		}
		self
	}

	pub fn backtrack(&self) -> Option<EdgeList<K, N, E>> {
		if self.list.len() == 0 {
			return None;
		}
		let mut res = EdgeList::new();
		res.add(self.list[self.list.len() - 1].clone());
		let mut i = 0;
		for edge in self.list.iter().rev() {
			edge.open();
			edge.target().open();
			edge.source().open();
			let source = &res.list[i].source();
			if edge.target() == *source {
				res.list.push(edge.clone());
				i += 1;
			}
		}
		Some(res)
	}
}

///////////////////////////////////////////////////////////////////////////////
