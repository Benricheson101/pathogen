/// A helper macro for implementing TypeMapKey for one or more types at a time
#[macro_export]
macro_rules! impl_tmk {
    ($type:ty) => {
        impl serenity::prelude::TypeMapKey for $type {
            type Value = std::sync::Arc<serenity::prelude::Mutex<$type>>;
        }
    };

    ($($type:ty),* $(,)?) => { $( impl_tmk![$type] )* };
}

/// An easy-to-use way to set the type of database to use in the main file
#[macro_export]
macro_rules! use_database {
    ($type:ident) => {
        use $type as Database;
    };
}
