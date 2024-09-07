pub struct FrontmatterValue {
  pub name: String,
  pub value: String,
}

pub fn generate_frontmatter(values: Vec<FrontmatterValue>) -> String {
  let mut base = String::from("---\n");

  for value in values {
    base.push_str(&format!("{}: {}\n", value.name, value.value));
  }

  base.push_str("---\n");

  base
}
