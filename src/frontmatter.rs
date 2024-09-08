use std::io::Stderr;

use promptuity::{
    prompts::{Confirm, Input, MultiSelect, MultiSelectOption, Select, SelectOption},
    Promptuity,
};

use crate::config::FrontmatterField;

pub struct FrontmatterValue {
    pub name: String,
    pub value: String,
}

pub fn extract_frontmatter_value_with_prompt(
    p: &mut Promptuity<'_, Stderr>,
    field: &FrontmatterField,
) -> Result<FrontmatterValue, Box<dyn std::error::Error>> {
    match field.field_type.as_str() {
        "text" => {
            let value = p.prompt(
                Input::new(field.question.clone())
                    .with_placeholder(field.placeholder.clone())
                    .with_required(field.required),
            )?;
            Ok(FrontmatterValue {
                name: field.name.to_owned(),
                value: value.to_owned(),
            })
        }
        "boolean" => {
            let value = p.prompt(Confirm::new(field.question.to_owned()).with_default(true))?;
            Ok(FrontmatterValue {
                name: field.name.to_owned(),
                value: value.to_string(),
            })
        }
        "select" => {
            let mut options = Vec::<SelectOption<&str>>::with_capacity(field.options.len());
            for option in &field.options {
                options.push(SelectOption::new(option, option));
            }

            let value = p.prompt(Select::new(field.question.clone(), options).as_mut())?;
            Ok(FrontmatterValue {
                name: field.name.to_owned(),
                value: value.to_owned(),
            })
        }
        "multiselect" => {
            let mut options = Vec::<MultiSelectOption<&str>>::with_capacity(field.options.len());
            for option in &field.options {
                options.push(MultiSelectOption::new(option, option));
            }

            let value = p.prompt(MultiSelect::new(field.question.clone(), options).as_mut())?;
            Ok(FrontmatterValue {
                name: field.name.to_owned(),
                value: format!("[{}]", value.join(", ")),
            })
        }
        _ => Ok(FrontmatterValue {
            name: field.name.to_owned(),
            value: "".to_owned(),
        }),
    }
}

pub fn generate_frontmatter_format_yaml(values: &Vec<FrontmatterValue>) -> String {
    let mut base = String::from("---\n");

    for value in values {
        base.push_str(&format!("{}: {}\n", value.name, value.value));
    }

    base.push_str("---\n");

    base
}
