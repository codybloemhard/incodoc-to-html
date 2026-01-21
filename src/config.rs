
#[derive(Clone, Default, Hash, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Config {
    pub nav: NavConfig,
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
    pub skip: bool,
    pub close_top: bool,
    pub closed_depth: usize,
}
