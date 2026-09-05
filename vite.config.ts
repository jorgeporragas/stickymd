import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { resolve } from 'node:path';

// One entry point per window type. See CLAUDE.md section 'Project structure'.
const windows = {
  note: resolve(import.meta.dirname, 'src/windows/note/index.html')
};

export default defineConfig({
  plugins: [svelte()],

  // Tauri owns the terminal; Vite must not wipe its output.
  clearScreen: false,

  server: {
    port: 1420,
    strictPort: true,
    // Rust rebuilds are Cargo's business, not Vite's.
    watch: { ignored: ['**/src-tauri/**'] }
  },

  build: {
    rollupOptions: { input: windows }
  }
});
