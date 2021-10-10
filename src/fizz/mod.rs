pub struct Matcher<T: ToString> {
    predicate: fn(&T) -> bool,
    substitute_with: String
}

impl<T: ToString> Matcher<T> {
    pub fn new(_predicate: fn(&T) -> bool, _substitute_with: &str) -> Matcher<T>
    {
        Matcher {
            predicate: _predicate,
            substitute_with: _substitute_with.to_string()
        }
    }

    pub fn apply(&self, elem: &T) -> Option<String> {
        if (self.predicate)(elem) {
            Some(self.substitute_with.clone())
        } else {
            None
        }
    }
}

pub struct Fizzy<T: ToString> {
    matchers: Vec<Matcher<T>>
}

impl<T: 'static + ToString> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy {
            matchers: vec![]
        }
    }

    pub fn add_matcher(mut self, _matcher: Matcher<T>) -> Self {
        self.matchers.push(_matcher);
        self
    }

    pub fn apply<I>(self, _iter: I) -> impl Iterator<Item = String>
    where I: Iterator<Item = T> {
        _iter.map(move |elem| {
            let mut was_transformed = false;
            let res = self.matchers.iter().fold(String::new(), |acc, matcher| {
                if let Some(transformed) = matcher.apply(&elem) {
                    was_transformed = true;
                    acc + transformed.as_str()
                } else {
                    acc
                }
            });
            if was_transformed {
                res
            } else {
                elem.to_string()
            }
        })
    }
}

pub fn fizz_buzz<T: 'static + ToString>() -> Fizzy<T> {

    fn to_u64_and_test<T: ToString, S>(elem: &T, pred: S) -> bool
    where S: Fn(u64) -> bool {
        let num = elem.to_string().parse().ok();
        num.map(|a| pred(a)).unwrap_or(false)
    }

    Fizzy::new()
        .add_matcher(Matcher::new(|a| to_u64_and_test(a, |num| num % 3 == 0), "fizz"))
        .add_matcher(Matcher::new(|a| to_u64_and_test(a, |num| num % 5 == 0), "buzz"))
}
