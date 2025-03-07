#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StringValue {
    value: Option<Box<str>>,
}
impl StringValue {
    #[inline]
    pub(crate) fn get_value_str(&self) -> &str {
        self.value.as_deref().unwrap_or("")
    }

    #[inline]
    pub(crate) fn get_value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    #[inline]
    pub(crate) fn set_value<S: Into<String>>(&mut self, value: S) -> &mut StringValue {
        self.value = Some(value.into().into_boxed_str());
        self
    }

    /// works same as `set_value()` as the value in the struct is already a
    /// string
    #[inline]
    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut StringValue {
        self.set_value(value.into())
    }

    #[inline]
    pub(crate) fn remove_value(&mut self) -> &mut Self {
        self.value = None;
        self
    }

    #[inline]
    pub(crate) fn has_value(&self) -> bool {
        self.value.is_some()
    }

    #[inline]
    pub(crate) fn get_hash_string(&self) -> &str {
        if self.has_value() {
            return self.get_value_str();
        }
        "empty!!"
    }
}
