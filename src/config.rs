
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
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TableOfContentsConfig {
    pub include: TableOfContentsInclusion,
    pub closed: bool,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TableOfContentsInclusion {
    #[default]
    Include,
    Exclude,
    IfSuggested
}

