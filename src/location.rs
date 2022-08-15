use derive_more::Display;

#[derive(Clone, PartialEq, Debug, Display)]
#[display(fmt = "{line}:{column}")]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, PartialEq, Debug, Display)]
// #[display(fmt = "{filepath}:{start}-{end}")]
#[display(fmt = "{filepath}:{start}")]
pub struct SourceSpan {
    pub filepath: String,
    pub start: SourceLocation,
    pub end: SourceLocation,
}
