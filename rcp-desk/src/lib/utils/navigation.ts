/**
 * Navigation utilities for the application
 * This replaces the $app/navigation imports
 */

/**
 * Navigate to a different URL
 * @param url The URL to navigate to
 */
export function goto(url: string): void {
  window.location.href = url;
}