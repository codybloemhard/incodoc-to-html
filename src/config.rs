
#[derive(Clone, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Config {
    pub nav: NavConfig,
    pub table_of_contents: TableOfContentsConfig,
    pub include: Include,
}

#[derive(Clone, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Include {
    BodyOnly,
    #[default]
    FullDocument,
    Augmented(String, String)
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct NavConfig {
    pub include: bool,
    pub close_top: bool,
    pub closed_depth: usize,
    pub position: Position,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TableOfContentsConfig {
    pub closed: bool,
    pub include: TableOfContentsInclusion,
    pub position: Position,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TableOfContentsInclusion {
    #[default]
    Include,
    Exclude,
    IfSuggested
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Position {
    #[default]
    Top,
    Bottom,
    BeforeFirstSubSection,
}

