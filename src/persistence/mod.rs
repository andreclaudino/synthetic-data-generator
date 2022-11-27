use std::{fs::File, io::Read, io::Write};
use indicatif::{ProgressStyle, ProgressIterator};
use crate::entities::{Schema, dataset::{Dataset}};


pub fn load_schema(schema_path: &str) -> anyhow::Result<Schema> {
	log::info!("Loading file `{schema_path}`");
	let mut schema_file = File::open(schema_path)?;
	let mut schema_data = String::new();
	schema_file.read_to_string(&mut schema_data)?;

	log::info!("Turning file `{schema_path}` into schema");
	let schema = serde_yaml::from_str::<Schema>(&schema_data)?;

	log::info!("Loading schema finished");
	Ok(schema)
}

pub fn save(generated: &Dataset, output_path: &str) -> anyhow::Result<()> {
	log::info!("Writing output to `{output_path}`");
	let mut file = File::create(output_path)?;

	println!("Saving file {output_path}");
	let progressbar_style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({eta})")?;
	for row in generated.clone().into_iter().progress_with_style(progressbar_style) {
		let line = serde_json::to_value(row)?;
		file.write(line.to_string().as_bytes())?;
		file.write(b"\n")?;
	}
	
	Ok(())
}