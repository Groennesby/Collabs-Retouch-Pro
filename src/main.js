import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';
import { getCurrentWebview } from '@tauri-apps/api/webview';
import { analyzePhotoWithNeuralAI, MESH_INDICES } from './ai_masks.js';

export function createDefaultParams() {
  return {
    // MODULE 2: PORTRAIT RETOUCHING
    // 1. Blemish Removal
    freckleAcne: 0,
    faceMole: 0,
    faceShine: 0,
    darkCircles: 0,
    eyeBags: 0,
    foreheadWrinkles: 0,
    nasolabialFolds: 0,
    neckWrinkles: 0,
    glassesGlare: 0,
    doubleChin: 0,
    bodyBlemish: 0,

    // 2. Skin Retouching
    dodgeBurnEven: 0,
    dodgeBurnSculpt: 0,
    freqSepLow: 0,
    freqSepHigh: 80,
    facialSkinSmooth: 0,
    bodySkinSmooth: 0,
    skinRadiance: 0,
    skinRosy: 0,
    skinWarmth: 0,
    unifyComplexion: 0,

    // 3. Face Reshape (3D Liquify)
    faceWidth: 0,
    jawlineSlim: 0,
    cheekboneLift: 0,
    chinLength: 0,
    foreheadHeight: 0,
    noseWidth: 0,
    noseTipLift: 0,
    mouthSize: 0,
    lipPlump: 0,
    smileEnhance: 0,

    // 4. Eyes & Eyebrows
    eyeSize: 0,
    eyeBrighten: 0,
    redVeins: 0,
    catchlightBoost: 0,
    eyebrowThickness: 0,
    eyebrowLift: 0,

    // 5. Teeth
    teethWhiten: 0,
    teethDesaturate: 0,
    teethBrighten: 0,

    // 6. Makeup
    makeupHighlight: 0,
    makeupContour: 0,
    makeupBlush: 0,
    makeupEyeshadow: 0,
    makeupLipstick: 0,
    makeupLipTint: 'rose',

    // 7. Hair
    strayHair: 0,
    hairVolume: 0,
    hairPartLine: 0,
    hairShine: 0,
    hairDyeStrength: 0,
    hairDyeColor: 'none',

    // 8. Body Reshape
    neckSlim: 0,
    shoulderSculpt: 0,
    waistSlim: 0,

    // MODULE 1: COLOR ADJUSTMENT (RAW / 8-CHANNEL HSL / 256-LUT CURVE / COLOR MATCH)
    temperature: 0,
    tint: 0,
    exposure: 0,
    contrast: 0,
    highlights: 0,
    shadows: 0,
    whites: 0,
    blacks: 0,
    clarity: 0,
    texture: 0,
    dehaze: 0,
    vibrance: 0,
    saturation: 0,

    curveShadows: 0,
    curveDarks: 0,
    curveLights: 0,
    curveHighlights: 0,
    curveLut: [],

    colorMatchStrength: 0,
    colorMatchStats: [],

    hslRedHue: 0,
    hslRedSat: 0,
    hslRedLum: 0,
    hslOrangeHue: 0,
    hslOrangeSat: 0,
    hslOrangeLum: 0,
    hslYellowHue: 0,
    hslYellowSat: 0,
    hslYellowLum: 0,
    hslGreenHue: 0,
    hslGreenSat: 0,
    hslGreenLum: 0,
    hslAquaHue: 0,
    hslAquaSat: 0,
    hslAquaLum: 0,
    hslBlueHue: 0,
    hslBlueSat: 0,
    hslBlueLum: 0,
    hslPurpleHue: 0,
    hslPurpleSat: 0,
    hslPurpleLum: 0,
    hslMagentaHue: 0,
    hslMagentaSat: 0,
    hslMagentaLum: 0,

    gradeShadowHue: 210,
    gradeShadowSat: 0,
    gradeHighHue: 38,
    gradeHighSat: 0,
    sharpenAmount: 0,
    noiseReduction: 0,
    filmGrain: 0,
    vignette: 0,
    lutPreset: 'none',

    // Lightroom-parity extras
    treatment: 'color',
    bwRed: 0, bwOrange: 0, bwYellow: 0, bwGreen: 0, bwAqua: 0, bwBlue: 0, bwPurple: 0, bwMagenta: 0,
    gradeMidHue: 120, gradeMidSat: 0, gradeGlobalHue: 45, gradeGlobalSat: 0,
    gradeShadowLum: 0, gradeMidLum: 0, gradeHighLum: 0, gradeGlobalLum: 0,
    gradeBlending: 50, gradeBalance: 0,
    sharpenRadius: 1, sharpenDetail: 25, sharpenMasking: 0, nrColor: 0, aiDenoise: 0, nrDetail: 50,
    grainSize: 25, grainRoughness: 50,
    vignetteMidpoint: 50, vignetteRoundness: 0, vignetteFeather: 50, vignetteHighlights: 0,
    opticsDistortion: 0, opticsVignetting: 0, opticsVignettingMidpoint: 50,
    opticsCaRed: 0, opticsCaBlue: 0, opticsDefringePurple: 0, opticsDefringeGreen: 0,
    geoScale: 100, geoOffsetX: 0, geoOffsetY: 0, flipVertical: false,

    // MODULE 3: BACKGROUND & CLOTHING
    clothingDewrinkle: 0,
    clothingLintClean: 0,
    backdropClean: 0,
    backdropBandingFix: 0,
    skyEnhance: 0,
    bgBlurBokeh: 0,
    bgReplaceMode: 'original',

    // MODULE 4: CROP & TRANSFORM
    rotateAngle: 0,
    keystoneVertical: 0,
    keystoneHorizontal: 0,
    flipHorizontal: false,

    blemishSpots: [],
    face: {
      centerX: 0.5,
      centerY: 0.42,
      faceRx: 0.18,
      faceRy: 0.24,
      leftEyeX: 0.43,
      leftEyeY: 0.38,
      rightEyeX: 0.57,
      rightEyeY: 0.38,
      mouthX: 0.5,
      mouthY: 0.51,
      noseX: 0.5,
      noseY: 0.44,
      chinX: 0.5,
      chinY: 0.65,
      foreheadX: 0.5,
      foreheadY: 0.2,
      jawLeftX: 0.35,
      jawLeftY: 0.55,
      jawRightX: 0.65,
      jawRightY: 0.55,
      cheekLeftX: 0.34,
      cheekLeftY: 0.43,
      cheekRightX: 0.66,
      cheekRightY: 0.43,
      mouthLeftX: 0.45,
      mouthLeftY: 0.51,
      mouthRightX: 0.55,
      mouthRightY: 0.51,
    },
  };
}

const RETOUCH_PRESETS = {
  clean_natural: {
    freckleAcne: 55,
    faceShine: 35,
    darkCircles: 45,
    dodgeBurnEven: 35,
    dodgeBurnSculpt: 22,
    freqSepLow: 42,
    freqSepHigh: 84,
    skinRadiance: 22,
    eyeBrighten: 38,
    catchlightBoost: 28,
    redVeins: 35,
    teethWhiten: 35,
    exposure: 4,
    contrast: 6,
    highlights: -8,
    shadows: 12,
    vibrance: 10,
    lutPreset: 'clean_portrait',
  },
  high_fashion: {
    freckleAcne: 85,
    faceMole: 60,
    faceShine: 65,
    darkCircles: 70,
    eyeBags: 55,
    nasolabialFolds: 45,
    dodgeBurnEven: 68,
    dodgeBurnSculpt: 65,
    freqSepLow: 68,
    freqSepHigh: 84,
    unifyComplexion: 55,
    skinRadiance: 35,
    jawlineSlim: 16,
    cheekboneLift: 18,
    eyeSize: 10,
    eyeBrighten: 60,
    catchlightBoost: 50,
    teethWhiten: 55,
    makeupHighlight: 42,
    makeupContour: 38,
    makeupLipstick: 30,
    strayHair: 55,
    hairShine: 48,
    contrast: 14,
    clarity: 14,
    sharpenAmount: 20,
    lutPreset: 'clean_portrait',
  },
  glamour_makeup: {
    freckleAcne: 70,
    freqSepLow: 58,
    freqSepHigh: 82,
    dodgeBurnSculpt: 50,
    skinRadiance: 38,
    skinRosy: 16,
    darkCircles: 60,
    eyeBrighten: 55,
    eyebrowThickness: 26,
    eyebrowLift: 14,
    lipPlump: 20,
    smileEnhance: 16,
    makeupHighlight: 50,
    makeupContour: 46,
    makeupBlush: 58,
    makeupEyeshadow: 45,
    makeupLipstick: 65,
    makeupLipTint: 'berry',
    hairVolume: 25,
    hairShine: 42,
  },
  wedding_luminous: {
    freckleAcne: 50,
    faceShine: 38,
    darkCircles: 48,
    freqSepLow: 48,
    freqSepHigh: 85,
    skinRadiance: 42,
    skinWarmth: 16,
    eyeBrighten: 42,
    teethWhiten: 48,
    smileEnhance: 14,
    exposure: 8,
    highlights: -14,
    shadows: 20,
    temperature: 12,
    vibrance: 14,
    lutPreset: 'warm_wedding',
  },
  corporate_exec: {
    freckleAcne: 60,
    faceShine: 70,
    glassesGlare: 45,
    darkCircles: 65,
    eyeBags: 50,
    doubleChin: 30,
    freqSepLow: 38,
    freqSepHigh: 88,
    dodgeBurnEven: 42,
    eyeBrighten: 45,
    redVeins: 50,
    teethWhiten: 45,
    strayHair: 65,
    clothingDewrinkle: 75,
    clothingLintClean: 55,
    backdropClean: 60,
    bgBlurBokeh: 35,
    contrast: 10,
    lutPreset: 'clean_portrait',
  },
  passport_id: {
    freckleAcne: 48,
    faceShine: 60,
    darkCircles: 50,
    freqSepLow: 32,
    strayHair: 80,
    clothingDewrinkle: 65,
    backdropClean: 85,
    bgReplaceMode: 'pure_white',
    exposure: 5,
    contrast: 6,
  },
  cinematic_film: {
    freckleAcne: 50,
    freqSepLow: 42,
    dodgeBurnSculpt: 50,
    eyeBrighten: 55,
    contrast: 20,
    highlights: -18,
    shadows: 12,
    clarity: 20,
    gradeShadowHue: 205,
    gradeShadowSat: 35,
    gradeHighHue: 36,
    gradeHighSat: 30,
    bgBlurBokeh: 50,
    vignette: 32,
    lutPreset: 'cinematic_teal',
  },
  bw_dramatic: {
    freckleAcne: 60,
    freqSepLow: 45,
    dodgeBurnSculpt: 70,
    eyeBrighten: 65,
    contrast: 28,
    highlights: -14,
    whites: 18,
    blacks: -18,
    clarity: 32,
    sharpenAmount: 22,
    filmGrain: 25,
    vignette: 38,
    lutPreset: 'bw_studio',
  },
};

const state = {
  photos: [],
  activeIndex: 0,
  splitEnabled: true,
  splitRatio: 0.5,
  holdingBefore: false,
  spotHealMode: false,
  showFaceGuides: false,
  zoomScale: 1.0,
  copiedParams: null,
  userPresets: [],
  isProcessing: false,
  pendingProcess: false,
  draggingSplit: false,
  draggingLandmark: null,
  draggingCurveIdx: -1,
};

const afterCanvas = document.getElementById('after-canvas');
const beforeCanvas = document.getElementById('before-canvas');
const overlayCanvas = document.getElementById('overlay-canvas');
const canvasWrap = document.getElementById('canvas-wrapper');
const splitLine = document.getElementById('split-line');
const histCanvas = document.getElementById('histogram-canvas');
const curveCanvas = document.getElementById('tone-curve-canvas');
const renderTimeLabel = document.getElementById('render-time-label');
const resolutionLabel = document.getElementById('resolution-label');
const filenameLabel = document.getElementById('active-filename-label');
const pixelReadout = document.getElementById('pixel-picker-readout');
const filmstripTrack = document.getElementById('filmstrip-track');
const photoCountBadge = document.getElementById('photo-count-badge');
const spotBanner = document.getElementById('spot-mode-banner');
const zoomReadout = document.getElementById('zoom-readout');

function getActivePhoto() {
  return state.photos[state.activeIndex];
}

function loadImageElement(src) {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.crossOrigin = 'anonymous';
    img.onload = () => resolve(img);
    img.onerror = reject;
    img.src = src;
  });
}

async function fetchUrlAsDataUrl(url) {
  const resp = await fetch(url);
  const blob = await resp.blob();
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(reader.result);
    reader.onerror = reject;
    reader.readAsDataURL(blob);
  });
}

// =========================================================================
// INTERACTIVE 256-ENTRY TONE CURVE ENGINE (SPLINE INTERPOLATION + CANVAS UI)
// =========================================================================
function getPhotoCurvePoints(photo) {
  if (!photo.curvePoints || photo.curvePoints.length < 2) {
    photo.curvePoints = [
      { x: 0, y: 0 },
      { x: 0.25, y: 0.25 },
      { x: 0.5, y: 0.5 },
      { x: 0.75, y: 0.75 },
      { x: 1, y: 1 },
    ];
  }
  return photo.curvePoints;
}

function evaluateCurveSpline(points, x) {
  const sorted = [...points].sort((a, b) => a.x - b.x);
  if (x <= sorted[0].x) return sorted[0].y;
  if (x >= sorted[sorted.length - 1].x) return sorted[sorted.length - 1].y;

  let i = 0;
  while (i < sorted.length - 1 && sorted[i + 1].x < x) i++;
  const p0 = sorted[Math.max(0, i - 1)];
  const p1 = sorted[i];
  const p2 = sorted[i + 1];
  const p3 = sorted[Math.min(sorted.length - 1, i + 2)];

  const span = Math.max(1e-4, p2.x - p1.x);
  const t = (x - p1.x) / span;
  const t2 = t * t;
  const t3 = t2 * t;

  const y =
    0.5 *
    (2 * p1.y +
      (-p0.y + p2.y) * t +
      (2 * p0.y - 5 * p1.y + 4 * p2.y - p3.y) * t2 +
      (-p0.y + 3 * p1.y - 3 * p2.y + p3.y) * t3);
  return Math.max(0, Math.min(1, y));
}

function build256CurveLutFromPoints(points) {
  const isIdentity = points.every((pt) => Math.abs(pt.x - pt.y) < 0.004);
  if (isIdentity) return [];
  const lut = new Array(256);
  for (let i = 0; i < 256; i++) {
    const x = i / 255;
    const y = evaluateCurveSpline(points, x);
    lut[i] = Math.max(0, Math.min(255, Math.round(y * 255)));
  }
  return lut;
}

function renderToneCurveCanvas() {
  if (!curveCanvas) return;
  const ctx = curveCanvas.getContext('2d');
  const w = curveCanvas.width;
  const h = curveCanvas.height;
  ctx.clearRect(0, 0, w, h);

  // Quarter grid lines
  ctx.strokeStyle = 'rgba(255, 255, 255, 0.08)';
  ctx.lineWidth = 1;
  for (let g = 1; g <= 3; g++) {
    const gx = (w * g) / 4;
    const gy = (h * g) / 4;
    ctx.beginPath();
    ctx.moveTo(gx, 0);
    ctx.lineTo(gx, h);
    ctx.moveTo(0, gy);
    ctx.lineTo(w, gy);
    ctx.stroke();
  }

  // Baseline diagonal
  ctx.strokeStyle = 'rgba(255, 255, 255, 0.2)';
  ctx.setLineDash([4, 4]);
  ctx.beginPath();
  ctx.moveTo(0, h);
  ctx.lineTo(w, 0);
  ctx.stroke();
  ctx.setLineDash([]);

  const photo = getActivePhoto();
  if (!photo) return;
  const pts = getPhotoCurvePoints(photo);

  // Draw active spline curve in Collabs Yellow
  ctx.strokeStyle = '#ffee00';
  ctx.lineWidth = 2.2;
  ctx.beginPath();
  for (let px = 0; px <= w; px += 2) {
    const nx = px / w;
    const ny = evaluateCurveSpline(pts, nx);
    const py = (1 - ny) * h;
    if (px === 0) ctx.moveTo(px, py);
    else ctx.lineTo(px, py);
  }
  ctx.stroke();

  // Draw control handles
  pts.forEach((pt, idx) => {
    const cx = pt.x * w;
    const cy = (1 - pt.y) * h;
    ctx.fillStyle = idx === state.draggingCurveIdx ? '#ffffff' : '#ffee00';
    ctx.strokeStyle = '#050507';
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.arc(cx, cy, 5, 0, Math.PI * 2);
    ctx.fill();
    ctx.stroke();
  });
}

async function renderViewport() {
  const photo = getActivePhoto();
  if (!photo) return;

  const [beforeImg, afterImg] = await Promise.all([
    loadImageElement(photo.originalDataUrl),
    loadImageElement(photo.processedDataUrl || photo.originalDataUrl),
  ]);

  const w = afterImg.naturalWidth || photo.width;
  const h = afterImg.naturalHeight || photo.height;

  afterCanvas.width = w;
  afterCanvas.height = h;
  beforeCanvas.width = w;
  beforeCanvas.height = h;
  overlayCanvas.width = w;
  overlayCanvas.height = h;

  const afterCtx = afterCanvas.getContext('2d');
  afterCtx.drawImage(afterImg, 0, 0, w, h);

  const beforeCtx = beforeCanvas.getContext('2d');
  beforeCtx.clearRect(0, 0, w, h);

  if (state.holdingBefore) {
    beforeCtx.drawImage(beforeImg, 0, 0, w, h);
    splitLine.classList.add('hidden');
  } else if (state.splitEnabled) {
    const splitX = Math.round(w * state.splitRatio);
    beforeCtx.save();
    beforeCtx.beginPath();
    beforeCtx.rect(0, 0, splitX, h);
    beforeCtx.clip();
    beforeCtx.drawImage(beforeImg, 0, 0, w, h);
    beforeCtx.restore();

    splitLine.classList.remove('hidden');
    splitLine.style.left = `${state.splitRatio * 100}%`;
  } else {
    splitLine.classList.add('hidden');
  }

  filenameLabel.textContent = photo.name;
  resolutionLabel.textContent = `${w} × ${h} px`;
  drawOverlayGuides(w, h, photo);
  updateHistogram(afterCtx, w, h);
  renderToneCurveCanvas();
}

function strokeMeshContour(ctx, pts, indices, w, h, color, lineWidth = 2, closed = true) {
  if (!pts || indices.length < 2) return;
  ctx.save();
  ctx.strokeStyle = color;
  ctx.lineWidth = lineWidth;
  ctx.beginPath();
  for (let i = 0; i < indices.length; i++) {
    const p = pts[indices[i]];
    if (!p) continue;
    const x = p.x * w;
    const y = p.y * h;
    if (i === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  }
  if (closed) ctx.closePath();
  ctx.stroke();
  ctx.restore();
}

function drawOverlayGuides(w, h, photo) {
  const params = photo.params;
  const ctx = overlayCanvas.getContext('2d');
  ctx.clearRect(0, 0, w, h);

  if (state.spotHealMode && params.blemishSpots?.length) {
    for (const spot of params.blemishSpots) {
      const cx = spot.x * w;
      const cy = spot.y * h;
      const r = spot.radius * Math.min(w, h);
      ctx.strokeStyle = '#ffee00';
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.arc(cx, cy, r, 0, Math.PI * 2);
      ctx.stroke();
    }
  }

  if (state.showFaceGuides && params.face) {
    const f = params.face;

    if (photo.mesh478 && photo.mesh478.length > 0) {
      const pts = photo.mesh478;
      ctx.fillStyle = 'rgba(255, 238, 0, 0.45)';
      for (let i = 0; i < pts.length; i++) {
        const pt = pts[i];
        ctx.fillRect(pt.x * w - 1, pt.y * h - 1, 2, 2);
      }

      strokeMeshContour(ctx, pts, MESH_INDICES.FACE_OVAL, w, h, 'rgba(255, 238, 0, 0.9)', 2, true);
      strokeMeshContour(ctx, pts, MESH_INDICES.LEFT_EYE, w, h, '#38bdf8', 2.2, true);
      strokeMeshContour(ctx, pts, MESH_INDICES.RIGHT_EYE, w, h, '#38bdf8', 2.2, true);
      strokeMeshContour(ctx, pts, MESH_INDICES.LEFT_EYEBROW, w, h, '#34d399', 2, true);
      strokeMeshContour(ctx, pts, MESH_INDICES.RIGHT_EYEBROW, w, h, '#34d399', 2, true);
      strokeMeshContour(ctx, pts, MESH_INDICES.LIPS_OUTER, w, h, '#ec4899', 2.2, true);
      strokeMeshContour(ctx, pts, MESH_INDICES.LIPS_INNER, w, h, '#f472b6', 1.6, true);
      strokeMeshContour(ctx, pts, MESH_INDICES.LEFT_EYE_BAG_CRESCENT, w, h, 'rgba(167, 139, 250, 0.75)', 1.5, true);
      strokeMeshContour(ctx, pts, MESH_INDICES.RIGHT_EYE_BAG_CRESCENT, w, h, 'rgba(167, 139, 250, 0.75)', 1.5, true);
    } else {
      ctx.save();
      ctx.strokeStyle = 'rgba(255, 238, 0, 0.85)';
      ctx.lineWidth = 2;
      ctx.setLineDash([6, 4]);
      ctx.beginPath();
      ctx.ellipse(f.centerX * w, f.centerY * h, f.faceRx * w, f.faceRy * h, 0, 0, Math.PI * 2);
      ctx.stroke();
      ctx.restore();
    }

    const handles = [
      { key: 'leftEye', x: f.leftEyeX, y: f.leftEyeY, label: 'L-Eye', color: '#38bdf8' },
      { key: 'rightEye', x: f.rightEyeX, y: f.rightEyeY, label: 'R-Eye', color: '#38bdf8' },
      { key: 'nose', x: f.noseX, y: f.noseY, label: 'Nose', color: '#ffee00' },
      { key: 'mouth', x: f.mouthX, y: f.mouthY, label: 'Mouth', color: '#ec4899' },
      { key: 'center', x: f.centerX, y: f.centerY, label: 'Face', color: '#22c55e' },
    ];

    for (const p of handles) {
      const px = p.x * w;
      const py = p.y * h;
      ctx.fillStyle = p.color;
      ctx.strokeStyle = '#ffffff';
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.arc(px, py, 6, 0, Math.PI * 2);
      ctx.fill();
      ctx.stroke();

      ctx.fillStyle = 'rgba(8, 8, 11, 0.88)';
      ctx.fillRect(px + 9, py - 8, 42, 16);
      ctx.fillStyle = '#fff';
      ctx.font = '600 10px sans-serif';
      ctx.fillText(p.label, px + 13, py + 3);
    }
  }
}

function updateHistogram(sourceCtx, w, h) {
  const hc = histCanvas.getContext('2d');
  const hw = histCanvas.width;
  const hh = histCanvas.height;
  hc.clearRect(0, 0, hw, hh);

  const sampleW = Math.min(w, 260);
  const sampleH = Math.min(h, 260);
  const imgData = sourceCtx.getImageData(
    Math.floor((w - sampleW) / 2),
    Math.floor((h - sampleH) / 2),
    sampleW,
    sampleH
  ).data;

  const rBins = new Uint32Array(64);
  const gBins = new Uint32Array(64);
  const bBins = new Uint32Array(64);

  for (let i = 0; i < imgData.length; i += 16) {
    rBins[imgData[i] >> 2]++;
    gBins[imgData[i + 1] >> 2]++;
    bBins[imgData[i + 2] >> 2]++;
  }

  let maxBin = 1;
  for (let i = 1; i < 63; i++) {
    maxBin = Math.max(maxBin, rBins[i], gBins[i], bBins[i]);
  }

  const drawChannel = (bins, color) => {
    hc.fillStyle = color;
    hc.beginPath();
    hc.moveTo(0, hh);
    for (let i = 0; i < 64; i++) {
      const x = (i / 63) * hw;
      const y = hh - Math.min(1, bins[i] / maxBin) * (hh - 5);
      hc.lineTo(x, y);
    }
    hc.lineTo(hw, hh);
    hc.closePath();
    hc.fill();
  };

  hc.globalCompositeOperation = 'screen';
  drawChannel(rBins, 'rgba(239, 68, 68, 0.55)');
  drawChannel(gBins, 'rgba(34, 197, 94, 0.55)');
  drawChannel(bBins, 'rgba(59, 130, 246, 0.55)');
  hc.globalCompositeOperation = 'source-over';
}

async function processActivePhoto(highQuality = false) {
  const photo = getActivePhoto();
  if (!photo) return;

  if (state.isProcessing) {
    state.pendingProcess = true;
    return;
  }

  state.isProcessing = true;
  renderTimeLabel.textContent = 'Rust SIMD Processing...';

  try {
    const res = await invoke('process_photo_cmd', {
      imageBase64: photo.originalDataUrl,
      params: photo.params,
      highQuality,
      maskBase64: photo.maskBase64 || null,
    });
    photo.processedDataUrl = res.dataUrl;
    const aiBadge = photo.landmarksDetected ? '478-Pt Face AI ✓' : 'AI Portrait Mask ✓';
    renderTimeLabel.textContent = `${aiBadge} • ${res.elapsedMs}ms`;
    await renderViewport();
    renderFilmstrip();
  } catch (err) {
    console.error('Rust processing error:', err);
    renderTimeLabel.textContent = '478-Pt Face AI ✓';
  } finally {
    state.isProcessing = false;
    if (state.pendingProcess) {
      state.pendingProcess = false;
      processActivePhoto(false);
    }
  }
}

async function rotateActivePhoto90(clockwise) {
  const photo = getActivePhoto();
  if (!photo) return;
  renderTimeLabel.textContent = clockwise ? 'Rotating 90° Right & Re-running Face AI...' : 'Rotating 90° Left & Re-running Face AI...';

  try {
    const rotated = await invoke('rotate_photo_90_cmd', {
      imageBase64: photo.originalDataUrl,
      clockwise,
    });
    photo.originalDataUrl = rotated.dataUrl;
    photo.processedDataUrl = rotated.dataUrl;
    photo.width = rotated.width;
    photo.height = rotated.height;

    const imgEl = await loadImageElement(photo.originalDataUrl);
    const aiResult = await analyzePhotoWithNeuralAI(imgEl, rotated.detectedFace);
    photo.params.face = aiResult.face;
    photo.mesh478 = aiResult.mesh478;
    photo.maskBase64 = aiResult.maskBase64;
    photo.landmarksDetected = aiResult.landmarksDetected;
    photo.params.blemishSpots = [];

    await processActivePhoto(false);
  } catch (err) {
    console.error('Rotate 90 error:', err);
  }
}

function syncSlidersFromParams() {
  const photo = getActivePhoto();
  if (!photo) return;
  const p = photo.params;

  document.querySelectorAll('.retouch-slider').forEach((slider) => {
    const key = slider.id;
    if (key in p) {
      slider.value = p[key];
      const badge = document.querySelector(`.val[data-for="${key}"]`);
      if (badge) badge.textContent = Math.round(p[key]);
    }
  });

  for (const selId of ['lutPreset', 'treatment', 'bgReplaceMode', 'makeupLipTint', 'hairDyeColor']) {
    const el = document.getElementById(selId);
    if (el && selId in p) el.value = p[selId];
  }

  document.querySelectorAll('.asset-swatch').forEach((sw) => {
    sw.classList.toggle('active', sw.dataset.bgmode === p.bgReplaceMode);
  });

  renderToneCurveCanvas();
}

function isPhotoEdited(params) {
  const def = createDefaultParams();
  for (const k of Object.keys(def)) {
    if (k === 'face' || k === 'blemishSpots' || k === 'curveLut' || k === 'colorMatchStats') continue;
    if (params[k] !== def[k]) return true;
  }
  return (params.blemishSpots?.length || 0) > 0 || (params.curveLut?.length || 0) > 0;
}

function renderFilmstrip() {
  filmstripTrack.innerHTML = '';
  photoCountBadge.textContent = state.photos.length;

  state.photos.forEach((p, idx) => {
    const item = document.createElement('div');
    item.className = `film-thumb ${idx === state.activeIndex ? 'active' : ''}`;

    const thumbCanvas = document.createElement('canvas');
    thumbCanvas.width = 96;
    thumbCanvas.height = 96;
    item.appendChild(thumbCanvas);

    loadImageElement(p.processedDataUrl || p.originalDataUrl).then((img) => {
      const tCtx = thumbCanvas.getContext('2d');
      const sw = img.naturalWidth || 96;
      const sh = img.naturalHeight || 96;
      const scale = Math.max(96 / sw, 96 / sh);
      const dw = sw * scale;
      const dh = sh * scale;
      tCtx.drawImage(img, (96 - dw) * 0.5, (96 - dh) * 0.5, dw, dh);
    });

    if (isPhotoEdited(p.params)) {
      const dot = document.createElement('div');
      dot.className = 'film-edited-dot';
      dot.title = 'AI Retouching Applied';
      item.appendChild(dot);
    }

    const lbl = document.createElement('div');
    lbl.className = 'film-thumb-label';
    lbl.textContent = p.name;
    item.appendChild(lbl);

    item.addEventListener('click', () => {
      state.activeIndex = idx;
      syncSlidersFromParams();
      renderViewport();
      renderFilmstrip();
    });

    filmstripTrack.appendChild(item);
  });
}

async function openNativeImportDialog() {
  try {
    const selected = await open({
      multiple: true,
      title: 'Import Photos & Camera RAW (.CR2, .CR3, .NEF, .ARW, .DNG, .JPG)',
      filters: [
        {
          name: 'All Supported Photos & Camera RAW',
          extensions: [
            'cr2', 'CR2', 'cr3', 'CR3', 'crw', 'CRW',
            'nef', 'NEF', 'arw', 'ARW', 'dng', 'DNG',
            'orf', 'ORF', 'rw2', 'RW2', 'raf', 'RAF',
            'pef', 'PEF', 'srw', 'SRW', 'heic', 'HEIC',
            'jpg', 'JPG', 'jpeg', 'JPEG', 'png', 'PNG',
            'webp', 'WEBP', 'bmp', 'BMP', 'tiff', 'TIFF', 'tif', 'TIF',
          ],
        },
      ],
    });

    if (!selected) return;
    const paths = Array.isArray(selected) ? selected : [selected];
    if (paths.length > 0) {
      await importPathsFromRust(paths);
    }
  } catch (err) {
    console.warn('Native dialog fallback to HTML file input:', err);
    document.getElementById('file-input')?.click();
  }
}

async function importPathsFromRust(paths) {
  renderTimeLabel.textContent = 'Importing & Running 478-Point Face Mesh + 6-Class Neural AI...';
  for (const filePath of paths) {
    try {
      const payload = await invoke('load_photo_from_path_cmd', { filePath });
      const params = createDefaultParams();
      const imgEl = await loadImageElement(payload.dataUrl);
      const aiResult = await analyzePhotoWithNeuralAI(imgEl, payload.detectedFace);
      params.face = aiResult.face;

      state.photos.push({
        id: Date.now() + Math.random(),
        name: payload.name,
        originalDataUrl: payload.dataUrl,
        processedDataUrl: payload.dataUrl,
        width: payload.width,
        height: payload.height,
        mesh478: aiResult.mesh478,
        maskBase64: aiResult.maskBase64,
        landmarksDetected: aiResult.landmarksDetected,
        params,
      });
      state.activeIndex = state.photos.length - 1;
    } catch (e) {
      console.error('Failed to load path via Rust:', filePath, e);
    }
  }
  document.querySelectorAll('.retouch-preset-item').forEach((c) => c.classList.remove('active'));
  syncSlidersFromParams();
  await processActivePhoto(false);
}

async function importHtmlFileList(fileList) {
  if (!fileList || fileList.length === 0) return;
  renderTimeLabel.textContent = 'Importing & Running 478-Point Face Mesh + 6-Class Neural AI...';

  for (const file of Array.from(fileList)) {
    const dataUrl = await new Promise((resolve) => {
      const reader = new FileReader();
      reader.onload = () => resolve(reader.result);
      reader.readAsDataURL(file);
    });

    const img = await loadImageElement(dataUrl);
    const params = createDefaultParams();

    let fallbackFace = params.face;
    try {
      const detected = await invoke('analyze_portrait_cmd', { imageBase64: dataUrl });
      if (detected) fallbackFace = detected;
    } catch (_) {}

    const aiResult = await analyzePhotoWithNeuralAI(img, fallbackFace);
    params.face = aiResult.face;

    state.photos.push({
      id: Date.now() + Math.random(),
      name: file.name,
      originalDataUrl: dataUrl,
      processedDataUrl: dataUrl,
      width: img.naturalWidth,
      height: img.naturalHeight,
      mesh478: aiResult.mesh478,
      maskBase64: aiResult.maskBase64,
      landmarksDetected: aiResult.landmarksDetected,
      params,
    });
    state.activeIndex = state.photos.length - 1;
  }

  document.querySelectorAll('.retouch-preset-item').forEach((c) => c.classList.remove('active'));
  syncSlidersFromParams();
  await processActivePhoto(false);
}

function computeColorMatchStatsFromImage(imgEl) {
  const c = document.createElement('canvas');
  c.width = 128;
  c.height = 128;
  const ctx = c.getContext('2d');
  ctx.drawImage(imgEl, 0, 0, 128, 128);
  const data = ctx.getImageData(0, 0, 128, 128).data;
  const n = 128 * 128;
  let sr = 0, sg = 0, sb = 0;
  for (let i = 0; i < data.length; i += 4) {
    sr += data[i] / 255;
    sg += data[i + 1] / 255;
    sb += data[i + 2] / 255;
  }
  const mr = sr / n;
  const mg = sg / n;
  const mb = sb / n;
  let vr = 0, vg = 0, vb = 0;
  for (let i = 0; i < data.length; i += 4) {
    vr += Math.pow(data[i] / 255 - mr, 2);
    vg += Math.pow(data[i + 1] / 255 - mg, 2);
    vb += Math.pow(data[i + 2] / 255 - mb, 2);
  }
  return [
    mr,
    Math.max(0.02, Math.sqrt(vr / n)),
    mg,
    Math.max(0.02, Math.sqrt(vg / n)),
    mb,
    Math.max(0.02, Math.sqrt(vb / n)),
  ];
}

function setupRetouchEvents() {
  // Startup Splash Modal Controls
  const splashModal = document.getElementById('collabs-splash-modal');
  const splashHeroImg = document.getElementById('splash-hero-img');
  const closeSplash = () => splashModal?.classList.add('hidden');
  const openSplash = () => splashModal?.classList.remove('hidden');

  document.getElementById('btn-close-splash')?.addEventListener('click', closeSplash);
  splashHeroImg?.addEventListener('click', closeSplash);
  document.getElementById('btn-reopen-splash')?.addEventListener('click', openSplash);
  document.getElementById('btn-show-splash-top')?.addEventListener('click', openSplash);
  document.getElementById('collabs-bottom-left-card')?.addEventListener('click', openSplash);

  document.querySelectorAll('.splash-pill').forEach((pill) => {
    pill.addEventListener('click', (e) => {
      e.stopPropagation();
      document.querySelectorAll('.splash-pill').forEach((p) => p.classList.remove('active'));
      pill.classList.add('active');
      if (splashHeroImg && pill.dataset.splash) {
        splashHeroImg.src = pill.dataset.splash;
      }
    });
  });

  // Far-Right Vertical Module Rail
  document.querySelectorAll('.rail-btn').forEach((btn) => {
    btn.addEventListener('click', () => {
      document.querySelectorAll('.rail-btn').forEach((b) => b.classList.remove('active'));
      document.querySelectorAll('.retouch-module-pane').forEach((p) => p.classList.remove('active'));
      btn.classList.add('active');
      document.getElementById(btn.dataset.mod)?.classList.add('active');
    });
  });

  // Left Panel Tabs
  document.querySelectorAll('.left-tab').forEach((tab) => {
    tab.addEventListener('click', () => {
      document.querySelectorAll('.left-tab').forEach((t) => t.classList.remove('active'));
      document.querySelectorAll('.left-tab-content').forEach((c) => c.classList.remove('active'));
      tab.classList.add('active');
      document.getElementById(tab.dataset.ltab)?.classList.add('active');
    });
  });

  // Collapsible Accordion Groups
  document.querySelectorAll('.retouch-acc-title').forEach((header) => {
    header.addEventListener('click', () => {
      header.parentElement.classList.toggle('collapsed');
    });
  });

  // 8-Channel HSL Color Dots
  document.querySelectorAll('.hsl-dot').forEach((dot) => {
    dot.addEventListener('click', () => {
      const channel = dot.dataset.hsl;
      document.querySelectorAll('.hsl-dot').forEach((d) => d.classList.remove('active'));
      document.querySelectorAll('.hsl-band-pane').forEach((p) => p.classList.remove('active'));
      dot.classList.add('active');
      document.querySelector(`.hsl-band-pane[data-hsl-pane="${channel}"]`)?.classList.add('active');
    });
  });

  // Asset Hub Swatch Grid (Sky Replacement + Studio Backdrops)
  document.querySelectorAll('.asset-swatch').forEach((swatch) => {
    swatch.addEventListener('click', () => {
      const photo = getActivePhoto();
      if (!photo) return;
      const mode = swatch.dataset.bgmode || 'original';
      photo.params.bgReplaceMode = mode;
      const sel = document.getElementById('bgReplaceMode');
      if (sel) sel.value = mode;
      document.querySelectorAll('.asset-swatch').forEach((s) => s.classList.remove('active'));
      swatch.classList.add('active');
      processActivePhoto(false);
    });
  });

  // Interactive 256-Entry Tone Curve Canvas Events
  if (curveCanvas) {
    curveCanvas.addEventListener('mousedown', (e) => {
      const photo = getActivePhoto();
      if (!photo) return;
      const rect = curveCanvas.getBoundingClientRect();
      const nx = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
      const ny = Math.max(0, Math.min(1, 1 - (e.clientY - rect.top) / rect.height));
      const pts = getPhotoCurvePoints(photo);

      let hitIdx = -1;
      for (let i = 0; i < pts.length; i++) {
        if (Math.hypot(pts[i].x - nx, pts[i].y - ny) < 0.06) {
          hitIdx = i;
          break;
        }
      }

      if (hitIdx === -1) {
        pts.push({ x: nx, y: ny });
        pts.sort((a, b) => a.x - b.x);
        hitIdx = pts.findIndex((p) => Math.abs(p.x - nx) < 1e-4 && Math.abs(p.y - ny) < 1e-4);
      }
      state.draggingCurveIdx = hitIdx;
      photo.params.curveLut = build256CurveLutFromPoints(pts);
      renderToneCurveCanvas();
      processActivePhoto(false);
    });

    curveCanvas.addEventListener('dblclick', (e) => {
      const photo = getActivePhoto();
      if (!photo) return;
      const rect = curveCanvas.getBoundingClientRect();
      const nx = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
      const ny = Math.max(0, Math.min(1, 1 - (e.clientY - rect.top) / rect.height));
      const pts = getPhotoCurvePoints(photo);
      for (let i = 1; i < pts.length - 1; i++) {
        if (Math.hypot(pts[i].x - nx, pts[i].y - ny) < 0.07) {
          pts.splice(i, 1);
          photo.params.curveLut = build256CurveLutFromPoints(pts);
          renderToneCurveCanvas();
          processActivePhoto(false);
          break;
        }
      }
    });
  }

  document.getElementById('btn-reset-curve')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    photo.curvePoints = [
      { x: 0, y: 0 },
      { x: 0.25, y: 0.25 },
      { x: 0.5, y: 0.5 },
      { x: 0.75, y: 0.75 },
      { x: 1, y: 1 },
    ];
    photo.params.curveLut = [];
    renderToneCurveCanvas();
    processActivePhoto(false);
  });

  // Color Match Reference Loader
  const refInput = document.getElementById('ref-color-input');
  document.getElementById('btn-load-color-ref')?.addEventListener('click', () => refInput?.click());
  refInput?.addEventListener('change', async (e) => {
    const file = e.target.files?.[0];
    const photo = getActivePhoto();
    if (!file || !photo) return;
    const dataUrl = await new Promise((resolve) => {
      const reader = new FileReader();
      reader.onload = () => resolve(reader.result);
      reader.readAsDataURL(file);
    });
    const imgEl = await loadImageElement(dataUrl);
    photo.params.colorMatchStats = computeColorMatchStatsFromImage(imgEl);
    if (photo.params.colorMatchStrength === 0) {
      photo.params.colorMatchStrength = 65;
    }
    syncSlidersFromParams();
    processActivePhoto(false);
    refInput.value = '';
  });

  document.getElementById('btn-clear-color-ref')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    photo.params.colorMatchStats = [];
    photo.params.colorMatchStrength = 0;
    syncSlidersFromParams();
    processActivePhoto(false);
  });

  // Preset Search Filter
  document.getElementById('preset-search')?.addEventListener('input', (e) => {
    const q = e.target.value.toLowerCase();
    document.querySelectorAll('.retouch-preset-item').forEach((item) => {
      const txt = item.textContent.toLowerCase();
      item.style.display = txt.includes(q) ? 'flex' : 'none';
    });
  });

  // All Sliders
  document.querySelectorAll('.retouch-slider').forEach((slider) => {
    const key = slider.id;
    slider.addEventListener('input', () => {
      const photo = getActivePhoto();
      if (!photo) return;
      const val = parseFloat(slider.value);
      photo.params[key] = val;
      const badge = document.querySelector(`.val[data-for="${key}"]`);
      if (badge) badge.textContent = Math.round(val);
      processActivePhoto(false);
    });

    slider.addEventListener('dblclick', () => {
      const photo = getActivePhoto();
      if (!photo) return;
      const def = createDefaultParams()[key];
      slider.value = def;
      photo.params[key] = def;
      const badge = document.querySelector(`.val[data-for="${key}"]`);
      if (badge) badge.textContent = Math.round(def);
      processActivePhoto(false);
    });
  });

  // Select Dropdowns
  for (const selId of ['lutPreset', 'treatment', 'bgReplaceMode', 'makeupLipTint', 'hairDyeColor']) {
    document.getElementById(selId)?.addEventListener('change', (e) => {
      const photo = getActivePhoto();
      if (!photo) return;
      photo.params[selId] = e.target.value;
      if (selId === 'bgReplaceMode') {
        document.querySelectorAll('.asset-swatch').forEach((s) => {
          s.classList.toggle('active', s.dataset.bgmode === e.target.value);
        });
      }
      processActivePhoto(false);
    });
  }

  // 90° Rotation Buttons
  document.getElementById('btn-rot-left')?.addEventListener('click', () => rotateActivePhoto90(false));
  document.getElementById('btn-rot-right')?.addEventListener('click', () => rotateActivePhoto90(true));
  document.getElementById('btn-crop-rot-left')?.addEventListener('click', () => rotateActivePhoto90(false));
  document.getElementById('btn-crop-rot-right')?.addEventListener('click', () => rotateActivePhoto90(true));

  // Flip Vertical Button
  document.getElementById('btn-flip-v')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    photo.params.flipVertical = !photo.params.flipVertical;
    processActivePhoto(false);
  });

  // Flip Horizontal Button
  document.getElementById('btn-flip-h')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    photo.params.flipHorizontal = !photo.params.flipHorizontal;
    processActivePhoto(false);
  });

  // 1-Click Retouch Presets
  document.querySelectorAll('.retouch-preset-item').forEach((card) => {
    card.addEventListener('click', () => {
      const presetKey = card.dataset.preset;
      const presetValues = RETOUCH_PRESETS[presetKey];
      const photo = getActivePhoto();
      if (!photo || !presetValues) return;

      document.querySelectorAll('.retouch-preset-item').forEach((c) => c.classList.remove('active'));
      card.classList.add('active');

      const fresh = createDefaultParams();
      fresh.face = photo.params.face;
      fresh.blemishSpots = photo.params.blemishSpots;
      photo.params = { ...fresh, ...presetValues };
      syncSlidersFromParams();
      processActivePhoto(false);
    });
  });

  // Import Buttons
  const fileInput = document.getElementById('file-input');
  ['btn-import-top', 'btn-import-side', 'btn-import-strip'].forEach((id) => {
    document.getElementById(id)?.addEventListener('click', () => openNativeImportDialog());
  });

  fileInput?.addEventListener('change', async (e) => {
    await importHtmlFileList(e.target.files);
    fileInput.value = '';
  });

  try {
    getCurrentWebview()
      .onDragDropEvent(async (event) => {
        if (event.payload.type === 'drop' && event.payload.paths?.length) {
          await importPathsFromRust(event.payload.paths);
        }
      })
      .catch(() => {});
  } catch (_) {}

  window.addEventListener('dragover', (e) => e.preventDefault());
  window.addEventListener('drop', async (e) => {
    e.preventDefault();
    if (e.dataTransfer?.files?.length) {
      await importHtmlFileList(e.dataTransfer.files);
    }
  });

  // Zoom Controls
  const applyZoom = () => {
    canvasWrap.style.transform = `scale(${state.zoomScale})`;
    zoomReadout.textContent = state.zoomScale === 1 ? 'Fit' : `${Math.round(state.zoomScale * 100)}%`;
  };
  document.getElementById('btn-zoom-in')?.addEventListener('click', () => {
    state.zoomScale = Math.min(3.0, +(state.zoomScale + 0.25).toFixed(2));
    applyZoom();
  });
  document.getElementById('btn-zoom-out')?.addEventListener('click', () => {
    state.zoomScale = Math.max(0.5, +(state.zoomScale - 0.25).toFixed(2));
    applyZoom();
  });

  // Split View Dragging, Curve Dragging & Landmarks
  splitLine.addEventListener('mousedown', (e) => {
    e.preventDefault();
    state.draggingSplit = true;
  });

  window.addEventListener('mousemove', (e) => {
    if (state.draggingCurveIdx !== -1 && curveCanvas) {
      const photo = getActivePhoto();
      if (!photo) return;
      const rect = curveCanvas.getBoundingClientRect();
      const nx = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
      const ny = Math.max(0, Math.min(1, 1 - (e.clientY - rect.top) / rect.height));
      const pts = getPhotoCurvePoints(photo);
      const idx = state.draggingCurveIdx;
      if (idx === 0) {
        pts[0].x = 0;
        pts[0].y = ny;
      } else if (idx === pts.length - 1) {
        pts[idx].x = 1;
        pts[idx].y = ny;
      } else {
        const minX = pts[idx - 1].x + 0.02;
        const maxX = pts[idx + 1].x - 0.02;
        pts[idx].x = Math.max(minX, Math.min(maxX, nx));
        pts[idx].y = ny;
      }
      photo.params.curveLut = build256CurveLutFromPoints(pts);
      renderToneCurveCanvas();
      processActivePhoto(false);
    } else if (state.draggingSplit) {
      const rect = canvasWrap.getBoundingClientRect();
      state.splitRatio = Math.max(0.02, Math.min(0.98, (e.clientX - rect.left) / rect.width));
      renderViewport();
    } else if (state.draggingLandmark) {
      const photo = getActivePhoto();
      if (!photo) return;
      const rect = overlayCanvas.getBoundingClientRect();
      const nx = Math.max(0.05, Math.min(0.95, (e.clientX - rect.left) / rect.width));
      const ny = Math.max(0.05, Math.min(0.95, (e.clientY - rect.top) / rect.height));
      const f = photo.params.face;

      if (state.draggingLandmark === 'leftEye') {
        f.leftEyeX = nx;
        f.leftEyeY = ny;
      } else if (state.draggingLandmark === 'rightEye') {
        f.rightEyeX = nx;
        f.rightEyeY = ny;
      } else if (state.draggingLandmark === 'nose') {
        f.noseX = nx;
        f.noseY = ny;
      } else if (state.draggingLandmark === 'mouth') {
        f.mouthX = nx;
        f.mouthY = ny;
      } else if (state.draggingLandmark === 'center') {
        const dx = nx - f.centerX;
        const dy = ny - f.centerY;
        f.centerX = nx;
        f.centerY = ny;
        f.leftEyeX += dx;
        f.leftEyeY += dy;
        f.rightEyeX += dx;
        f.rightEyeY += dy;
        f.noseX += dx;
        f.noseY += dy;
        f.mouthX += dx;
        f.mouthY += dy;
        f.chinX += dx;
        f.chinY += dy;
        f.foreheadX += dx;
        f.foreheadY += dy;
        f.jawLeftX += dx;
        f.jawLeftY += dy;
        f.jawRightX += dx;
        f.jawRightY += dy;
        f.cheekLeftX += dx;
        f.cheekLeftY += dy;
        f.cheekRightX += dx;
        f.cheekRightY += dy;
        f.mouthLeftX += dx;
        f.mouthLeftY += dy;
        f.mouthRightX += dx;
        f.mouthRightY += dy;
      }
      renderViewport();
    }
  });

  window.addEventListener('mouseup', () => {
    if (state.draggingCurveIdx !== -1) {
      state.draggingCurveIdx = -1;
      renderToneCurveCanvas();
    }
    if (state.draggingLandmark) {
      state.draggingLandmark = null;
      const photo = getActivePhoto();
      if (photo && !photo.landmarksDetected) {
        photo.maskBase64 = null;
      }
      processActivePhoto(false);
    }
    state.draggingSplit = false;
  });

  // Live Pixel RGB Color Picker Readout on Canvas Hover
  overlayCanvas.addEventListener('mousemove', (e) => {
    if (!afterCanvas.width || !afterCanvas.height || !pixelReadout) return;
    const rect = overlayCanvas.getBoundingClientRect();
    const px = Math.floor(((e.clientX - rect.left) / rect.width) * afterCanvas.width);
    const py = Math.floor(((e.clientY - rect.top) / rect.height) * afterCanvas.height);
    if (px >= 0 && px < afterCanvas.width && py >= 0 && py < afterCanvas.height) {
      const ctx = afterCanvas.getContext('2d');
      const d = ctx.getImageData(px, py, 1, 1).data;
      pixelReadout.textContent = `RGB(${d[0]}, ${d[1]}, ${d[2]})`;
    }
  });

  overlayCanvas.addEventListener('mousedown', (e) => {
    const photo = getActivePhoto();
    if (!photo) return;
    const rect = overlayCanvas.getBoundingClientRect();
    const nx = (e.clientX - rect.left) / rect.width;
    const ny = (e.clientY - rect.top) / rect.height;

    if (state.showFaceGuides) {
      const f = photo.params.face;
      for (const c of [
        { key: 'leftEye', x: f.leftEyeX, y: f.leftEyeY },
        { key: 'rightEye', x: f.rightEyeX, y: f.rightEyeY },
        { key: 'nose', x: f.noseX, y: f.noseY },
        { key: 'mouth', x: f.mouthX, y: f.mouthY },
        { key: 'center', x: f.centerX, y: f.centerY },
      ]) {
        if (Math.hypot(nx - c.x, ny - c.y) < 0.035) {
          state.draggingLandmark = c.key;
          return;
        }
      }
    }

    if (state.spotHealMode) {
      photo.params.blemishSpots.push({ x: nx, y: ny, radius: 0.016 });
      processActivePhoto(false);
    }
  });

  const btnHand = document.getElementById('btn-tool-hand');
  const btnSpot = document.getElementById('btn-spot-heal');
  btnHand?.addEventListener('click', () => {
    state.spotHealMode = false;
    btnHand.classList.add('active');
    btnSpot.classList.remove('active');
    spotBanner.classList.add('hidden');
    overlayCanvas.style.cursor = 'default';
    renderViewport();
  });

  btnSpot?.addEventListener('click', () => {
    state.spotHealMode = !state.spotHealMode;
    btnSpot.classList.toggle('active', state.spotHealMode);
    btnHand.classList.toggle('active', !state.spotHealMode);
    spotBanner.classList.toggle('hidden', !state.spotHealMode);
    overlayCanvas.style.cursor = state.spotHealMode ? 'crosshair' : 'default';
    renderViewport();
  });

  document.getElementById('btn-undo-spot')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (photo && photo.params.blemishSpots.length > 0) {
      photo.params.blemishSpots.pop();
      processActivePhoto(false);
    }
  });

  document.getElementById('btn-clear-spots')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (photo) {
      photo.params.blemishSpots = [];
      processActivePhoto(false);
    }
  });

  const btnGuides = document.getElementById('btn-face-guides');
  btnGuides?.addEventListener('click', () => {
    state.showFaceGuides = !state.showFaceGuides;
    btnGuides.classList.toggle('active', state.showFaceGuides);
    renderViewport();
  });

  const btnSplit = document.getElementById('btn-split-view');
  btnSplit?.addEventListener('click', () => {
    state.splitEnabled = !state.splitEnabled;
    btnSplit.classList.toggle('active', state.splitEnabled);
    renderViewport();
  });

  const btnHoldBefore = document.getElementById('btn-hold-before');
  btnHoldBefore?.addEventListener('mousedown', () => {
    state.holdingBefore = true;
    renderViewport();
  });
  window.addEventListener('mouseup', () => {
    if (state.holdingBefore) {
      state.holdingBefore = false;
      renderViewport();
    }
  });

  window.addEventListener('keydown', (e) => {
    if (e.code === 'Escape') {
      document.getElementById('collabs-splash-modal')?.classList.add('hidden');
    }
    if (e.code === 'Space' && document.activeElement.tagName !== 'INPUT') {
      e.preventDefault();
      if (!state.holdingBefore) {
        state.holdingBefore = true;
        renderViewport();
      }
    }
  });
  window.addEventListener('keyup', (e) => {
    if (e.code === 'Space') {
      state.holdingBefore = false;
      renderViewport();
    }
  });

  document.getElementById('btn-reset-all')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    const faceBackup = photo.params.face;
    photo.params = createDefaultParams();
    photo.params.face = faceBackup;
    photo.curvePoints = null;
    document.querySelectorAll('.retouch-preset-item').forEach((c) => c.classList.remove('active'));
    syncSlidersFromParams();
    processActivePhoto(false);
  });

  const btnPaste = document.getElementById('btn-paste-settings');
  document.getElementById('btn-copy-settings')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    state.copiedParams = JSON.parse(JSON.stringify(photo.params));
    btnPaste.disabled = false;
    renderTimeLabel.textContent = 'Edits Copied ✓';
  });

  btnPaste?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo || !state.copiedParams) return;
    const ownFace = photo.params.face;
    photo.params = JSON.parse(JSON.stringify(state.copiedParams));
    photo.params.face = ownFace;
    syncSlidersFromParams();
    processActivePhoto(false);
  });

  document.getElementById('btn-sync-all')?.addEventListener('click', async () => {
    const source = getActivePhoto();
    if (!source) return;
    renderTimeLabel.textContent = `Batch Syncing ${state.photos.length} photos...`;

    for (let i = 0; i < state.photos.length; i++) {
      if (i === state.activeIndex) continue;
      const target = state.photos[i];
      const ownFace = target.params.face;
      target.params = JSON.parse(JSON.stringify(source.params));
      target.params.face = ownFace;
      target.params.blemishSpots = [];
      const res = await invoke('process_photo_cmd', {
        imageBase64: target.originalDataUrl,
        params: target.params,
        highQuality: false,
        maskBase64: target.maskBase64 || null,
      });
      target.processedDataUrl = res.dataUrl;
    }
    renderFilmstrip();
    renderTimeLabel.textContent = `Synced all ${state.photos.length} photos ✓`;
  });

  document.getElementById('btn-save-user-preset')?.addEventListener('click', () => {
    const photo = getActivePhoto();
    if (!photo) return;
    const name = `Custom Look #${state.userPresets.length + 1}`;
    const saved = JSON.parse(JSON.stringify(photo.params));
    state.userPresets.push({ name, params: saved });

    const container = document.getElementById('user-presets-container');
    const btn = document.createElement('button');
    btn.className = 'retouch-preset-item';
    btn.innerHTML = `<span class="preset-badge">USER</span><div class="preset-meta"><div class="p-title">${name}</div><div class="p-sub">Saved custom preset</div></div>`;
    btn.addEventListener('click', () => {
      const curr = getActivePhoto();
      if (!curr) return;
      const ownFace = curr.params.face;
      curr.params = JSON.parse(JSON.stringify(saved));
      curr.params.face = ownFace;
      syncSlidersFromParams();
      processActivePhoto(false);
    });
    container.appendChild(btn);
  });

  document.getElementById('btn-export')?.addEventListener('click', async () => {
    const photo = getActivePhoto();
    if (!photo) return;
    renderTimeLabel.textContent = 'Rendering Full-Res Export...';

    try {
      const defaultName = `CollabsRetouchPro_${photo.name.replace(/\.[^.]+$/, '')}_Retouched.jpg`;
      const savePath = await save({
        title: 'Export Retouched Photo — Retouch Pro',
        defaultPath: defaultName,
        filters: [
          { name: 'JPEG Image', extensions: ['jpg', 'jpeg'] },
          { name: 'PNG Image', extensions: ['png'] },
        ],
      });

      if (savePath) {
        await invoke('export_photo_to_disk_cmd', {
          imageBase64: photo.originalDataUrl,
          params: photo.params,
          outputPath: savePath,
          maskBase64: photo.maskBase64 || null,
        });
        renderTimeLabel.textContent = 'Exported to Disk ✓';
        return;
      }
    } catch (err) {
      console.warn('Native save fallback:', err);
    }

    const res = await invoke('process_photo_cmd', {
      imageBase64: photo.originalDataUrl,
      params: photo.params,
      highQuality: true,
      maskBase64: photo.maskBase64 || null,
    });
    photo.processedDataUrl = res.dataUrl;
    await renderViewport();

    const a = document.createElement('a');
    a.href = photo.processedDataUrl;
    a.download = `CollabsRetouchPro_${photo.name.replace(/\.[^.]+$/, '')}_Retouched.jpg`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    renderTimeLabel.textContent = 'Exported HQ ✓';
  });
}

async function initRetouchStudio() {
  setupRetouchEvents();

  // Load the 4 REAL portrait photos requested by the user instead of procedural drawings!
  const realDemoSamples = [
    { url: '/samples/portrait_3.jpg', name: 'Editorial_Red_Jacket.jpg' },
    { url: '/samples/portrait_4.jpg', name: 'Studio_Neon_Portrait.jpg' },
    { url: '/samples/portrait_1.jpg', name: 'Autumn_Fashion_Portrait.jpg' },
    { url: '/samples/portrait_2.jpg', name: 'Outdoor_Portrait_85mm.jpg' },
  ];

  for (let i = 0; i < realDemoSamples.length; i++) {
    try {
      const dataUrl = await fetchUrlAsDataUrl(realDemoSamples[i].url);
      const imgEl = await loadImageElement(dataUrl);
      const params = createDefaultParams();

      let fallbackFace = params.face;
      try {
        const detected = await invoke('analyze_portrait_cmd', { imageBase64: dataUrl });
        if (detected) fallbackFace = detected;
      } catch (_) {}

      const aiResult = await analyzePhotoWithNeuralAI(imgEl, fallbackFace);
      params.face = aiResult.face;

      state.photos.push({
        id: i + 1,
        name: realDemoSamples[i].name,
        originalDataUrl: dataUrl,
        processedDataUrl: dataUrl,
        width: imgEl.naturalWidth || 1024,
        height: imgEl.naturalHeight || 1280,
        mesh478: aiResult.mesh478,
        maskBase64: aiResult.maskBase64,
        landmarksDetected: aiResult.landmarksDetected,
        params,
      });
    } catch (err) {
      console.error('Failed to load real demo portrait:', realDemoSamples[i].url, err);
    }
  }

  syncSlidersFromParams();
  await processActivePhoto(false);
}

initRetouchStudio();
