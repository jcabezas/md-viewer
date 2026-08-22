<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import MarkdownIt from 'markdown-it'
import hljs from 'highlight.js'
import DOMPurify from 'dompurify'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import 'highlight.js/styles/github-dark.css'

const fileName = ref('Sin archivo')
const markdown = ref(`# Markdown Viewer

Bienvenido a **Markdown Viewer**.

Presiona **Abrir** para cargar un archivo Markdown.

## Ejemplo

Puedes abrir archivos:

- \`.md\`
- \`.markdown\`
- Archivos de texto Markdown

## Código

\`\`\`typescript
const message = "Hola Markdown!"

console.log(message)
\`\`\`
`)

const md = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true,

  highlight(code, language) {
    if (language && hljs.getLanguage(language)) {
      try {
        return hljs.highlight(code, {
          language,
          ignoreIllegals: true,
        }).value
      } catch {
        // Fallback al código normal
      }
    }

    return ''
  },
})

const renderedMarkdown = computed(() => {
  return DOMPurify.sanitize(md.render(markdown.value))
})

async function loadFilePath(path: string) {
  try {
    const content = await invoke<string>('read_markdown_file', { path })

    markdown.value = content

    const parts = path.split(/[\\/]/)
    fileName.value = parts[parts.length - 1]
  } catch (error) {
    console.error('Error al leer el archivo:', error)
  }
}

async function openMarkdownFile() {
  try {
    const path = await open({
      multiple: false,
      filters: [
        {
          name: 'Markdown',
          extensions: ['md', 'markdown', 'mdown', 'mkdn'],
        },
      ],
    })

    if (!path || Array.isArray(path)) {
      return
    }

    await loadFilePath(path)
  } catch (error) {
    console.error('Error al abrir el archivo:', error)
  }
}

async function loadPendingFile() {
  try {
    const path = await invoke<string | null>('get_pending_file')

    if (!path) {
      return
    }

    await loadFilePath(path)
  } catch (error) {
    console.error('Error al cargar archivo:', error)
  }
}

let unlistenOpenFile: (() => void) | undefined

onMounted(async () => {
  await loadPendingFile()

  unlistenOpenFile = await listen<string>('open-file', (event) => {
    loadFilePath(event.payload)
  })
})

onUnmounted(() => {
  unlistenOpenFile?.()
})
</script>

<template>
  <main class="app">
    <header class="toolbar">
      <div class="logo">
        <span class="logo-icon">M</span>
        <span>Markdown Viewer</span>
      </div>

      <div class="toolbar-right">
        <span class="file-name">
          {{ fileName }}
        </span>

        <button
          class="open-button"
          @click="openMarkdownFile"
        >
          📂 Abrir
        </button>
      </div>
    </header>

    <section class="content">
      <article
        class="markdown-body"
        v-html="renderedMarkdown"
      />
    </section>
  </main>
</template>

<style>
* {
  box-sizing: border-box;
}

html,
body,
#app {
  margin: 0;
  width: 100%;
  height: 100%;
}

body {
  font-family:
    Inter,
    -apple-system,
    BlinkMacSystemFont,
    "Segoe UI",
    sans-serif;

  background: #ffffff;
  color: #24292f;
}

.app {
  min-height: 100vh;
  background: #ffffff;
}

/* =========================
   Toolbar
   ========================= */

.toolbar {
  height: 56px;

  display: flex;
  align-items: center;
  justify-content: space-between;

  padding: 0 20px;

  border-bottom: 1px solid #e5e7eb;
  background: #ffffff;
}

.logo {
  display: flex;
  align-items: center;
  gap: 10px;

  font-size: 15px;
  font-weight: 600;
}

.logo-icon {
  display: flex;
  align-items: center;
  justify-content: center;

  width: 30px;
  height: 30px;

  border-radius: 6px;

  background: #24292f;
  color: white;

  font-weight: 700;
}

.toolbar-right {
  display: flex;
  align-items: center;
  gap: 12px;
}

.file-name {
  max-width: 300px;

  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;

  font-size: 13px;
  color: #6b7280;
}

.open-button {
  border: none;
  border-radius: 6px;

  padding: 8px 14px;

  background: #24292f;
  color: white;

  font-size: 13px;
  font-weight: 600;

  cursor: pointer;
}

.open-button:hover {
  background: #000000;
}

/* =========================
   Content
   ========================= */

.content {
  max-width: 900px;
  margin: 0 auto;
  padding: 48px 32px 80px;
}

.markdown-body {
  font-size: 16px;
  line-height: 1.7;
}

.markdown-body h1 {
  margin-top: 0;
  margin-bottom: 24px;

  font-size: 36px;
  line-height: 1.2;
}

.markdown-body h2 {
  margin-top: 40px;
  margin-bottom: 16px;

  font-size: 26px;

  border-bottom: 1px solid #e5e7eb;
  padding-bottom: 8px;
}

.markdown-body h3 {
  margin-top: 30px;
  font-size: 21px;
}

.markdown-body p {
  margin: 16px 0;
}

/* =========================
   Lists
   ========================= */

.markdown-body ul,
.markdown-body ol {
  padding-left: 28px;
}

.markdown-body li {
  margin: 6px 0;
}

/* =========================
   Blockquote
   ========================= */

.markdown-body blockquote {
  margin: 20px 0;
  padding: 4px 20px;

  border-left: 4px solid #d1d5db;

  color: #6b7280;
  background: #f9fafb;
}

/* =========================
   Inline code
   ========================= */

.markdown-body :not(pre) > code {
  padding: 2px 6px;

  border-radius: 4px;

  background: #f3f4f6;
  color: #24292f;

  font-family:
    "Cascadia Code",
    Consolas,
    monospace;

  font-size: 0.9em;
}

/* =========================
   Code blocks
   ========================= */

.markdown-body pre {
  margin: 24px 0;
  padding: 20px;

  overflow-x: auto;

  border-radius: 8px;

  background: #0d1117;

  box-shadow:
    inset 0 0 0 1px rgba(255, 255, 255, 0.08);
}

.markdown-body pre code {
  display: block;

  padding: 0;

  background: transparent !important;

  color: #e6edf3;

  font-family:
    "Cascadia Code",
    "JetBrains Mono",
    Consolas,
    monospace;

  font-size: 14px;
  line-height: 1.6;
}

/* =========================
   Syntax highlighting
   ========================= */

.markdown-body pre code .hljs-keyword {
  color: #ff7b72;
}

.markdown-body pre code .hljs-string {
  color: #a5d6ff;
}

.markdown-body pre code .hljs-title,
.markdown-body pre code .hljs-title.class_,
.markdown-body pre code .hljs-title.function_ {
  color: #d2a8ff;
}

.markdown-body pre code .hljs-number {
  color: #79c0ff;
}

.markdown-body pre code .hljs-comment {
  color: #8b949e;
}

.markdown-body pre code .hljs-built_in {
  color: #ffa657;
}

.markdown-body pre code .hljs-variable,
.markdown-body pre code .hljs-attr {
  color: #79c0ff;
}

/* =========================
   Tables
   ========================= */

.markdown-body table {
  width: 100%;

  margin: 24px 0;

  border-collapse: collapse;

  border: 1px solid #d0d7de;

  font-size: 15px;
}

.markdown-body th,
.markdown-body td {
  padding: 10px 14px;

  border: 1px solid #d0d7de;

  text-align: left;

  vertical-align: top;
}

.markdown-body th {
  background: #f6f8fa;

  font-weight: 600;
}

.markdown-body tr:nth-child(even) {
  background: #fafbfc;
}

/* =========================
   Links
   ========================= */

.markdown-body a {
  color: #0969da;
  text-decoration: none;
}

.markdown-body a:hover {
  text-decoration: underline;
}

/* =========================
   Horizontal rule
   ========================= */

.markdown-body hr {
  margin: 32px 0;

  border: 0;
  border-top: 1px solid #d8dee4;
}

/* =========================
   Images
   ========================= */

.markdown-body img {
  max-width: 100%;
  height: auto;

  border-radius: 8px;
}

/* =========================
   Task lists
   ========================= */

.markdown-body input[type="checkbox"] {
  margin-right: 8px;
}
</style>
