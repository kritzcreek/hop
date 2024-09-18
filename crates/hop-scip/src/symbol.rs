use crate::proto::scip;
use std::borrow::Cow;
mod context_error;
mod format;
mod parse;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Symbol<'a> {
    Local { local_id: Cow<'a, str> },
    Global(GlobalSymbol<'a>),
}

impl Symbol<'_> {
    pub fn parse(raw: &str) -> Result<Symbol, String> {
        parse::parse_symbol(raw)
    }

    pub fn is_local(&self) -> bool {
        matches!(self, Symbol::Local { .. })
    }

    pub fn to_proto(&self) -> scip::Symbol {
        match self {
            Symbol::Local { local_id } => scip::Symbol {
                scheme: "local".to_string(),
                package: None,
                descriptors: vec![scip::Descriptor {
                    name: local_id.to_string(),
                    disambiguator: "".to_string(),
                    suffix: scip::descriptor::Suffix::Local.into(),
                }],
            },
            Symbol::Global(global) => scip::Symbol {
                scheme: global.scheme.to_string(),
                package: Some(global.package.to_proto()),
                descriptors: global.descriptors.iter().map(|d| d.to_proto()).collect(),
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct GlobalSymbol<'a> {
    pub scheme: Scheme<'a>,
    pub package: Package<'a>,
    pub descriptors: Vec<Descriptor<'a>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct Scheme<'a>(Cow<'a, str>);

impl Scheme<'_> {
    pub fn new(s: &str) -> Scheme {
        Scheme(s.into())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Package<'a> {
    manager: Cow<'a, str>,
    package_name: Cow<'a, str>,
    version: Cow<'a, str>,
}

impl Default for Package<'_> {
    fn default() -> Self {
        Self::new(None, None, None)
    }
}

impl Package<'_> {
    pub fn new<'a>(
        manager: Option<&'a str>,
        package_name: Option<&'a str>,
        version: Option<&'a str>,
    ) -> Package<'a> {
        Package {
            manager: manager.unwrap_or(".").into(),
            package_name: package_name.unwrap_or(".").into(),
            version: version.unwrap_or(".").into(),
        }
    }
    pub fn manager(&self) -> Option<&str> {
        let manager = self.manager.as_ref();
        if manager == "." {
            None
        } else {
            Some(manager)
        }
    }
    pub fn package_name(&self) -> Option<&str> {
        let package_name = self.package_name.as_ref();
        if package_name == "." {
            None
        } else {
            Some(package_name)
        }
    }
    pub fn version(&self) -> Option<&str> {
        let version = self.version.as_ref();
        if version == "." {
            None
        } else {
            Some(version)
        }
    }

    pub fn to_proto(&self) -> scip::Package {
        scip::Package {
            manager: self.manager.to_string(),
            name: self.package_name.to_string(),
            version: self.version.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Descriptor<'a> {
    Namespace(Cow<'a, str>),
    Type(Cow<'a, str>),
    Term(Cow<'a, str>),
    Meta(Cow<'a, str>),
    Macro(Cow<'a, str>),
    Method {
        name: Cow<'a, str>,
        disambiguator: Option<&'a str>,
    },
    TypeParameter(Cow<'a, str>),
    Parameter(Cow<'a, str>),
}

impl Descriptor<'_> {
    pub fn to_proto(&self) -> scip::Descriptor {
        match self {
            Descriptor::Namespace(n) => scip::Descriptor {
                name: n.to_string(),
                disambiguator: "".to_string(),
                suffix: scip::descriptor::Suffix::Namespace.into(),
            },
            Descriptor::Type(t) => scip::Descriptor {
                name: t.to_string(),
                disambiguator: "".to_string(),
                suffix: scip::descriptor::Suffix::Type.into(),
            },
            Descriptor::Term(t) => scip::Descriptor {
                name: t.to_string(),
                disambiguator: "".to_string(),
                suffix: scip::descriptor::Suffix::Term.into(),
            },
            Descriptor::Meta(m) => scip::Descriptor {
                name: m.to_string(),
                disambiguator: "".to_string(),
                suffix: scip::descriptor::Suffix::Meta.into(),
            },
            Descriptor::Macro(m) => scip::Descriptor {
                name: m.to_string(),
                disambiguator: "".to_string(),
                suffix: scip::descriptor::Suffix::Macro.into(),
            },
            Descriptor::Method {
                name,
                disambiguator,
            } => scip::Descriptor {
                name: name.to_string(),
                disambiguator: disambiguator.unwrap_or_default().to_string(),
                suffix: scip::descriptor::Suffix::Method.into(),
            },
            Descriptor::TypeParameter(p) => scip::Descriptor {
                name: p.to_string(),
                disambiguator: "".to_string(),
                suffix: scip::descriptor::Suffix::TypeParameter.into(),
            },
            Descriptor::Parameter(p) => scip::Descriptor {
                name: p.to_string(),
                disambiguator: "".to_string(),
                suffix: scip::descriptor::Suffix::Parameter.into(),
            },
        }
    }
}
