use std::io::Stderr;

use promptuity::{
    prompts::{Confirm, Input, MultiSelect, MultiSelectOption, Select, SelectOption},
    Promptuity,
};

use crate::config::FrontmatterField;

pub struct FrontmatterValue {
    pub name: String,
    pub value: String,
    pub parent: Option<String>,
}

pub fn extract_frontmatter_value_with_prompt(
    p: &mut Promptuity<'_, Stderr>,
    field: &FrontmatterField,
    parent: Option<&str>,
) -> Result<FrontmatterValue, promptuity::Error> {
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
                parent: parent.map(|p| p.to_owned()),
            })
        }
        "boolean" => {
            let value = p.prompt(Confirm::new(field.question.to_owned()).with_default(true))?;
            Ok(FrontmatterValue {
                name: field.name.to_owned(),
                value: value.to_string(),
                parent: parent.map(|p| p.to_owned()),
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
                parent: parent.map(|p| p.to_owned()),
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
                parent: parent.map(|p| p.to_owned()),
            })
        }
        _ => Ok(FrontmatterValue {
            name: field.name.to_owned(),
            value: "".to_owned(),
            parent: None,
        }),
    }
}

pub fn generate_frontmatter_format_yaml(values: &Vec<FrontmatterValue>) -> String {
    let mut base = String::from("---\n");
    let mut current_parent = None;

    for value in values {
        // If tha parent has no value, then we need to reset the current parent
        if value.parent.is_none() {
            current_parent = None;
        }

        // If the parent is not the same as the current parent, then we need to add a new parent
        if value.parent.is_some() && current_parent != value.parent {
            current_parent = value.parent.clone();
            base.push_str(&format!("{}:\n", value.parent.as_ref().unwrap()));
        }

        if value.value != "" {
            if value.parent.is_some() {
                base.push_str(&format!("  {}: {}\n", value.name, value.value));
            } else {
                base.push_str(&format!("{}: {}\n", value.name, value.value));
            }
        }
    }

    base.push_str("---\n");

    base
}
