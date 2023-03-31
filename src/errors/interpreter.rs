use itertools::Itertools;

use crate::{errors::ZError, types::value::Value};

impl ZError {
    #[must_use]
    pub fn i001(args: &[Value]) -> Self {
        Self::new(
            "I001",
            format!(
                "Builtin function failed (Arguments: {})",
                args.iter().map(ToString::to_string).join(", ")
            ),
        )
    }
}
