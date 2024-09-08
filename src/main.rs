use std::fs::File;
use std::io::Write;
use std::path::Path;

use promptuity::prompts::Input;
use promptuity::themes::FancyTheme;
use promptuity::{Error, Promptuity, Term};

use quickmd::config::read_from_json;
use quickmd::frontmatter::{
    extract_frontmatter_value_with_prompt, generate_frontmatter_format_yaml, FrontmatterValue,
};

fn main() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;

    let config = read_from_json().unwrap();
    let ext = config.ext;
    let output_path = config.output_path;
    let frontmatter_fields = config.frontmatter_fields;

    p.with_intro("You can start writing quickly from here.")
        .begin()?;

    let filename = p.prompt(Input::new("Please enter `filename`").with_placeholder("filename"))?;
    let mut frontmatter_values = Vec::<FrontmatterValue>::with_capacity(frontmatter_fields.len());

    if frontmatter_fields.len() > 0 {
        p.step("Please fill in the frontmatter fields.")?;

        // Iterate over the frontmatter fields and prompt the user for input
        for field in &frontmatter_fields {
            let extracted_value = extract_frontmatter_value_with_prompt(&mut p, field).unwrap();
            frontmatter_values.push(extracted_value);
        }
    }

    let path: std::path::PathBuf =
        Path::new(&(format!("{}/{}", output_path, filename))).with_extension(&ext);
    let display = path.display();

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}: {}", display, why),
    };

    if frontmatter_fields.len() > 0 {
        let frontmatter = generate_frontmatter_format_yaml(&frontmatter_values);
        match file.write_all(frontmatter.as_bytes()) {
            Ok(()) => (),
            Err(why) => panic!("couldn't write to {}: {}", display, why),
        }
    }

    p.with_outro(format!(
        "Successfully generated {}.{}🎉 Happy writing!",
        filename, ext
    ))
    .finish()?;

    Ok(())
}
