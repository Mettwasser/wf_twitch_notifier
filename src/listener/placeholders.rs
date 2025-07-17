use crate::placeholder::Placeholder;

pub struct Node<'a>(pub &'a str);

impl<'a> Placeholder for Node<'a> {
    fn key(&self) -> &'static str {
        "{node}"
    }

    fn value(&self) -> &'a str {
        self.0
    }
}

pub struct Planet<'a>(pub &'a str);

impl<'a> Placeholder for Planet<'a> {
    fn key(&self) -> &'static str {
        "{planet}"
    }

    fn value(&self) -> &'a str {
        self.0
    }
}

pub struct Difficulty {
    pub is_hard: bool,
}

impl Placeholder for Difficulty {
    fn key(&self) -> &'static str {
        "{difficulty}"
    }

    fn value(&self) -> &str {
        if self.is_hard { "Steel Path" } else { "Normal" }
    }
}
