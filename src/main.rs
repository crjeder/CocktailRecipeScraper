#[macro_use] extern crate quick_error;

mod converters;
mod cocktail;

fn main() -> std::io::Result<()>
{
  Ok(())  
}


// Always use a limit to prevent DoS attacks.
const LIMIT: u64 = 1024;
/*
impl data::FromDataSimple for Cocktail
{
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String>
    {
        // Read the data into a String.
        let mut string = String::new();
        if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string)
        {
            return Outcome::Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        let cocktail: Cocktail = match serde_json::from_str(string.to_string().as_ref())
        {
            Err(e) => return Outcome::Failure((Status::UnprocessableEntity, format!("':' {}", e))),
            Ok(c) => c,
        };
        Outcome::Success(cocktail)
    }
}
*/
// curl --data '{"name": "Screwdriver", "glass": "Highball", "category": "All Day", "shaken_not_stirred": true, "ingredients": [{"amount": 60,"name": "Orage Juice"},{"amount": 30,"name": "Vodka"}],"garnish": "Slice of orange","preparation": "Pour everything into a glass over ice."}' http://localhost:8000/REST/post/cocktail
