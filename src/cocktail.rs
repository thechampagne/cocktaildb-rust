/*
 * Copyright 2022 XXIV
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use std::io::Read;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use urlencoding::encode;
use crate::error::CocktailError;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Cocktails {
    drinks: Vec<Cocktail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cocktail {
    pub id_drink: Option<String>,
    pub str_drink: Option<String>,
    pub str_drink_alternate: Option<String>,
    pub str_tags: Option<String>,
    pub str_video: Option<String>,
    pub str_category: Option<String>,
    #[serde(rename = "strIBA")]
    pub str_iba: Option<String>,
    pub str_alcoholic: Option<String>,
    pub str_glass: Option<String>,
    pub str_instructions: Option<String>,
    #[serde(rename = "strInstructionsES")]
    pub str_instructions_es: Option<String>,
    #[serde(rename = "strInstructionsDE")]
    pub str_instructions_de: Option<String>,
    #[serde(rename = "strInstructionsFR")]
    pub str_instructions_fr: Option<String>,
    #[serde(rename = "strInstructionsIT")]
    pub str_instructions_it: Option<String>,
    #[serde(rename = "strInstructionsZH-HANS")]
    pub str_instructions_zh_hans: Option<String>,
    #[serde(rename = "strInstructionsZH-HANT")]
    pub str_instructions_zh_hant: Option<String>,
    pub str_drink_thumb: Option<String>,
    pub str_ingredient1: Option<String>,
    pub str_ingredient2: Option<String>,
    pub str_ingredient3: Option<String>,
    pub str_ingredient4: Option<String>,
    pub str_ingredient5: Option<String>,
    pub str_ingredient6: Option<String>,
    pub str_ingredient7: Option<String>,
    pub str_ingredient8: Option<String>,
    pub str_ingredient9: Option<String>,
    pub str_ingredient10: Option<String>,
    pub str_ingredient11: Option<String>,
    pub str_ingredient12: Option<String>,
    pub str_ingredient13: Option<String>,
    pub str_ingredient14: Option<String>,
    pub str_ingredient15: Option<String>,
    pub str_measure1: Option<String>,
    pub str_measure2: Option<String>,
    pub str_measure3: Option<String>,
    pub str_measure4: Option<String>,
    pub str_measure5: Option<String>,
    pub str_measure6: Option<String>,
    pub str_measure7: Option<String>,
    pub str_measure8: Option<String>,
    pub str_measure9: Option<String>,
    pub str_measure10: Option<String>,
    pub str_measure11: Option<String>,
    pub str_measure12: Option<String>,
    pub str_measure13: Option<String>,
    pub str_measure14: Option<String>,
    pub str_measure15: Option<String>,
    pub str_image_source: Option<String>,
    pub str_image_attribution: Option<String>,
    pub str_creative_commons_confirmed: Option<String>,
    pub date_modified: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Ingredients {
    ingredients: Vec<Ingredient>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
    pub id_ingredient: Option<String>,
    pub str_ingredient: Option<String>,
    pub str_description: Option<String>,
    pub str_type: Option<String>,
    pub str_alcohol: Option<String>,
    #[serde(rename = "strABV")]
    pub str_abv: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FilterDrinks {
    drinks: Vec<Filter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub str_drink: Option<String>,
    pub str_drink_thumb: Option<String>,
    pub id_drink: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CategoriesFilter {
    drinks: Vec<Category>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Category {
    str_category: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GlassesFilter {
    drinks: Vec<Glass>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Glass {
    str_glass: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IngredientsFilter {
    drinks: Vec<IngredientFilter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IngredientFilter {
    str_ingredient1: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AlcoholicFilter {
    drinks: Vec<Alcoholic>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Alcoholic {
    str_alcoholic: String,
}

fn http(endpoint: &str) -> Option<String> {
    match reqwest::blocking::Client::new().get(format!("https://thecocktaildb.com/api/json/v1/1/{}", endpoint))
        .send() {
        Ok(mut response) => {
            let mut body = String::new();
            match response.read_to_string(&mut body) {
                Ok(_) => Some(body),
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

/// Search cocktail by name
pub fn search(s: &str) -> Result<Vec<Cocktail>, CocktailError> {
    match http(format!("search.php?s={}", encode(s)).as_str()) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let cocktail: Cocktails = json;
                    for i in cocktail.drinks {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(CocktailError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(CocktailError::Error(String::from("null")))
            }
        },
        None => Err(CocktailError::Error(String::from("null")))
    }
}

/// Search cocktails by first letter
pub fn search_by_letter(c: char) -> Result<Vec<Cocktail>, CocktailError> {
    match http(format!("search.php?f={}", c).as_str()) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let cocktail: Cocktails = json;
                    for i in cocktail.drinks {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(CocktailError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(CocktailError::Error(String::from("null")))
            }
        },
        None => Err(CocktailError::Error(String::from("null")))
    }
}

/// Search ingredient by name
pub fn search_ingredient(s: &str) -> Result<Ingredient, CocktailError> {
    match http(format!("search.php?i={}", encode(s)).as_str()) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let cocktail: Ingredients = json;
                    if cocktail.ingredients.is_empty() {
                        Err(CocktailError::Error(String::from("null")))
                    } else {
                        Ok(cocktail.ingredients[0].clone())
                    }
                },
                Err(_) => Err(CocktailError::Error(String::from("null")))
            }
        },
        None => Err(CocktailError::Error(String::from("null")))
    }
}

/// Search cocktail details by id
pub fn search_by_id(i: i64) -> Result<Cocktail, CocktailError> {
    match http(format!("lookup.php?i={}", i).as_str()) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let cocktail: Cocktails = json;
                    if cocktail.drinks.is_empty() {
                        Err(CocktailError::Error(String::from("null")))
                    } else {
                        Ok(cocktail.drinks[0].clone())
                    }
                },
                Err(_) => Err(CocktailError::Error(String::from("null")))
            }
        },
        None => Err(CocktailError::Error(String::from("null")))
    }
}

/// Search ingredient by ID
pub fn search_ingredient_by_id(i: i64) -> Result<Ingredient, CocktailError> {
    match http(format!("lookup.php?iid={}", i).as_str()) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let cocktail: Ingredients = json;
                    if cocktail.ingredients.is_empty() {
                        Err(CocktailError::Error(String::from("null")))
                    } else {
                        Ok(cocktail.ingredients[0].clone())
                    }
                },
                Err(_) => Err(CocktailError::Error(String::from("null")))
            }
        },
        None => Err(CocktailError::Error(String::from("null")))
    }
}

/// Search a random cocktail
pub fn random() -> Result<Cocktail, CocktailError> {
    match http("random.php") {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let cocktail: Cocktails = json;
                    if cocktail.drinks.is_empty() {
                        Err(CocktailError::Error(String::from("null")))
                    } else {
                        Ok(cocktail.drinks[0].clone())
                    }
                },
                Err(_) => Err(CocktailError::Error(String::from("null")))
            }
        },
        None => Err(CocktailError::Error(String::from("null")))
    }
}

/// Filter by ingredient
pub fn filter_by_ingredient(s: &str) -> Result<Vec<Filter>, CocktailError> {
    match http(format!("filter.php?i={}", encode(s)).as_str()) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let cocktail: FilterDrinks = json;
                    for i in cocktail.drinks {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(CocktailError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(CocktailError::Error(String::from("null")))
            }
        },
        None => Err(CocktailError::Error(String::from("null")))
    }
}

/// Filter by alcoholic
pub fn filter_by_alcoholic(s: &str) -> Result<Vec<Filter>, CocktailError> {
    match http(format!("filter.php?a={}", encode(s)).as_str()) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let cocktail: FilterDrinks = json;
                    for i in cocktail.drinks {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(CocktailError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(CocktailError::Error(String::from("null")))
            }
        },
        None => Err(CocktailError::Error(String::from("null")))
    }
}

/// Filter by Category
pub fn filter_by_category(s: &str) -> Result<Vec<Filter>, CocktailError> {
    match http(format!("filter.php?c={}", encode(s)).as_str()) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let cocktail: FilterDrinks = json;
                    for i in cocktail.drinks {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(CocktailError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(CocktailError::Error(String::from("null")))
            }
        },
        None => Err(CocktailError::Error(String::from("null")))
    }
}

/// Filter by Glass
pub fn filter_by_glass(s: &str) -> Result<Vec<Filter>, CocktailError> {
    match http(format!("filter.php?g={}", encode(s)).as_str()) {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let cocktail: FilterDrinks = json;
                    for i in cocktail.drinks {
                        vector.push(i)
                    }
                    if vector.is_empty() {
                        Err(CocktailError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(CocktailError::Error(String::from("null")))
            }
        },
        None => Err(CocktailError::Error(String::from("null")))
    }
}

/// List the categories filter
pub fn categories_filter() -> Result<Vec<String>, CocktailError> {
    match http("list.php?c=list") {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let cocktail: CategoriesFilter = json;
                    for i in cocktail.drinks {
                        vector.push(i.str_category)
                    }
                    if vector.is_empty() {
                        Err(CocktailError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(CocktailError::Error(String::from("null")))
            }
        },
        None => Err(CocktailError::Error(String::from("null")))
    }
}

/// List the glasses filter
pub fn glasses_filter() -> Result<Vec<String>, CocktailError> {
    match http("list.php?g=list") {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let cocktail: GlassesFilter = json;
                    for i in cocktail.drinks {
                        vector.push(i.str_glass)
                    }
                    if vector.is_empty() {
                        Err(CocktailError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(CocktailError::Error(String::from("null")))
            }
        },
        None => Err(CocktailError::Error(String::from("null")))
    }
}

/// List the ingredients filter
pub fn ingredients_filter() -> Result<Vec<String>, CocktailError> {
    match http("list.php?i=list") {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let cocktail: IngredientsFilter = json;
                    for i in cocktail.drinks {
                        vector.push(i.str_ingredient1)
                    }
                    if vector.is_empty() {
                        Err(CocktailError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(CocktailError::Error(String::from("null")))
            }
        },
        None => Err(CocktailError::Error(String::from("null")))
    }
}

/// List the alcoholic filter
pub fn alcoholic_filter() -> Result<Vec<String>, CocktailError> {
    match http("list.php?a=list") {
        Some(response) => {
            match serde_json::from_str(&response) {
                Ok(json) => {
                    let mut vector = vec![];
                    let cocktail: AlcoholicFilter = json;
                    for i in cocktail.drinks {
                        vector.push(i.str_alcoholic)
                    }
                    if vector.is_empty() {
                        Err(CocktailError::Error(String::from("null")))
                    } else {
                        Ok(vector)
                    }
                },
                Err(_) => Err(CocktailError::Error(String::from("null")))
            }
        },
        None => Err(CocktailError::Error(String::from("null")))
    }
}