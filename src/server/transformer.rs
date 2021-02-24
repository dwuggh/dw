///! transformer.rs -- text transformer for various purposes.

pub trait Transformer {
    /// perform the transform
    fn act<'a>(&self, text: &'a mut str) -> &'a str;
}

#[allow(dead_code)]
pub enum TF {
    Concat,
}

impl Transformer for TF {
    fn act<'a>(&self, _text: &'a mut str) -> &'a str {
        todo!()
    }
}
