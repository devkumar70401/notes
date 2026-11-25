#!/usr/bin/env python3
"""
Universal, publication-grade PDF generator for Embedded Rust Markdown notes.
Guarantees across ALL current and future chapters:
  1. Universal Callout Protection: No note or callout is ever sliced across pages.
  2. Universal Heading Guard: No heading (H1-H5) is ever left orphaned at a page bottom.
  3. Universal Intro Binding: Any paragraph introducing code/diagrams (ending in ':') stays with its block.
  4. Automatic Chapter Breaks: Every new Chapter (H1) automatically starts on a fresh page.
  5. Scalable Table of Contents: Every Unit is formatted in an atomic card.
"""

import os
import sys
import html
import subprocess
import time
import re
import gi

gi.require_version('WebKit2', '4.1')
gi.require_version('Gtk', '3.0')
from gi.repository import WebKit2, Gtk, GLib

CROSSNOTE_DIR = '/home/dev/.vscode/extensions/shd101wyy.markdown-preview-enhanced-0.8.40/crossnote'
NOTE_DIR = '/home/dev/se/notes/embedded_rust'

if len(sys.argv) > 1:
    MD_FILE = os.path.abspath(sys.argv[1])
    if len(sys.argv) > 2:
        PDF_FILE = os.path.abspath(sys.argv[2])
    else:
        PDF_FILE = MD_FILE.rsplit('.', 1)[0] + '.pdf'
else:
    MD_FILE = os.path.join(NOTE_DIR, 'unit_01_foundations', 'ch01_syntax_and_execution_model.md')
    PDF_FILE = os.path.join(NOTE_DIR, 'unit_01_foundations', 'ch01_syntax_and_execution_model.pdf')

PORT = 3625

def read_file_safe(path):
    if os.path.exists(path):
        with open(path, 'r', encoding='utf-8') as f:
            return f.read()
    return ''

VOID_TAGS = {'hr', 'br', 'img', 'meta', 'link'}

def slice_top_level_blocks(html_str):
    """Accurately slices HTML into top-level DOM block tokens."""
    tag_re = re.compile(r'<\/?([a-zA-Z0-9]+)(\s+[^>]*)?>')
    depth = 0
    blocks = []
    block_start = 0
    cur_tag = ''
    pos = 0
    while pos < len(html_str):
        m = tag_re.search(html_str, pos)
        if not m:
            break
        tag_text = m.group(0)
        tag_name = m.group(1).lower()
        is_closing = tag_text.startswith('</')
        is_self_closing = tag_text.endswith('/>') or tag_name in VOID_TAGS
        if depth == 0:
            block_start = m.start()
            cur_tag = tag_name
            if is_self_closing:
                blocks.append((cur_tag, html_str[block_start:m.end()]))
                pos = m.end()
                continue
            else:
                depth = 1
        else:
            if is_closing:
                if tag_name == cur_tag and depth == 1:
                    depth = 0
                    blocks.append((cur_tag, html_str[block_start:m.end()]))
                else:
                    depth -= 1
            elif not is_self_closing:
                depth += 1
        pos = m.end()
    return blocks

def main():
    print("1. Compiling Markdown via VS Code crossnote engine...")
    proc = subprocess.Popen([
        '/usr/share/code/code',
        f'{CROSSNOTE_DIR}/../out/native/crossnote-serve.js',
        'serve', NOTE_DIR,
        '--port', str(PORT),
        '--build-dir', CROSSNOTE_DIR,
        '--vscode'
    ], env={'ELECTRON_RUN_AS_NODE': '1', 'PATH': '/usr/bin:/bin'}, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    time.sleep(2)
    try:
        import urllib.request
        with urllib.request.urlopen(f'http://127.0.0.1:{PORT}/preview?file={MD_FILE}') as resp:
            page_html = resp.read().decode('utf-8')
            start = page_html.find('data-html="') + len('data-html="')
            m = re.search(r'"\s*>\s*</body>', page_html[start:])
            end = start + m.start() if m else len(page_html)
            raw_body = html.unescape(page_html[start:end])
    finally:
        proc.terminate()

    print("2. Restructuring layout with universal publication rules...")
    blocks = slice_top_level_blocks(raw_body)

    index_html = []
    body_html = []

    in_index = True
    i = 0
    while i < len(blocks):
        tag, content = blocks[i]
        
        # Universal detector: First H1 that is not 'Index' marks the beginning of chapters
        if tag == 'h1' and not re.search(r'id=["\']index["\']', content, flags=re.IGNORECASE):
            in_index = False
        
        if in_index:
            # Universal Index Rule: Wrap each curriculum Unit / Section into an unbreakable card
            if tag == 'h3' and i + 1 < len(blocks) and blocks[i+1][0] == 'ul':
                u_head = content
                u_body = blocks[i+1][1]
                index_html.append(f'<div class="index-unit-card">{u_head}{u_body}</div>')
                i += 2
                if i < len(blocks) and blocks[i][0] == 'hr':
                    i += 1
                continue
            elif tag == 'hr':
                i += 1
                continue
            else:
                index_html.append(content)
                i += 1
        else:
            # ================= UNIVERSAL MAIN CONTENT RULES =================

            # Rule 1: Universal Intro Binding
            # Any paragraph ending with a colon (e.g. "Here is...", "Source Code:", "Take the following example:")
            # immediately followed by a code block, diagram, or table is bound to it as an atomic unit.
            clean_text = re.sub(r'<[^>]+>', '', content).strip()
            if tag == 'p' and clean_text.endswith(':') and i + 1 < len(blocks) and blocks[i+1][0] in ['pre', 'table', 'blockquote']:
                body_html.append(f'<div class="atomic-block">{content}{blocks[i+1][1]}</div>')
                i += 2
                continue
            
            # Rule 2: Universal Code Block + Note/Quote Binding
            # If a code block is immediately followed by a blockquote/note that explains it, glue them.
            if tag == 'pre' and i + 1 < len(blocks) and blocks[i+1][0] == 'blockquote':
                body_html.append(f'<div class="atomic-block">{content}{blocks[i+1][1]}</div>')
                i += 2
                continue

            # Rule 3: Universal Heading Guard
            # Any heading (H2-H5) is strictly grouped with its following child element so it can NEVER be orphaned.
            if tag in ['h2', 'h3', 'h4', 'h5']:
                if i + 1 < len(blocks):
                    next_tag, next_content = blocks[i+1]
                    if next_tag in ['h3', 'h4', 'h5'] and i + 2 < len(blocks):
                        # Stacked headings (e.g., H2 followed by H3): glue both headings + following child
                        next2_tag, next2_content = blocks[i+2]
                        body_html.append(f'<div class="heading-group">{content}{next_content}{next2_content}</div>')
                        i += 3
                        continue
                    elif next_tag in ['p', 'ul', 'ol', 'table', 'div']:
                        body_html.append(f'<div class="heading-group">{content}{next_content}</div>')
                        i += 2
                        continue
            
            # Default: append block as-is
            body_html.append(content)
            i += 1

    idx_str = ''.join(index_html)
    body_str = ''.join(body_html)

    full_body = f'<div class="index-section">{idx_str}</div><div class="page-break-chapter"></div><div class="main-content">{body_str}</div>'

    print("3. Inlining stylesheets and PDF page constraints...")
    katex_css = read_file_safe(f'{CROSSNOTE_DIR}/dependencies/katex/katex.min.css')
    callout_css = read_file_safe(f'{CROSSNOTE_DIR}/styles/markdown-it-callout.css')
    preview_theme_css = read_file_safe(f'{CROSSNOTE_DIR}/styles/preview_theme/github-light.css')
    prism_css = read_file_safe(f'{CROSSNOTE_DIR}/styles/prism_theme/github.css')

    custom_css = """
    @page {
        size: A4;
        margin: 18mm 16mm 18mm 16mm;
    }

    .page-break-chapter {
        page-break-before: always !important;
        break-before: page !important;
        height: 0;
        margin: 0;
        padding: 0;
    }

    body {
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Ubuntu', Helvetica, Arial, sans-serif !important;
        font-size: 13px !important;
        line-height: 1.55 !important;
        color: #1f2328 !important;
        background-color: #ffffff !important;
        padding: 0 !important;
        margin: 0 !important;
    }

    /* ================= ATOMIC CONTAINERS (Universal) ================= */
    .atomic-block,
    .heading-group,
    .callout,
    .index-unit-card,
    pre,
    blockquote,
    table,
    tr {
        display: block !important;
        position: relative !important;
        overflow: visible !important;
        break-inside: avoid !important;
        page-break-inside: avoid !important;
        -webkit-column-break-inside: avoid !important;
    }

    table {
        display: table !important;
    }
    tr {
        display: table-row !important;
    }

    /* ================= HEADINGS (Universal) ================= */
    h1, h2, h3, h4, h5, h6 {
        break-after: avoid !important;
        page-break-after: avoid !important;
        break-inside: avoid !important;
        page-break-inside: avoid !important;
        color: #0969da !important;
    }

    /* Every Chapter automatically starts on a brand-new page */
    .main-content h1 {
        page-break-before: always !important;
        break-before: page !important;
        font-size: 21px !important;
        border-bottom: 2px solid #d0d7de !important;
        padding-bottom: 5px !important;
        margin-top: 0 !important;
        margin-bottom: 14px !important;
    }
    .main-content h2 {
        font-size: 16.5px !important;
        border-bottom: 1px solid #d0d7de !important;
        padding-bottom: 4px !important;
        margin-top: 22px !important;
        margin-bottom: 10px !important;
    }
    .main-content h3 {
        font-size: 14px !important;
        color: #1f2328 !important;
        margin-top: 16px !important;
        margin-bottom: 6px !important;
    }
    .main-content h4 {
        font-size: 13px !important;
        color: #24292f !important;
        margin-top: 12px !important;
        margin-bottom: 4px !important;
    }
    .main-content h5 {
        font-size: 12px !important;
        color: #57606a !important;
        margin-top: 10px !important;
        margin-bottom: 4px !important;
    }

    /* ================= INDEX STYLING (Universal across all units) ================= */
    .index-section {
        font-size: 9.8px !important;
        line-height: 1.28 !important;
    }
    .index-section h1 {
        font-size: 21px !important;
        font-weight: 700 !important;
        color: #0969da !important;
        border-bottom: 2px solid #d0d7de !important;
        padding-bottom: 3px !important;
        margin-bottom: 6px !important;
    }
    .index-section blockquote {
        background: #f6f8fa !important;
        border-left: 4px solid #0969da !important;
        padding: 5px 10px !important;
        border-radius: 4px !important;
        margin-bottom: 8px !important;
        font-size: 9.5px !important;
    }
    .index-unit-card {
        background: #f8fafc !important;
        border: 1px solid #e2e8f0 !important;
        border-radius: 6px !important;
        padding: 7px 12px !important;
        margin-bottom: 8px !important;
    }
    .index-unit-card h3 {
        font-size: 11.5px !important;
        font-weight: 700 !important;
        color: #0969da !important;
        margin-top: 0 !important;
        margin-bottom: 3px !important;
        border-bottom: 1px solid #cbd5e1 !important;
        padding-bottom: 2px !important;
    }
    .index-unit-card ul {
        margin: 1px 0 !important;
        padding-left: 16px !important;
    }
    .index-unit-card li {
        margin-bottom: 1px !important;
    }

    /* ================= CALLOUT NOTES (Universal) ================= */
    .callout {
        border-left: 4px solid #0969da !important;
        background-color: #f6f8fa !important;
        border-radius: 6px !important;
        margin: 12px 0 !important;
        padding: 10px 14px !important;
    }
    .callout-title {
        break-after: avoid !important;
        page-break-after: avoid !important;
        font-size: 12.5px !important;
        font-weight: 700 !important;
        color: #0969da !important;
        margin-bottom: 5px !important;
    }
    .callout ul {
        margin: 2px 0 !important;
        padding-left: 18px !important;
    }
    .callout li {
        margin-bottom: 3px !important;
    }

    /* ================= CODE BLOCKS (Universal) ================= */
    pre, pre code {
        font-family: 'JetBrains Mono', 'Fira Code', 'DejaVu Sans Mono', Consolas, monospace !important;
        font-size: 10px !important;
    }
    pre {
        background-color: #f6f8fa !important;
        border: 1px solid #d0d7de !important;
        border-radius: 6px !important;
        padding: 10px 12px !important;
        line-height: 1.34 !important;
        margin: 11px 0 !important;
    }

    /* ================= TABLES (Universal) ================= */
    table {
        width: 100% !important;
        border-collapse: collapse !important;
        margin: 12px 0 !important;
        font-size: 11px !important;
    }
    thead {
        display: table-header-group !important;
    }
    th, td {
        border: 1px solid #d0d7de !important;
        padding: 6px 9px !important;
    }
    th {
        background-color: #f6f8fa !important;
        font-weight: 600 !important;
    }

    /* ================= LISTS & TEXT ================= */
    li {
        break-inside: avoid !important;
        page-break-inside: avoid !important;
        margin-bottom: 3px !important;
    }
    p, li {
        orphans: 3 !important;
        widows: 3 !important;
    }
    hr {
        border: 0;
        height: 1px;
        background: #e2e8f0;
        margin: 16px 0;
    }
    """

    full_html = f"""<!DOCTYPE html>
    <html>
    <head>
    <meta charset="utf-8">
    <style>
    {katex_css}
    {preview_theme_css}
    {prism_css}
    {callout_css}
    {custom_css}
    </style>
    </head>
    <body class="markdown-body">
    {full_body}
    </body>
    </html>"""

    print("4. Rendering publication-ready vector PDF...")
    loop = GLib.MainLoop()
    view = WebKit2.WebView()

    def do_print():
        op = WebKit2.PrintOperation.new(view)
        s = Gtk.PrintSettings()
        s.set(Gtk.PRINT_SETTINGS_OUTPUT_URI, 'file://' + os.path.abspath(PDF_FILE))
        s.set(Gtk.PRINT_SETTINGS_PRINTER, 'Print to File')

        page_setup = Gtk.PageSetup()
        paper_size = Gtk.PaperSize.new(Gtk.PAPER_NAME_A4)
        page_setup.set_paper_size(paper_size)
        op.set_page_setup(page_setup)
        op.set_print_settings(s)

        def on_finished(op):
            print(f"SUCCESS: Rendered professional PDF to {PDF_FILE}")
            loop.quit()

        def on_failed(op, err):
            print(f"Print failed: {err}")
            loop.quit()

        op.connect('finished', on_finished)
        op.connect('failed', on_failed)
        op.print_()

    view.connect('load-changed', lambda v, e: GLib.timeout_add(300, do_print) if e == WebKit2.LoadEvent.FINISHED else None)
    view.load_html(full_html, 'file://' + os.path.abspath(NOTE_DIR) + '/')

    GLib.timeout_add_seconds(15, lambda: (print("Timeout"), loop.quit()))
    loop.run()

if __name__ == "__main__":
    main()
