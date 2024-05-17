const SUPPORTED_TYPES: [&str; 3] = [
    "::std::primitive::u8",
    "::std::primitive::char",
    "::std::ffi::c_char",
];

#[derive(Debug, Clone)]
pub struct RustType {
    segments: Vec<String>,
}

impl RustType {
    pub fn new(segments: Vec<String>) -> Self {
        Self { segments }
    }

    pub fn from_string(typ: &str) -> Self {
        if typ.is_empty() {
            return Self { segments: vec![] };
        }

        Self {
            segments: typ.split("::").map(String::from).collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    pub fn is_supported(&self) -> bool {
        SUPPORTED_TYPES
            .iter()
            .any(|&typ| (..=RustType::from(typ)).contains(self))
    }

    pub fn fuzzy_matches(&self, other: &Self) -> bool {
        self >= other && (self.is_supported() && other.is_supported())
    }

    pub fn exact_same_as(&self, other: &Self) -> bool {
        self.segments == other.segments
    }

    pub fn same_as(&self, other: &Self) -> bool {
        self.exact_same_as(other) || self.fuzzy_matches(other)
    }

    pub fn superset_of(&self, other: &Self) -> bool {
        self.segments.ends_with(&other.segments)
    }
}

impl ::std::ops::RangeBounds<RustType> for RustType {
    fn start_bound(&self) -> ::std::ops::Bound<&RustType> {
        if self.is_empty() {
            ::std::ops::Bound::Unbounded
        } else {
            ::std::ops::Bound::Included(self)
        }
    }

    fn end_bound(&self) -> ::std::ops::Bound<&RustType> {
        if self.is_empty() {
            ::std::ops::Bound::Unbounded
        } else {
            ::std::ops::Bound::Included(self)
        }
    }
}

impl PartialOrd for RustType {
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        if self.exact_same_as(other) {
            Some(::std::cmp::Ordering::Equal)
        } else if self.superset_of(other) {
            Some(::std::cmp::Ordering::Greater)
        } else if other.superset_of(self) {
            Some(::std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}

impl PartialEq for RustType {
    fn eq(&self, other: &Self) -> bool {
        self.same_as(other)
    }
}

impl From<String> for RustType {
    fn from(typ: String) -> Self {
        Self::from_string(&typ)
    }
}

impl From<&str> for RustType {
    fn from(typ: &str) -> Self {
        Self::from_string(typ)
    }
}

impl std::fmt::Display for RustType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.segments.join("::"))
    }
}

impl syn::parse::Parse for RustType {
    fn parse(stream: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = stream.parse::<syn::TypePath>()?;
        let segments = path
            .path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect();

        Ok(Self::new(segments))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let typ = RustType::from_string("::std::primitive::u8");
        assert_eq!(
            typ.segments(),
            vec!["", "std", "primitive", "u8"].as_slice()
        );
    }

    #[test]
    fn test_empty() {
        let typ = RustType::from_string("");
        let expected: Vec<String> = vec![];
        assert_eq!(typ.segments(), expected.as_slice());
    }

    #[test]
    fn test_superset() {
        let start = RustType::from_string("u8");
        let end = RustType::from_string("::std::primitive::u8");
        let value = RustType::from_string("std::primitive::u8");
        assert!(end.superset_of(&start));
        assert!(!start.superset_of(&end));
        assert!(value.superset_of(&start));
        assert!(end.superset_of(&value));

        assert!(end.superset_of(&end));
        assert!(start.superset_of(&start));
        assert!(value.superset_of(&value));

        assert!(start.superset_of(&RustType::from_string("")));
    }

    #[test]
    fn test_exact_same_as() {
        let start = RustType::from_string("u8");
        let end = RustType::from_string("::std::primitive::u8");
        let value = RustType::from_string("std::primitive::u8");
        assert!(!end.exact_same_as(&start));
        assert!(!start.exact_same_as(&end));
        assert!(!value.exact_same_as(&start));
        assert!(!end.exact_same_as(&value));
    }

    #[test]
    fn test_fuzzy_matches() {
        let start = RustType::from_string("u8");
        let end = RustType::from_string("::std::primitive::u8");
        let value = RustType::from_string("std::primitive::u8");
        assert!(end.fuzzy_matches(&start));
        assert!(!start.fuzzy_matches(&end));
        assert!(value.fuzzy_matches(&start));
        assert!(end.fuzzy_matches(&value));
    }
}
