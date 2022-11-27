use command_line::CommandLine;
use persistence::{load_schema, save};

mod entities;
mod persistence;
mod command_line;


fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = CommandLine::load();

    let schema_path = args.schema_path;
    let sample_size = args.sample_size;

    let schema = load_schema(&schema_path)?;
    let generated = schema.generate(sample_size)?;

    save(&generated, &args.output_path)?;
    
    Ok(())
}
