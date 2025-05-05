// src/app/api/signup.rs
use crate::lib::crypto::post_quantum::generate_hybrid_key_pair;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct SignupRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Clone)]
pub struct HybridKeys {
    classical_public_key: String,
    quantum_public_key: String,
}

#[derive(Serialize, Clone)]
pub struct SignupResponse {
    success: bool,
    message: String,
    pubkeys: Option<HybridKeys>,
    registry: String,
}

// Server function annotation - equivalent to your Next.js API route
#[server(Signup, "/api/signup")]
pub async fn signup(email: String, password: String) -> Result<SignupResponse, ServerFnError> {
    // Validation
    if email.is_empty() || password.is_empty() {
        return Err(ServerFnError::ServerError("Missing fields".to_string()));
    }

    // Mock database check (replace with real DB in production)
    let users =
        leptos::use_context::<std::sync::Mutex<std::collections::HashMap<String, String>>>()
            .ok_or_else(|| ServerFnError::ServerError("Database not available".to_string()))?;

    {
        let users_lock = users.lock().unwrap();
        if users_lock.contains_key(&email) {
            return Err(ServerFnError::ServerError(
                "User already exists".to_string(),
            ));
        }
    }

    // Generate post-quantum hybrid key pair
    let hybrid_keys = generate_hybrid_key_pair()?;

    // Store user (mock implementation)
    {
        let mut users_lock = users.lock().unwrap();
        users_lock.insert(email.clone(), password); // NOTE: In real apps, hash this!
    }

    Ok(SignupResponse {
        success: true,
        message: "User created".to_string(),
        pubkeys: Some(hybrid_keys),
        registry: "harbor.qompass.ai/library/test:latest does not support insert".to_string(),
    })
}
