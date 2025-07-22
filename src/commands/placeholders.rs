use crate::placeholder::Placeholder;

#[derive(Clone, Copy)]
pub struct Author<'a>(pub &'a str);

impl Placeholder for Author<'_> {
    fn key(&self) -> &'static str {
        "{author}"
    }

    fn value(&self) -> &str {
        self.0
    }
}

pub struct Average(pub String);

impl Placeholder for Average {
    fn key(&self) -> &'static str {
        "{average}"
    }

    fn value(&self) -> &str {
        self.0.as_ref()
    }
}

pub struct MovingAverage(pub String);

impl Placeholder for MovingAverage {
    fn key(&self) -> &'static str {
        "{moving_average}"
    }

    fn value(&self) -> &str {
        self.0.as_ref()
    }
}

pub struct AmountSold(pub String);

impl Placeholder for AmountSold {
    fn key(&self) -> &'static str {
        "{amount_sold}"
    }

    fn value(&self) -> &str {
        self.0.as_ref()
    }
}

pub struct ItemName<'a>(pub &'a str);

impl Placeholder for ItemName<'_> {
    fn key(&self) -> &'static str {
        "{item_name}"
    }

    fn value(&self) -> &str {
        self.0
    }
}
