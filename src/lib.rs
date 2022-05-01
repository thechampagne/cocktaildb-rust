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
//! TheCocktailDB API client
//!
//! An open, crowd-sourced database of drinks
//! and cocktails from around the world.
mod cocktail;
mod error;
pub use error::CocktailError;
pub use cocktail::Filter;
pub use cocktail::Ingredient;
pub use cocktail::Cocktail;
pub use cocktail::search;
pub use cocktail::search_by_letter;
pub use cocktail::search_ingredient;
pub use cocktail::search_by_id;
pub use cocktail::search_ingredient_by_id;
pub use cocktail::random;
pub use cocktail::filter_by_ingredient;
pub use cocktail::filter_by_alcoholic;
pub use cocktail::filter_by_category;
pub use cocktail::filter_by_glass;
pub use cocktail::categories_filter;
pub use cocktail::glasses_filter;
pub use cocktail::ingredients_filter;
pub use cocktail::alcoholic_filter;