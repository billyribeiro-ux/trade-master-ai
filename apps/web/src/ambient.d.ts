/// <reference types="svelte" />
/// <reference types="vite/client" />

// Svelte 5 runes are globally available in .svelte.ts files
declare function $state<T>(initial: T): T;
declare function $derived<T>(fn: () => T): T;
declare function $effect(fn: () => void | (() => void)): void;
declare function $props<T>(): T;
