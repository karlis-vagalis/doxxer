#[derive(Debug)]
pub struct TemplateVariables {
    pub pre: String,
    pub inc: usize,
    pub identifier: String,
    pub hash: String,
    pub distance: usize,
}
impl TemplateVariables {
    fn fields(&self) -> Vec<(&'static str, String)> {
        vec![
            ("{pre}", self.pre.clone()),
            ("{inc}", self.inc.to_string()),
            ("{identifier}", self.identifier.clone()),
            ("{hash}", self.hash.clone()),
            ("{distance}", self.distance.to_string()),
        ]
    }

    pub fn inject(&self, template: &str) -> String {
        let mut template = String::from(template);
        for (field, value) in self.fields() {
            //dbg!(&field, &value);
            template = template.replace(field, value.as_str());
            template = match template.strip_prefix(".") {
                Some(s) => s.to_string(),
                None => template,
            };
            template = match template.strip_suffix(".") {
                Some(s) => s.to_string(),
                None => template,
            };
        }

        //dbg!(&template);
        template
    }
}
