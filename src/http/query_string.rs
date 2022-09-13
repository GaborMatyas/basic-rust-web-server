use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(query: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in query.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(index_of_equal_sign) = query.find('=') {
                key = &sub_str[..index_of_equal_sign];
                val = &sub_str[index_of_equal_sign + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        *existing = Value::Multiple(vec![prev_val, val]);
                    }
                    Value::Multiple(_vec) => {}
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}
