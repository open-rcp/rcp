import { writable } from 'svelte/store';

// Define the store type
interface SidebarState {
  isCollapsed: boolean;
}

// Check if screen is below medium breakpoint on initial load
const isMobileInitially = typeof window !== 'undefined' && window.innerWidth < 768;

// Create a writable store with initial value based on screen size
export const sidebarStore = writable<SidebarState>({
  isCollapsed: isMobileInitially
});

// Helper function to toggle sidebar state
export function toggleSidebar(): void {
  sidebarStore.update((state: SidebarState): SidebarState => ({
    ...state,
    isCollapsed: !state.isCollapsed
  }));
}