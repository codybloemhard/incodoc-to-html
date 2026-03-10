use std::collections::HashSet;
use incodoc::actions::toc::TableOfContentsItemType;
use incodoc::actions::toc::TableOfContentsFilterType;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Config {
    pub nav: NavConfig,
    pub table_of_contents: TableOfContentsConfig,
    pub include: Include,
    pub header_links: Vec<HeaderLink>,
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

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct TableOfContentsConfig {
    pub closed: bool,
    pub include: TableOfContentsInclusion,
    pub position: Position,
    pub filter: Option<(HashSet<TableOfContentsItemType>, TableOfContentsFilterType)>,
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

#[derive(Clone, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum HeaderLink {
    Css {
        href: String,
    },
    General {
        rel: String,
        ltype: String,
        href: String,
    }
}
