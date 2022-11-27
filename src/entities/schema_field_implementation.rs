use rand::{thread_rng, prelude::Distribution};
use statrs::{distribution::{Bernoulli, Binomial, DiscreteUniform, Normal, Uniform}};
use super::{SchemaField, distribution::distribution::DistributionDefinition};
use indicatif::{ProgressStyle, ProgressIterator};


impl SchemaField {

	pub fn generate(&self, samples: usize) -> (String, Vec<serde_json::Value>) {
		let field_name = self.name.clone();
		let distribution = self.distribution.as_ref().unwrap_or(&self.schema.default_distribution()).clone();
		let mut rng = thread_rng();

		println!("Generating field: {field_name}");
		
		let progressbar_style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({eta})").unwrap();

		let samples = match distribution {
			DistributionDefinition::Bernoulli { p } => {
				let distribution = Bernoulli::new(p).unwrap();
				distribution.sample_iter(&mut rng).progress_count(samples as u64).with_style(progressbar_style).take(samples).map(serde_json::Value::from).collect()
			},
			DistributionDefinition::Binomial { p, n } => {
				let distribution = Binomial::new(p, n).unwrap();
				distribution.sample_iter(&mut rng).progress_count(samples as u64).with_style(progressbar_style).take(samples).map(serde_json::Value::from).collect()
			},
			DistributionDefinition::Normal { mean, std_dev } => {
				let distribution = Normal::new(mean, std_dev).unwrap();
				distribution.sample_iter(&mut rng).progress_count(samples as u64).with_style(progressbar_style).take(samples).map(serde_json::Value::from).collect()
			},
			DistributionDefinition::DiscreteUniform { min, max } => {
				let distribution = DiscreteUniform::new(min, max).unwrap();
				distribution.sample_iter(&mut rng).progress_count(samples as u64).with_style(progressbar_style).take(samples).map(serde_json::Value::from).collect()
			},
			DistributionDefinition::Uniform { min, max } => {
				let distribution = Uniform::new(min, max).unwrap();
				distribution.sample_iter(&mut rng).progress_count(samples as u64).with_style(progressbar_style).take(samples).map(serde_json::Value::from).collect()
			},
		};

		(field_name, samples)
	}

}
