<script lang="ts">
  // AuthLayout component for login and other authentication-related pages
</script>

<div class="auth-layout">
  <div class="min-h-screen flex flex-col bg-gray-100">
    <!-- Main content area with consistent spacing -->
    <div class="flex-1 flex items-center justify-center p-4">
      <slot />
    </div>
    <div class="container mx-auto px-4 my-4 text-center text-sm text-gray-500">
        &copy; {new Date().getFullYear()} RCP Desk • All rights reserved
      </div>
  </div>
</div>

<style>
  .auth-layout {
    display: flex;
    width: 100%;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
  }
</style>