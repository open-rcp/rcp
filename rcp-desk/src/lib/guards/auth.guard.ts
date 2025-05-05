import { goto } from '$utils/navigation';
import { authService } from '$services/auth.service';

// Auth guard to protect routes
export async function authGuard(path: string): Promise<boolean> {
  // If user is authenticated, allow access
  if (authService.isAuthenticated()) {
    return true;
  }

  // Try to initialize auth from refresh token
  const success = await authService.initAuth();
  if (success) {
    return true;
  }

  // Not authenticated, redirect to login with returnUrl
  goto(`/login?returnUrl=${encodeURIComponent(path)}`);
  return false;
}