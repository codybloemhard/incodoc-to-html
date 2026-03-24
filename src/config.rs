use std::collections::HashSet;
use incodoc::actions::toc::TableOfContentsItemType;
use incodoc::actions::toc::TableOfContentsFilterType;

/// Configuration for the HTML generation
#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct Config {
    pub nav: NavConfig,
    pub table_of_contents: TableOfContentsConfig,
    pub include: Include,
    pub header_links: Vec<HeaderLink>,
    pub links: LinksConfig,
    pub paragraphs: ParagraphsConfig,
}

/// What to include in the document
#[derive(Clone, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Include {
    /// Only include the body
    BodyOnly,
    /// Include everything
    #[default]
    FullDocument,
    /// Include everything along with a prefix and suffix inside the body
    Augmented(String, String)
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct NavConfig {
    /// Whether to include navigation
    pub include: bool,
    /// Whether the top level navigation is collapsed or not
    pub close_top: bool,
    /// At what depth the sub navigation is collapsed
    pub closed_depth: usize,
    /// Position of the navigation within the document
    pub position: Position,
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct TableOfContentsConfig {
    /// Whether the top level table of contents is collapsed or not
    pub closed: bool,
    /// Whether to include the table of contents
    pub include: TableOfContentsInclusion,
    /// Position of the table of contents within the document
    pub position: Position,
    /// The filter decides what items make it into the table of contents
    pub filter: Option<(HashSet<TableOfContentsItemType>, TableOfContentsFilterType)>,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum TableOfContentsInclusion {
    #[default]
    Include,
    Exclude,
    /// Include if property 'table-of-contents' is set to 'include'
    IfSuggested
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Position {
    /// At the top of the document
    #[default]
    Top,
    /// At the bottom of the document
    Bottom,
    /// Before the first subsection, usually after the first paragraphs
    BeforeFirstSubSection,
}

#[derive(Clone, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum HeaderLink {
    /// A header link liking to a css style sheet
    Css {
        href: String,
    },
    /// A header link for general use
    General {
        rel: String,
        ltype: String,
        href: String,
    }
}

#[derive(Clone, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct LinksConfig {
    /// Whether links to local files open in target '_blank'
    pub local_links_open_in_blank: bool,
    /// Whether links to footnote definitions open in target '_blank'
    pub footnote_ref_links_open_in_blank: bool,
    /// Prefix for links to footnote definitions
    pub footnote_ref_prefix: String,
}

#[derive(Clone, Copy, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ParagraphsConfig {
    /// 'big' paragraphs are like incodoc's paragraphs, represented by a <div>
    #[default]
    JustBigParagraphs,
    /// 'small' paragraphs are like HTML's paragraphs, represented by <p>
    JustSmallParagraphs,
    BigAndSmallParagraphs,
}

impl ParagraphsConfig {
    /// Returns (use big pars, use small pars)
    pub fn split(&self) -> (bool, bool) {
        match self {
            Self::JustBigParagraphs => (true, false),
            Self::JustSmallParagraphs => (false, true),
            _ => (true, true),
        }
    }
}
