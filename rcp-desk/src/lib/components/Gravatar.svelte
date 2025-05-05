<script lang="ts">
  import { onMount } from 'svelte';
  import md5 from 'md5';

  // Component props
  export let email: string = '';
  export let size: number = 40;
  export let defaultImage: string = 'identicon';
  export let rating: string = 'g';
  export let className: string = '';
  
  // Computed Gravatar URL
  let gravatarUrl: string;
  
  // MD5 function is required for Gravatar
  // We'll check if md5 is available at runtime
  let md5Available: boolean = false;
  
  onMount(async () => {
    // If md5 is not available, try to import it dynamically
    if (typeof md5 !== 'function') {
      try {
        const md5Module = await import('md5');
        md5Available = true;
        updateGravatarUrl(md5Module.default);
      } catch (error) {
        console.error('Could not load md5 module:', error);
        // Use default image as fallback
        gravatarUrl = `https://www.gravatar.com/avatar/0?s=${size}&d=${defaultImage}&r=${rating}`;
      }
    } else {
      md5Available = true;
      updateGravatarUrl(md5);
    }
  });

  // Update the Gravatar URL when props change
  $: if (md5Available && typeof md5 === 'function' && email) {
    updateGravatarUrl(md5);
  }

  // Function to update Gravatar URL
  function updateGravatarUrl(hashFunction: Function) {
    if (!email) {
      gravatarUrl = `https://www.gravatar.com/avatar/0?s=${size}&d=${defaultImage}&r=${rating}`;
      return;
    }

    const hash = hashFunction(email.trim().toLowerCase());
    gravatarUrl = `https://www.gravatar.com/avatar/${hash}?s=${size}&d=${defaultImage}&r=${rating}`;
  }
</script>

<img 
  src={gravatarUrl || `https://www.gravatar.com/avatar/0?s=${size}&d=${defaultImage}&r=${rating}`} 
  alt="User Avatar" 
  width={size} 
  height={size} 
  class={className}
/>