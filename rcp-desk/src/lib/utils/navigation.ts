/**
 * Navigation utilities for the application
 * This replaces the $app/navigation imports
 */

/**
 * Navigate to a different URL
 * @param url The URL to navigate to
 */
export function goto(url: string): void {
  // Use direct window.location for navigation after authentication
  // This ensures a full page reload which correctly evaluates authentication state
  window.location.href = url;
}