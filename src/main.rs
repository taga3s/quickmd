use std::fs::File;
use std::io::Write;
use std::path::Path;

use promptuity::prompts::{Confirm, Input, MultiSelect, MultiSelectOption, Select, SelectOption};
use promptuity::themes::FancyTheme;
use promptuity::{Error, Promptuity, Term};

use quickmd::config::read_from_json;
use quickmd::frontmatter::{generate_frontmatter, FrontmatterValue};

fn main() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;

    let config = read_from_json().unwrap();
    let ext = config.ext;
    let output_path = config.output_path;
    let frontmatter_fields = config.frontmatter;

    p.with_intro("You can start writing quickly from here.")
        .begin()?;

    let filename = p.prompt(Input::new("Please enter `filename`").with_placeholder("filename"))?;
    let mut frontmatter_values = Vec::<FrontmatterValue>::with_capacity(frontmatter_fields.len());

    if frontmatter_fields.len() > 0 {
        p.step("Please fill in the frontmatter fields.")?;

        // Iterate over the frontmatter fields and prompt the user for input
        for field in &frontmatter_fields {
            if field.field_type == "text" {
                let value = p.prompt(
                    Input::new(&format!("Please enter `{}`", field.name))
                        .with_placeholder(field.placeholder.clone())
                        .with_required(field.required),
                )?;
                frontmatter_values.push(FrontmatterValue {
                    name: field.name.clone(),
                    value,
                });
                continue;
            }

            if field.field_type == "boolean" {
                let value = p.prompt(
                    Confirm::new(format!("Please confirm `{}`", field.name)).with_default(true),
                )?;
                frontmatter_values.push(FrontmatterValue {
                    name: field.name.clone(),
                    value: value.to_string(),
                });
                continue;
            }

            if field.field_type == "select" {
                let mut options = Vec::<SelectOption<&str>>::with_capacity(field.options.len());
                for option in &field.options {
                    options.push(SelectOption::new(option, option));
                }

                let value = p.prompt(Select::new(field.question.clone(), options).as_mut())?;

                frontmatter_values.push(FrontmatterValue {
                    name: field.name.to_owned(),
                    value: value.to_owned(),
                });
                continue;
            }

            if field.field_type == "multiselect" {
                let mut options =
                    Vec::<MultiSelectOption<&str>>::with_capacity(field.options.len());
                for option in &field.options {
                    options.push(MultiSelectOption::new(option, option));
                }

                let value = p.prompt(MultiSelect::new(field.question.clone(), options).as_mut())?;

                frontmatter_values.push(FrontmatterValue {
                    name: field.name.to_owned(),
                    value: format!("[{}]", value.join(", ")),
                });
                continue;
            }
        }
    }

    let path: std::path::PathBuf = Path::new(&(format!("{}/{}", output_path, filename))).with_extension(&ext);
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

    p.with_outro(format!(
        "Successfully generated {}.{}🎉 Happy writing!",
        filename, ext
    ))
    .finish()?;

    Ok(())
}
