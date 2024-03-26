#[derive(Clone)]
pub struct StringValue {
    pub value: String,
}
pub trait AppendStr {
    fn append_str(&mut self, new_str: String) -> &mut Self;
    fn append_number(&mut self, new_number: f64) -> &mut Self;
    fn remove_punctuation_marks(&mut self) -> &mut Self;
}
impl AppendStr for StringValue {
    fn append_str(&mut self, new_str: String) -> &mut Self {
        self.value += &new_str;
        self
    }
    fn append_number(&mut self, new_number: f64) -> &mut Self {
        self.append_str(new_number.to_string())
    }
    fn remove_punctuation_marks(&mut self) -> &mut Self {
        self.value = self.value.replace(['.', ',', '?', '!'], "");
        self
    }
}
