use std::{collections::HashMap};

#[derive(Clone)]
pub struct Dataset {
	sample_size: usize,
	pub fields: Vec<String>,
	hash_map: HashMap<String, Vec<serde_json::Value>>,

	current_index: usize,
}

impl Dataset {
	pub fn new(sample_size: usize, samples: HashMap<String, Vec<serde_json::Value>>) -> Dataset {
		let fields = samples.keys().map(|column| column.clone()).collect();
		log::debug!("Creating dataset with {sample_size} rows and columns {fields:?}");
		Dataset { sample_size, fields, hash_map: samples, current_index: 0 }
	}

	pub fn row_at(&self, index: usize) -> Option<HashMap<String, serde_json::Value>> {
		if index >= self.sample_size {
			return None;
		}

		let row = self.fields.iter().map(|field|{
			let column = self.hash_map.get(field).unwrap();
			let value = column.get(index).unwrap();
			(field.clone(), value.clone())
		})
		.collect();

		Some(row)
	}

}

impl Iterator for Dataset {
	type Item = HashMap<String, serde_json::Value>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current_index >= self.sample_size {
			self.current_index = 0;
			return None;
		}

    let current_row = self.row_at(self.current_index)?;
		self.current_index += 1;

		Some(current_row)
  }	
}

impl ExactSizeIterator for Dataset {
	fn len(&self) -> usize {
		self.sample_size
	}
}