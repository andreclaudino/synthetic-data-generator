use crate::entities::DataType;
use super::distribution::DistributionDefinition;

impl DataType {
	
	pub fn default_distribution(&self) -> DistributionDefinition {
		match &self {
			DataType::Float { minimum, maximum } => {
				DistributionDefinition::Uniform { min: *minimum, max: *maximum }
			},
			DataType::Integer { minimum, maximum } => {
				DistributionDefinition::DiscreteUniform { min: *minimum, max: *maximum }
			}
		}
	}
}