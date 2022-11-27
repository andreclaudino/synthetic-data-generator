use clap::Parser;


/// Awin feed importer to partioned HIVE structure
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CommandLine {
  /// Path for file schema
  #[arg(short, long)]
  pub schema_path: String,

	/// Path for output file
	#[arg(short, long)]
	pub output_path: String,

	/// Number of samples
	#[arg(long)]
	pub sample_size: usize,

	
}

impl CommandLine {
	pub fn load() -> Self {
		Self::parse()
	}
}