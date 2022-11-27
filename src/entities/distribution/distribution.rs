use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum DistributionDefinition {
	Bernoulli{p: f64},
	Binomial{p: f64, n: u64},
	Normal{mean: f64, std_dev: f64},
	DiscreteUniform{min: i64, max: i64},
	Uniform{min: f64, max: f64}
}