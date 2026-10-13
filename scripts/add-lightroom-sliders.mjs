import fs from 'node:fs';

const html = fs.readFileSync('index.html', 'utf8');
let js = fs.readFileSync('src/main.js', 'utf8');

const sl = (id, label, min, max, def, step) =>
  `                <div class="retouch-slider-item"><div class="s-head"><span>${label}</span><span class="val" data-for="${id}">${def}</span></div><input type="range" class="retouch-slider" id="${id}" min="${min}" max="${max}" value="${def}"${step ? ` step="${step}"` : ''} /></div>`;

const group = (title, body) =>
  `            <div class="retouch-acc-group">\n              <div class="retouch-acc-title">▾ ${title}</div>\n              <div class="retouch-acc-body">\n${body}\n              </div>\n            </div>\n`;

const groups = [
  group('Black &amp; White Mixer', [
    `                <select id="treatment" class="retouch-select" style="margin-bottom:8px;"><option value="color">Color</option><option value="bw">Black &amp; White</option></select>`,
    sl('bwRed', 'Red', -100, 100, 0),
    sl('bwOrange', 'Orange', -100, 100, 0),
    sl('bwYellow', 'Yellow', -100, 100, 0),
    sl('bwGreen', 'Green', -100, 100, 0),
    sl('bwAqua', 'Aqua', -100, 100, 0),
    sl('bwBlue', 'Blue', -100, 100, 0),
    sl('bwPurple', 'Purple', -100, 100, 0),
    sl('bwMagenta', 'Magenta', -100, 100, 0),
  ].join('\n')),
  group('Color Grading — Midtones, Global &amp; Luminance', [
    sl('gradeMidHue', 'Midtone Hue', 0, 360, 120),
    sl('gradeMidSat', 'Midtone Amount', 0, 100, 0),
    sl('gradeGlobalHue', 'Global Hue', 0, 360, 45),
    sl('gradeGlobalSat', 'Global Amount', 0, 100, 0),
    sl('gradeShadowLum', 'Shadow Luminance', -100, 100, 0),
    sl('gradeMidLum', 'Midtone Luminance', -100, 100, 0),
    sl('gradeHighLum', 'Highlight Luminance', -100, 100, 0),
    sl('gradeGlobalLum', 'Global Luminance', -100, 100, 0),
    sl('gradeBlending', 'Blending', 0, 100, 50),
    sl('gradeBalance', 'Balance', -100, 100, 0),
  ].join('\n')),
  group('Detail — Sharpen Radius, Detail &amp; Masking', [
    sl('sharpenRadius', 'Sharpen Radius', 0.5, 3, 1, 0.1),
    sl('sharpenDetail', 'Sharpen Detail', 0, 100, 25),
    sl('sharpenMasking', 'Sharpen Masking', 0, 100, 0),
    sl('nrColor', 'Color Noise Reduction', 0, 100, 0),
  ].join('\n')),
  group('Effects — Grain &amp; Vignette Controls', [
    sl('grainSize', 'Grain Size', 0, 100, 25),
    sl('grainRoughness', 'Grain Roughness', 0, 100, 50),
    sl('vignetteMidpoint', 'Vignette Midpoint', 0, 100, 50),
    sl('vignetteRoundness', 'Vignette Roundness', -100, 100, 0),
    sl('vignetteFeather', 'Vignette Feather', 0, 100, 50),
    sl('vignetteHighlights', 'Vignette Highlights', 0, 100, 0),
  ].join('\n')),
  group('Lens Corrections', [
    sl('opticsDistortion', 'Distortion', -100, 100, 0),
    sl('opticsVignetting', 'Lens Vignetting Correction', -100, 100, 0),
    sl('opticsVignettingMidpoint', 'Vignetting Midpoint', 0, 100, 50),
    sl('opticsCaRed', 'Chromatic Aberration — Red/Cyan', -100, 100, 0),
    sl('opticsCaBlue', 'Chromatic Aberration — Blue/Yellow', -100, 100, 0),
    sl('opticsDefringePurple', 'Defringe Purple', 0, 100, 0),
    sl('opticsDefringeGreen', 'Defringe Green', 0, 100, 0),
  ].join('\n')),
].join('\n');

const anchor = 'id="vignette" min="-100" max="100" value="0" /></div>\n              </div>\n            </div>\n';
if (!html.includes(anchor)) throw new Error('vignette anchor not found');
let out = html.replace(anchor, anchor + '\n' + groups);

// Geometry group in the Crop module
const flipAnchor = '<button id="btn-flip-h"';
if (!out.includes(flipAnchor)) throw new Error('flip anchor not found');
const geo = group('Geometry', [
  sl('geoScale', 'Scale (%)', 50, 200, 100),
  sl('geoOffsetX', 'Offset X', -100, 100, 0),
  sl('geoOffsetY', 'Offset Y', -100, 100, 0),
  `                <button id="btn-flip-v" class="retouch-sync-btn" style="width:100%;">⇅ Flip Vertical Mirror</button>`,
].join('\n'));
const cropMarker = '<!-- FAR-RIGHT';
const idx = out.indexOf(cropMarker);
if (idx < 0) throw new Error('crop end marker not found');
// insert before the "</div>" that closes mod-crop: find last "        </div>" before marker
const before = out.lastIndexOf('</div>', idx);
out = out.slice(0, before) + '\n' + geo + '          ' + out.slice(before);
fs.writeFileSync('index.html', out);

// JS defaults
const defaults = `
    // Lightroom-parity extras
    treatment: 'color',
    bwRed: 0, bwOrange: 0, bwYellow: 0, bwGreen: 0, bwAqua: 0, bwBlue: 0, bwPurple: 0, bwMagenta: 0,
    gradeMidHue: 120, gradeMidSat: 0, gradeGlobalHue: 45, gradeGlobalSat: 0,
    gradeShadowLum: 0, gradeMidLum: 0, gradeHighLum: 0, gradeGlobalLum: 0,
    gradeBlending: 50, gradeBalance: 0,
    sharpenRadius: 1, sharpenDetail: 25, sharpenMasking: 0, nrColor: 0,
    grainSize: 25, grainRoughness: 50,
    vignetteMidpoint: 50, vignetteRoundness: 0, vignetteFeather: 50, vignetteHighlights: 0,
    opticsDistortion: 0, opticsVignetting: 0, opticsVignettingMidpoint: 50,
    opticsCaRed: 0, opticsCaBlue: 0, opticsDefringePurple: 0, opticsDefringeGreen: 0,
    geoScale: 100, geoOffsetX: 0, geoOffsetY: 0, flipVertical: false,
`;
js = js.replace("    lutPreset: 'none',\n", "    lutPreset: 'none',\n" + defaults);
js = js.split("['lutPreset', 'bgReplaceMode', 'makeupLipTint', 'hairDyeColor']").join("['lutPreset', 'treatment', 'bgReplaceMode', 'makeupLipTint', 'hairDyeColor']");
js = js.replace(
  "  // Flip Horizontal Button",
  `  // Flip Vertical Button
  document.getElementById('btn-flip-v')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    photo.params.flipVertical = !photo.params.flipVertical;
    processActivePhoto(false);
  });

  // Flip Horizontal Button`
);
fs.writeFileSync('src/main.js', js);
console.log('patched');
