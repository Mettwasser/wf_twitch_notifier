#[macro_export]
macro_rules! commands {
    ($($cond:expr => $command:expr),* $(,)?) => {
        {
            let mut temp = Vec::new();

            $(
                if $cond {
                    temp.push(Box::new($command) as Box<dyn $crate::commands::Command>);
                }
            )*

            temp
        }
    };
}
