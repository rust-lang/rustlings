/// Gibt jedes Element von a und dann jedes Element von b zurück (ohne chain)
pub fn append<I, J>(a: I, b: J) -> Append<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    Append { a, b }
}

pub struct Append<I, J> {
    a: I,
    b: J,
}

impl<I, J> Iterator for Append<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.a.next().or_else(|| self.b.next())
    }
}

/// Kombiniert alle Elemente aus allen verschachtelten Iteratoren zu einem flachen Iterator (ohne flatten)
pub fn concat<I>(nested_iter: I) -> Concat<I>
where
    I: Iterator,
    I::Item: Iterator,
{
    Concat {
        outer: nested_iter,
        inner: None,
    }
}

pub struct Concat<I>
where
    I: Iterator,
    I::Item: Iterator,
{
    outer: I,
    inner: Option<I::Item>,
}

impl<I> Iterator for Concat<I>
where
    I: Iterator,
    I::Item: Iterator,
{
    type Item = <I::Item as Iterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(inner) = &mut self.inner {
                if let Some(item) = inner.next() {
                    return Some(item);
                }
            }
            match self.outer.next() {
                Some(next_inner) => self.inner = Some(next_inner),
                None => return None,
            }
        }
    }
}

/// Gibt einen Iterator über alle Elemente zurück, für die das Prädikat true ergibt (ohne filter)
pub fn filter<I, F>(iter: I, predicate: F) -> Filter<I, F>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    Filter { iter, predicate }
}

pub struct Filter<I, F> {
    iter: I,
    predicate: F,
}

impl<I, F> Iterator for Filter<I, F>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if (self.predicate)(&item) {
                return Some(item);
            }
        }
        None
    }
}

pub fn length<I: Iterator>(mut iter: I) -> usize {
    let mut n = 0;
    while let Some(_) = iter.next() {
        n += 1;
    }
    n
}

/// Gibt einen Iterator mit den Ergebnissen der Funktion auf alle Elemente zurück (ohne map)
pub fn map<I, F, U>(iter: I, function: F) -> Map<I, F>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    Map { iter, function }
}

pub struct Map<I, F> {
    iter: I,
    function: F,
}

impl<I, F, U> Iterator for Map<I, F>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    type Item = U;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&self.function)
    }
}

pub fn foldl<I, F, U>(mut iter: I, mut acc: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    while let Some(item) = iter.next() {
        acc = function(acc, item);
    }
    acc
}

pub fn foldr<I, F, U>(mut iter: I, acc: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    // Sammle alle Elemente in einen Vektor, dann von rechts falten
    let mut items = Vec::new();
    while let Some(item) = iter.next() {
        items.push(item);
    }
    let mut acc = acc;
    while let Some(item) = items.pop() {
        acc = function(acc, item);
    }
    acc
}

/// Gibt einen Iterator mit allen ursprünglichen Elementen in umgekehrter Reihenfolge zurück (ohne rev)
pub fn reverse<I: DoubleEndedIterator>(iter: I) -> Reverse<I>
where
    I: DoubleEndedIterator,
{
    Reverse { iter, finished: false }
}

pub struct Reverse<I> {
    iter: I,
    finished: bool,
}

impl<I> Iterator for Reverse<I>
where
    I: DoubleEndedIterator,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            let next = self.iter.next_back();
            if next.is_none() {
                self.finished = true;
            }
            next
        }
    }
}
