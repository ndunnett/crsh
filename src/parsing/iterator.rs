pub trait ParsingIterator {
    type PredicateItem;
    type Item;

    fn peek_item(&mut self) -> Option<&Self::Item>;
    fn next_item(&mut self) -> Option<&Self::Item>;
    fn check_if(&mut self, f: impl FnOnce(Self::PredicateItem) -> bool) -> bool;

    fn next_if(&mut self, f: impl FnOnce(Self::PredicateItem) -> bool) -> Option<&Self::Item> {
        if self.check_if(f) {
            self.next_item()
        } else {
            None
        }
    }

    fn advance_if(&mut self, f: impl FnOnce(Self::PredicateItem) -> bool) -> bool {
        self.next_if(f).is_some()
    }

    fn take_until(&mut self, f: impl Fn(Self::PredicateItem) -> bool) {
        while self.advance_if(|t| !f(t)) {}
    }
}
