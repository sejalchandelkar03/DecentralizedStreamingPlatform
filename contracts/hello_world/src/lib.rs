#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

// Define structures for tracking content and payments
#[contracttype]
#[derive(Clone)]
pub struct Content {
    pub content_id: u64,      // Unique identifier for each content (video/audio)
    pub title: String,        // Title of the content
    pub description: String,  // Description of the content
    pub creator: String,      // Address of the content creator
    pub price: u64,           // Price to access the content
    pub total_views: u64,     // Total number of views (accesses)
    pub royalties_collected: u64, // Total royalties collected for this content
}

// For referencing unique content
const CONTENT_COUNTER: Symbol = symbol_short!("C_COUNT");

// Mapping content IDs to their respective Content struct
#[contracttype]
pub enum ContentMap {
    Content(u64)
}

#[contract]
pub struct DecentralizedStreamingPlatform;

#[contractimpl]
impl DecentralizedStreamingPlatform {
    
    // Function to register new content on the platform
    pub fn register_content(env: Env, title: String, description: String, creator: String, price: u64) -> u64 {
        // Fetch the current content count and increment it
        let mut content_count: u64 = env.storage().instance().get(&CONTENT_COUNTER).unwrap_or(0);
        content_count += 1;

        // Create a new content struct with the provided details
        let new_content = Content {
            content_id: content_count,
            title: title.clone(),
            description: description.clone(),
            creator: creator.clone(),
            price: price,
            total_views: 0,
            royalties_collected: 0,
        };

        // Store the newly created content in storage
        env.storage().instance().set(&ContentMap::Content(content_count), &new_content);
        env.storage().instance().set(&CONTENT_COUNTER, &content_count);

        log!(&env, "Content Registered: ID = {}, Title = {}", content_count, title);

        // Return the content ID as confirmation
        content_count
    }

    // Function to view the details of registered content by its content_id
    pub fn view_content(env: Env, content_id: u64) -> Content {
        // Fetch content from storage, or return a default structure if not found
        env.storage().instance().get(&ContentMap::Content(content_id)).unwrap_or(Content {
            content_id: 0,
            title: String::from_str(&env, "Not Found"),
            description: String::from_str(&env, "Not Found"),
            creator: String::from_str(&env, "Not Found"),
            price: 0,
            total_views: 0,
            royalties_collected: 0,
        })
    }

    // Function to handle content access and royalty payments
    pub fn access_content(env: Env, content_id: u64, user: String) {
        // Fetch content by ID
        let mut content = Self::view_content(env.clone(), content_id);

        // Check if the content exists and user is accessing it
        if content.content_id != 0 {
            // Increment the view count
            content.total_views += 1;

            // Assuming user has paid the content price, royalties are updated
            content.royalties_collected += content.price;

            // Store updated content back to storage
            env.storage().instance().set(&ContentMap::Content(content_id), &content);

            log!(&env, "User: {} accessed Content ID: {}. Total Views: {}", user, content_id, content.total_views);
        } else {
            log!(&env, "Content with ID: {} not found", content_id);
            panic!("Content not found");
        }
    }

    // Function to view the royalties collected for a content
    pub fn view_royalties(env: Env, content_id: u64) -> u64 {
        let content = Self::view_content(env.clone(), content_id);
        content.royalties_collected
    }
}
