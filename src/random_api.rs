use reqwest::Error;
use json::{parse, JsonValue};

pub struct RandomApiResponse {
    pub gender: String,
    pub name: String,
    pub location: String,
    pub email: String,
    pub login: String,
    pub dob: String,
    pub registered: String,
    pub phone: String,
    pub cell: String,
    pub id: String,
    pub picture: String,
    pub nat: String,
}

pub async fn random_api() -> Result<RandomApiResponse, Box<dyn std::error::Error>> {
    let url = "https://randomuser.me/api/?password=special,lower,upper,number,1-20";

    let response = reqwest::get(url).await?.text().await?;
    let json: Result<JsonValue, json::Error> = parse(&response);

    match json {
        Ok(parsed_json) => {
            // Extract fields from the parsed JSON
            let gender = parsed_json["results"][0]["gender"].to_string();
            let name = parsed_json["results"][0]["name"].to_string();
            let location = parsed_json["results"][0]["location"].to_string();
            let email = parsed_json["results"][0]["email"].to_string();
            let login = parsed_json["results"][0]["login"].to_string();
            let dob = parsed_json["results"][0]["dob"].to_string();
            let registered = parsed_json["results"][0]["registered"].to_string();
            let phone = parsed_json["results"][0]["phone"].to_string();
            let cell = parsed_json["results"][0]["cell"].to_string();
            let id = parsed_json["results"][0]["id"].to_string();
            let picture = parsed_json["results"][0]["picture"].to_string();
            let nat = parsed_json["results"][0]["nat"].to_string();

            // Create and return the RandomApiResponse struct
            let response = RandomApiResponse {
                gender,
                name,
                location,
                email,
                login,
                dob,
                registered,
                phone,
                cell,
                id,
                picture,
                nat,
            };

            Ok(response)
        }
        Err(err) => Err(Box::new(err)),
    }
}
    