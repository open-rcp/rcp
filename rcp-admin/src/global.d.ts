// Global declarations for Svelte components
declare namespace svelteHTML {
    interface HTMLAttributes<T> {
        // Svelte on:* event handlers
        'on:click'?: (event: CustomEvent<any> & { target: EventTarget & T }) => any;
        'on:change'?: (event: CustomEvent<any> & { target: EventTarget & T }) => any;
        'on:input'?: (event: CustomEvent<any> & { target: EventTarget & T }) => any;
        'on:focus'?: (event: CustomEvent<any> & { target: EventTarget & T }) => any;
        'on:blur'?: (event: CustomEvent<any> & { target: EventTarget & T }) => any;
        'on:mouseenter'?: (event: CustomEvent<any> & { target: EventTarget & T }) => any;
        'on:mouseleave'?: (event: CustomEvent<any> & { target: EventTarget & T }) => any;
        'on:submit'?: (event: CustomEvent<any> & { target: EventTarget & T }) => any;
        
        // Svelte binding directives
        'bind:value'?: any;
        'bind:group'?: any;
        'bind:checked'?: boolean;
        'bind:this'?: any;
        
        // Svelte class directives
        'class:active'?: boolean;
        'class:selected'?: boolean;
        
        // Other Svelte directives
        'use:action'?: any;
        'in:transition'?: any;
        'out:transition'?: any;
        'transition:name'?: any;
    }
}
