use std::fs::File;
use std::io::Write;
use std::path::Path;

use promptuity::prompts::Input;
use promptuity::themes::FancyTheme;
use promptuity::{Error, Promptuity, Term};

use quickmd::frontmatter::{generate_frontmatter, FrontmatterValue};
use quickmd::config::read_from_json;

fn main() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;

    let config = read_from_json().unwrap();
    let ext = config.ext;
    let output_path = config.output_path;
    let frontmatter_fields = config.frontmatter;

    p.with_intro("You can start writing quickly from here.").begin()?;

    let filename = p.prompt(Input::new("Please enter `filename`").with_placeholder("filename"))?;
    let mut frontmatter_values = Vec::<FrontmatterValue>::with_capacity(frontmatter_fields.len());

    if frontmatter_fields.len() > 0 {
        p.step("Please fill in the frontmatter fields.")?;
    }

    for field in &frontmatter_fields {
        let value = p.prompt(Input::new(&format!("Please enter `{}`", field.name)).with_placeholder(field.placeholder.clone()).with_required(field.required))?;
        frontmatter_values.push(FrontmatterValue {
            name: field.name.clone(),
            value,
        });
    }

    let path = Path::new(&(format!("{}/{}", output_path, filename))).with_extension(&ext);
    let display = path.display();

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}: {}", display, why),
    };

    if frontmatter_fields.len() > 0 {
        let frontmatter = generate_frontmatter(frontmatter_values);
        match file.write_all(frontmatter.as_bytes()) {
            Ok(()) => (),
            Err(why) => panic!("couldn't write to {}: {}", display, why),
        }
    }

    p.with_outro(format!("Successfully created {}.{}🎉 Happy writing!", filename, ext)).finish()?;

    Ok(())
}

