use std::{
    borrow::Cow,
    fmt::Display,
};

#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value<T> {
    value: Option<T>,
}

impl<T: Clone> Value<T> {
    pub(crate) fn get_value(&self) -> Option<T> {
        self.value.clone()
    }

    pub(crate) fn get_value_unchecked(&self) -> T {
        self.value.clone().unwrap()
    }

    pub(crate) fn set_value(&mut self, value: T) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub(crate) fn remove_value(&mut self) -> &mut Self {
        self.value = None;
        self
    }

    pub(crate) fn has_value(&self) -> bool {
        self.value.is_some()
    }
}

impl<T: Display + Default + Clone> Value<T> {
    pub(crate) fn get_value_or_default(&self) -> T {
        self.value.clone().unwrap_or_default()
    }
}

macro_rules! create_and_export_ValueType {
    ($t:ty, $i:ident) => {
        pub type $i = Value<$t>;

        impl $i {
            pub(crate) fn get_value_string(&self) -> Cow<'_, str> {
                self.value
                    .as_ref()
                    .map_or_else(|| Cow::Borrowed(""), |v| Cow::Owned(v.to_string()))
            }

            pub(crate) fn set_value_string<S: AsRef<str>>(&mut self, value: S) -> &mut Self {
                match value.as_ref().parse::<T>() {
                    Ok(parsed) => self.set_value(parsed),
                    Err(_) => self.remove_value(), // Or handle the error differently, e.g., log it
                }
            }
        }
    };
}

create_and_export_ValueType!(bool, BooleanValue);
create_and_export_ValueType!(u8, ByteValue);
create_and_export_ValueType!(i8, SignedByteValue);
create_and_export_ValueType!(f64, DoubleValue);
create_and_export_ValueType!(i16, Int16Value);
create_and_export_ValueType!(i32, Int32Value);
create_and_export_ValueType!(i64, Int64Value);
create_and_export_ValueType!(u16, UInt16Value);
create_and_export_ValueType!(u32, UInt32Value);
create_and_export_ValueType!(u64, UInt64Value);

pub type StringValue<'a> = Value<Cow<'a, str>>;
impl<'a> StringValue<'a> {
    pub(crate) fn get_value_string(&self) -> Cow<'a, str> {
        self.value.clone().unwrap_or(Cow::Borrowed(""))
    }

    pub(crate) fn set_value_string<S: AsRef<str>>(&mut self, value: S) -> &mut Self {
        self.set_value(Cow::Owned(value.as_ref().to_string()))
    }
}
