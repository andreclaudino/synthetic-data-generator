use std::{collections::HashMap};

use super::{Schema, dataset::Dataset};

impl Schema {
	
	pub fn generate(&self, sample_size: usize) -> anyhow::Result<Dataset> {
		log::info!("Generating rows");

		let samples: HashMap<String, Vec<serde_json::Value>> =
			self.fields.iter().map(|schema_field|{
				let field = schema_field.name.clone();
				log::debug!("Generating column {field}");
				schema_field.generate(sample_size)
			}).collect();

		log::info!("{sample_size} rows generated");
		let dataset = Dataset::new(sample_size, samples);
		Ok(dataset)
	}
}