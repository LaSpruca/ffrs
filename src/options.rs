use std::marker::PhantomData;

#[derive(Copy, Clone, Debug)]
pub enum SortKind {
    InsertOrder,
    BestMatch,
}

pub struct A;
pub struct DefaultVec;

pub struct FuzzyOptions<'a, T, U = A> {
    pub(crate) ignore_case: bool,
    pub(crate) ignore_symbols: bool,
    pub(crate) normalize_whitespace: bool,
    pub(crate) use_damerau: bool,
    pub(crate) use_sellers: bool,
    pub(crate) use_separated_unicode: bool,
    pub(crate) sort_by: SortKind,
    pub(crate) key_selector: Box<dyn Fn(&'a T) -> Vec<String>>,
    pub(crate) threshold: f64,
    _marker: PhantomData<U>,
}

impl<'a, StringLike> Default for FuzzyOptions<'a, StringLike>
where
    StringLike: ToString,
{
    fn default() -> Self {
        Self {
            ignore_case: true,
            ignore_symbols: true,
            normalize_whitespace: true,
            use_damerau: true,
            use_sellers: true,
            use_separated_unicode: false,
            sort_by: SortKind::BestMatch,
            key_selector: Box::new(|x| vec![x.to_string()]),
            threshold: 0.6,
            _marker: PhantomData,
        }
    }
}

impl<'a, StringLike> Default for FuzzyOptions<'a, Vec<StringLike>, DefaultVec>
where
    StringLike: ToString,
{
    fn default() -> Self {
        Self {
            ignore_case: true,
            ignore_symbols: true,
            normalize_whitespace: true,
            use_damerau: true,
            use_sellers: true,
            use_separated_unicode: false,
            sort_by: SortKind::BestMatch,
            key_selector: Box::new(|x| x.iter().map(|x| x.to_string()).collect()),
            threshold: 0.6,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> FuzzyOptions<'a, T> {
    pub fn default_with_key_selector<Func>(key_selector: Func) -> Self
    where
        Func: Fn(&'a T) -> Vec<String> + 'static,
    {
        Self {
            key_selector: Box::new(key_selector),
            ignore_case: true,
            ignore_symbols: true,
            normalize_whitespace: true,
            use_damerau: true,
            use_sellers: true,
            use_separated_unicode: false,
            sort_by: SortKind::BestMatch,
            threshold: 0.6,
            _marker: PhantomData,
        }
    }

    pub fn ignore_case(mut self, ignore_case: bool) -> Self {
        self.ignore_case = ignore_case;
        self
    }

    pub fn ignore_symbols(mut self, ignore_symbols: bool) -> Self {
        self.ignore_symbols = ignore_symbols;
        self
    }

    pub fn normalize_whitespace(mut self, normalize_whitespace: bool) -> Self {
        self.normalize_whitespace = normalize_whitespace;
        self
    }

    pub fn use_damerau(mut self, use_damerau: bool) -> Self {
        self.use_damerau = use_damerau;
        self
    }

    pub fn use_sellers(mut self, use_sellers: bool) -> Self {
        self.use_sellers = use_sellers;
        self
    }

    pub fn use_separated_unicode(mut self, use_separated_unicode: bool) -> Self {
        self.use_separated_unicode = use_separated_unicode;
        self
    }

    pub fn sort_by(mut self, sort_by: SortKind) -> Self {
        self.sort_by = sort_by;
        self
    }

    pub fn key_selector<F>(mut self, key_selector: F) -> Self
    where
        F: Fn(&'a T) -> Vec<String> + 'static,
    {
        self.key_selector = Box::new(key_selector);
        self
    }

    pub fn threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold;
        self
    }

    pub fn as_str_options(&self) -> FuzzyOptions<'a, String> {
        FuzzyOptions {
            key_selector: Box::new(|x| vec![x.clone()]),
            ignore_case: self.ignore_case,
            ignore_symbols: self.ignore_symbols,
            normalize_whitespace: self.normalize_whitespace,
            use_damerau: self.use_damerau,
            use_sellers: self.use_sellers,
            use_separated_unicode: self.use_separated_unicode,
            sort_by: self.sort_by,
            threshold: self.threshold,
            _marker: PhantomData,
        }
    }
}
