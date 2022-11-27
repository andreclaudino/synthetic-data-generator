mod schema_implementation;
mod schema_field_implementation;
mod distribution;
pub mod dataset;

use serde::{Deserialize};

use self::distribution::distribution::DistributionDefinition;

#[derive(Deserialize, Debug)]
pub struct Schema {
	pub fields: Vec<SchemaField>
}

#[derive(Deserialize, Debug, Clone)]
pub struct SchemaField {
	name: String,
	schema: DataType,
	distribution: Option<DistributionDefinition>
}


#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum DataType {
	Integer{
		minimum: i64,
		maximum: i64
	},
	Float {
		minimum: f64,
		maximum: f64
	}
}
