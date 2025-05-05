// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare namespace App {
	// interface Locals {}
	// interface PageData {}
	// interface PageState {}
	// interface Platform {}
}

// Declare module for $app/stores
declare module '$app/stores' {
	import { readable } from 'svelte/store';

	export interface Page {
		url: URL;
		params: Record<string, string>;
		routeId: string | null;
		status: number;
		error: Error | null;
		data: Record<string, any>;
		form: Record<string, any> | null;
	}

	export const page: import('svelte/store').Readable<Page>;
	export const navigating: import('svelte/store').Readable<{
		from: URL;
		to: URL;
	} | null>;
}

// Declare module for $lib/stores/auth
declare module '$lib/stores/auth' {
	import { Writable } from 'svelte/store';

	interface User {
		id: string;
		name: string;
		email: string;
		roles: string[];
	}

	interface AuthStore {
		isAuthenticated: boolean;
		user: User | null;
		token: string | null;
		login: (user: User, token: string) => void;
		logout: () => void;
		updateUser: (user: User) => void;
	}

	export const authStore: import('svelte/store').Readable<AuthStore>;
}