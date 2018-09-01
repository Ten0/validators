#[macro_use]
extern crate validators;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validated_customized_string() {
        validated_customized_string!(S1, (),
            from_string input {
                Ok(input.to_string())
            },
            from_str input {
                Ok(input.to_string())
            }
        );

        validated_customized_string!(pub S2, (),
            from_string input {
                Ok(input.to_string())
            },
            from_str input {
                Ok(input.to_string())
            }
        );
    }
}