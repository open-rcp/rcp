/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#f6f8fa',
          100: '#edf1f5',
          200: '#dbe4ec',
          300: '#c1d0de',
          400: '#a0b7ca',
          500: '#839db4',
          600: '#6b83a3',
          700: '#536c8e', // Proxmox primary blue
          800: '#1f356e', // Proxmox darker blue
          900: '#162752',
          950: '#0f1a36',
        },
        secondary: {
          // Proxmox orange
          50: '#fff8f1',
          100: '#feecdc',
          200: '#fcd9b8',
          300: '#fbbc8c',
          400: '#f89b5c',
          500: '#f6802c', // Proxmox accent orange
          600: '#e05e17',
          700: '#ba4912',
          800: '#973a14',
          900: '#7a3114',
          950: '#451a09',
        },
        gray: {
          50: '#f9fafb',
          100: '#f3f4f6',
          200: '#e5e7eb',
          300: '#d1d5db',
          400: '#9ca3af',
          500: '#6b7280',
          600: '#4b5563',
          700: '#374151',
          800: '#1f2937',
          900: '#111827',
          950: '#030712',
        },
      },
    },
  },
  plugins: [],
}