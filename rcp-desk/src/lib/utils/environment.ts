/**
 * Environment utilities for the application
 * This replaces the $app/environment imports
 */

/**
 * Whether the app is running in a browser environment
 */
export const browser: boolean = typeof window !== 'undefined';