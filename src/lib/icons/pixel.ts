/**
 * Pixel glyphs, from Pixelarticons.
 *
 * MIT, Gerrit Halfmann — the licence text is vendored beside them at
 * `src/assets/icons/LICENSE-Pixelarticons.txt`, the same way the typefaces
 * carry theirs. Only the six paths the insert menu uses are here; the set has
 * a thousand, and nothing fetches anything at runtime.
 *
 * They are drawn on a 24-unit grid and are **pixel art**, which is the one
 * thing to know before using them anywhere else: they only look right at 24px
 * or an integer multiple of it. At the 9px the window's own controls use, a
 * design pixel is under half a screen pixel and the glyph turns to mush —
 * which is why the chrome keeps its hand-drawn strokes and this set is used
 * only where a bubble is large enough to carry it.
 *
 * They fill rather than stroke. A stroke rule applied to these would draw
 * nothing.
 */

export type PixelIcon = 'table' | 'code' | 'task' | 'list' | 'numbered' | 'link';

export const PIXEL_ICONS: Record<PixelIcon, string> = {
  // grid-3x3
  table:
    'M4 2h16v2H4zm0 18h16v2H4zM2 4h2v16H2zm18 0h2v16h-2zM4 8h16v2H4zm0 6h16v2H4zM8 4h2v16H8zm6 0h2v16h-2z',

  code: 'M11 18H9v-4h2v4Zm-4-1H5v-2h2v2Zm12-2v2h-2v-2h2ZM5 15H3v-2h2v2Zm16 0h-2v-2h2v2Zm-8-1h-2v-4h2v4ZM3 13H1v-2h2v2Zm20 0h-2v-2h2v2ZM5 11H3V9h2v2Zm16 0h-2V9h2v2Zm-6-1h-2V6h2v4ZM7 9H5V7h2v2Zm12 0h-2V7h2v2Z',

  // checkbox-on. The ticked one, not the empty one: an empty box at this size
  // is a square, and the ring already has a square in the table.
  task: 'M4 2h16v2H4zm0 18h16v2H4zM2 4h2v16H2zm18 0h2v16h-2zM7 12h2v2H7zm2 2h2v2H9zm2-2h2v2h-2zm2-2h2v2h-2zm2-2h2v2h-2z',

  // bulletlist
  list: 'M10 5h12v2H10zm0 4h8v2h-8zm0 4h12v2H10zm0 4h8v2h-8zm-4-6H4V9h2v2ZM4 9H2V7h2v2Zm4 0H6V7h2v2ZM6 7H4V5h2v2Zm-2 6h2v2H4zm0 4h2v2H4zm-2 0v-2h2v2zm4 0v-2h2v2z',

  // list-box. The nearest the set has to a numbered list — the hovered label
  // in the ring says which it is, so the glyph carries less than it would in a
  // toolbar with no words.
  numbered:
    'M4 2h16v2H4zm2 5h2v2H6zm4 0h8v2h-8zm-4 4h2v2H6zm4 0h8v2h-8zm-4 4h2v2H6zm4 0h8v2h-8zm-6 5h16v2H4zM2 4h2v16H2zm18 0h2v16h-2z',

  link: 'M4 6h7v2H4zm0 10h7v2H4zM2 8h2v8H2zm18-2h-7v2h7zm0 10h-7v2h7zm2-8h-2v8h2zM7 11h10v2H7z'
};
