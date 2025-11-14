// utils/helpers.js
/**
 * Shared utility functions nya~
 */

// Format date to locale string
export function formatDate(dateString) {
    return new Date(dateString).toLocaleString();
}

// Format duration in seconds to "Xm Ys"
export function formatDuration(seconds) {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}m ${secs}s`;
}

// Truncate UUID to the first 8 characters
export function truncateId(id, length = 8) {
    return id.slice(0, length);
}

// Base API URL (can change for production!)
export const API_BASE_URL = 'http://localhost:8000/api';

// API helper functions
export async function fetchAPI(endpoint, options = {}) {
    try {
        const response = await fetch(`${API_BASE_URL}${endpoint}`, options);
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}`);
        }
        return await response.json();
    } catch (error) {
        console.error('API Error:', error);
        throw error;
    }
}

export async function postAPI(endpoint, data) {
    return fetchAPI(endpoint, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data)
    });
}

export async function deleteAPI(endpoint) {
    return fetchAPI(endpoint, { method: 'DELETE' });
}