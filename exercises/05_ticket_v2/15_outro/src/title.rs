// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 bytes.
//   Implement the traits required to make the tests pass too.

#[derive(Debug, PartialEq, Clone)]
pub struct TicketTitle(String);

#[derive(Debug)]
pub enum TitleError{
    Err(String)
}

fn check(value:String) -> Result<TicketTitle, TitleError>{
    if value.is_empty(){

            return Err(TitleError::Err("The title cannot be empty".to_string()));
        }
    else if value.len() > 50{
        return Err(TitleError::Err("The title cannot be longer than 50 bytes".to_string()));
    }

        Ok(TicketTitle(value))
}
impl TryFrom<&str> for TicketTitle{
    type Error = TitleError;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        
        match check(value.to_string()){
            Ok(val) => {
                Ok(val)
            },
            Err(err) =>{
                Err(err)
            }
        }
    }
}

impl TryFrom<String> for TicketTitle{
    type Error = TitleError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl From<TitleError> for String{
    fn from(value: TitleError) -> Self {
        let TitleError::Err(err) = value;
        err
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(String::from(err), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(String::from(err), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
