#[cfg(test)]
pub mod test {
  macro_rules! with_functions {
    ($($name:ident,)*) => {
        paste::paste! {
            $(
                fn [<with_ $name>](mut self, value: String) -> Self {
                    self.$name = Some(value);
                    self
                }
            )*
        }
    };
  }

  struct ParserRevision {
    revision_id: Option<String>,
    revision_id_parent: Option<String>,
    sha1: Option<String>,
    timestamp: Option<String>,
    contributor_id: Option<String>,
    contributor_ip: Option<String>,
    contributor_name: Option<String>,
  }


  impl ParserRevision {
    with_functions! {
        revision_id,
        revision_id_parent,
        sha1,
        timestamp,
        contributor_id,
        contributor_ip,
        contributor_name,
      }
  }


  #[test]
  fn inline_macro_example_test() {
    let builder = ParserRevision {
      revision_id: None,
      revision_id_parent: None,
      sha1: None,
      timestamp: None,
      contributor_id: None,
      contributor_ip: None,
      contributor_name: None,
    };

    builder.with_revision_id("some-id".to_string());
  }
}