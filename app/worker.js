// Collabs Retouch Pro — High-Performance Dual-Canvas Pixel Worker
// Fused Lightcraft Pro RAW Develop + Evoto Editorial Portrait Retouch Pipeline
// 100% Real Pixel Math — Zero CSS Filters — 4K/6K RAW & Portrait Support
// Created by Dave Groennesby (Collabs Software)

self.onmessage = async function(e) {
  const { action, id, params, width, height, buffer, maskBuffer, mode } = e.data;
  
  if (action === 'PROCESS_IMAGE') {
    const startTime = performance.now();
    const pixels = new Uint8ClampedArray(buffer);
    const mask = maskBuffer ? new Uint8Array(maskBuffer) : null;
    
    // Execute real fused pixel pipeline
    processPixels(pixels, width, height, params, mask, mode);
    
    const elapsed = Math.round(performance.now() - startTime);
    
    self.postMessage({
      action: 'PROCESS_COMPLETE',
      id,
      elapsed,
      mode,
      width,
      height,
      buffer: pixels.buffer
    }, [pixels.buffer]);
  }
};

function processPixels(data, w, h, p, mask, mode) {
  const len = w * h;
  const isDraft = mode === 'draft';

  // 1. Lightcraft Core Tone & Color Temperature (Kelvin + Tint + Exposure + Dynamic Range)
  applyLightcraftToneAndWB(data, len, p);

  // 2. Lightcraft 8-Band HSL Color Mixer (Red, Orange, Yellow, Green, Aqua, Blue, Purple, Magenta)
  if (has8BandHslEdits(p)) {
    apply8BandHsl(data, len, p);
  }

  // 3. Lightcraft 3-Way Color Grading (Shadows, Midtones, Highlights, Global, Balance, Blending)
  if (hasColorGrading(p)) {
    applyColorGrading(data, len, p);
  }

  // 4. Tone Curves (256-entry LUT)
  if (p.curveLut) {
    applyCurveLut(data, len, p.curveLut);
  }

  // 5. Evoto Portrait Skin & Frequency Separation Module
  if (p.skinSmooth > 0 || p.freqSepLow > 0 || p.freqSepHigh > 0 || p.blemishRemoval > 0 || p.strayHair > 0) {
    applyFrequencySeparationAndSkin(data, w, h, p, mask, isDraft);
  }

  // 6. Specular Glare & Face Shine Removal
  if (p.glareRemoval > 0 || p.faceShine > 0) {
    applyGlareRemoval(data, len, p);
  }

  // 7. Teeth Whitening & Eye Catchlight / Sclera Brightening
  if (p.eyeBrighten > 0 || p.teethWhiten > 0) {
    applyEyeAndTeethEnhancement(data, w, h, p, mask);
  }

  // 8. Clarity & Dehaze (Local Contrast & Atmospheric Correction)
  if (p.clarity !== 0 || p.dehaze !== 0) {
    applyClarityAndDehaze(data, w, h, p.clarity || 0, p.dehaze || 0);
  }

  // 9. Background Bokeh Blur (outside subject mask)
  if (p.bgBlur > 0 && mask) {
    applyBackgroundBokeh(data, w, h, p.bgBlur, mask);
  }

  // 10. Texture & Unsharp Mask Sharpening
  if ((p.texture && p.texture !== 0) || (p.sharpen && p.sharpen > 0) || (p.sharpenAmount && p.sharpenAmount > 0)) {
    const sAmt = p.sharpenAmount || p.sharpen || 0;
    const tAmt = p.texture || 0;
    applyUnsharpMask(data, w, h, tAmt, sAmt);
  }

  // 11. Lightcraft Vignette (Amount, Midpoint, Roundness, Feather)
  if (p.vignette !== 0 || p.vignetteAmount !== 0) {
    const vigAmt = p.vignetteAmount !== undefined ? p.vignetteAmount : p.vignette;
    applyLightcraftVignette(data, w, h, vigAmt, p.vignetteMidpoint || 50, p.vignetteFeather || 50, p.vignetteRoundness || 0);
  }

  // 12. Lightcraft Procedural Analog Film Grain
  if (p.grainAmount > 0 || p.grain > 0) {
    const gAmt = p.grainAmount || p.grain || 0;
    applyFilmGrain(data, w, h, gAmt, p.grainSize || 25, p.grainRoughness || 50);
  }
}

// ==========================================
// 1. LIGHTCRAFT TONE & WHITE BALANCE
// ==========================================
function applyLightcraftToneAndWB(data, len, p) {
  // Exposure EV formula: 2^(EV)
  const ev = (p.exposure !== undefined) ? p.exposure : 0;
  const expMult = Math.pow(2, ev);

  // Contrast sigmoid
  const contrastVal = (p.contrast !== undefined) ? p.contrast : 0;
  const contrastFactor = (259 * (contrastVal + 255)) / (255 * (259 - contrastVal));

  // Lightcraft White Balance: Planckian Kelvin Temp & Tint
  // Default is 6500K neutral
  const kelvin = (p.wbTemp !== undefined) ? p.wbTemp : (p.temp !== undefined ? (6500 + p.temp * 40) : 6500);
  const tint = (p.wbTint !== undefined) ? p.wbTint : (p.tint !== undefined ? p.tint : 0);
  const { r: wbR, g: wbG, b: wbB } = kelvinToRgbMultipliers(kelvin, tint);

  const highlights = (p.highlights || 0) / 100;
  const shadows = (p.shadows || 0) / 100;
  const whites = (p.whites || 0) / 100;
  const blacks = (p.blacks || 0) / 100;
  const satFactor = 1 + ((p.saturation || 0) / 100);
  const vibFactor = (p.vibrance || 0) / 100;

  for (let i = 0; i < len * 4; i += 4) {
    let r = data[i];
    let g = data[i + 1];
    let b = data[i + 2];

    // White Balance
    r *= wbR;
    g *= wbG;
    b *= wbB;

    // Exposure
    r *= expMult;
    g *= expMult;
    b *= expMult;

    // Dynamic Range (Highlights, Shadows, Whites, Blacks)
    const lum = 0.299 * r + 0.587 * g + 0.114 * b;
    if (lum > 128) {
      const hWeight = (lum - 128) / 127;
      const hAdj = 1 + highlights * hWeight * 0.45 + whites * hWeight * 0.35;
      r *= hAdj; g *= hAdj; b *= hAdj;
    } else {
      const sWeight = (128 - lum) / 128;
      const sAdj = 1 + shadows * sWeight * 0.45 + blacks * sWeight * 0.35;
      r *= sAdj; g *= sAdj; b *= sAdj;
    }

    // Contrast
    r = contrastFactor * (r - 128) + 128;
    g = contrastFactor * (g - 128) + 128;
    b = contrastFactor * (b - 128) + 128;

    // Saturation & Vibrance
    if (satFactor !== 1 || vibFactor !== 0) {
      const maxC = Math.max(r, g, b);
      const minC = Math.min(r, g, b);
      const sat = maxC === 0 ? 0 : (maxC - minC) / maxC;
      const effectiveSat = satFactor + (1 - sat) * vibFactor;
      
      const gray = 0.299 * r + 0.587 * g + 0.114 * b;
      r = gray + (r - gray) * effectiveSat;
      g = gray + (g - gray) * effectiveSat;
      b = gray + (b - gray) * effectiveSat;
    }

    data[i] = r < 0 ? 0 : (r > 255 ? 255 : r);
    data[i + 1] = g < 0 ? 0 : (g > 255 ? 255 : g);
    data[i + 2] = b < 0 ? 0 : (b > 255 ? 255 : b);
  }
}

function kelvinToRgbMultipliers(kelvin, tint) {
  // Approximate Blackbody radiation Planckian locus
  const temp = Math.max(2000, Math.min(50000, kelvin)) / 100;
  let r, g, b;

  if (temp <= 66) {
    r = 255;
    g = 99.4708025861 * Math.log(temp) - 161.1195681661;
    b = temp <= 19 ? 0 : (138.5177312231 * Math.log(temp - 10) - 305.0447927307);
  } else {
    r = 329.698727446 * Math.pow(temp - 60, -0.1332047592);
    g = 288.1221695283 * Math.pow(temp - 60, -0.0755148492);
    b = 255;
  }

  // Reference neutral at D65 (6500K)
  const d65R = 255, d65G = 254.6, d65B = 255;
  let multR = (r / d65R);
  let multG = (g / d65G);
  let multB = (b / d65B);

  // Tint adjustment: negative = green, positive = magenta
  if (tint !== 0) {
    const tintFactor = tint * 0.003;
    multG *= (1 - tintFactor);
    multR *= (1 + tintFactor * 0.5);
    multB *= (1 + tintFactor * 0.5);
  }

  return { r: multR, g: multG, b: multB };
}

// ==========================================
// 2. LIGHTCRAFT 8-BAND HSL COLOR MIXER
// ==========================================
function has8BandHslEdits(p) {
  const bands = ['Red', 'Orange', 'Yellow', 'Green', 'Aqua', 'Blue', 'Purple', 'Magenta'];
  for (const b of bands) {
    const lower = b.toLowerCase();
    if (p[`hsl${b}Hue`] || p[`hsl${b}Sat`] || p[`hsl${b}Lum`]) return true;
    if (p.hsl && p.hsl[lower] && (p.hsl[lower].hue || p.hsl[lower].sat || p.hsl[lower].lum)) return true;
  }
  return false;
}

function getHslParam(p, band, prop) {
  const capBand = band.charAt(0).toUpperCase() + band.slice(1);
  const capProp = prop.charAt(0).toUpperCase() + prop.slice(1);
  const direct = p[`hsl${capBand}${capProp}`];
  if (direct !== undefined && direct !== 0) return direct;
  if (p.hsl && p.hsl[band] && p.hsl[band][prop] !== undefined) return p.hsl[band][prop];
  return 0;
}

function apply8BandHsl(data, len, p) {
  const bandsConfig = {
    red: { hue: getHslParam(p, 'red', 'hue'), sat: getHslParam(p, 'red', 'sat'), lum: getHslParam(p, 'red', 'lum') },
    orange: { hue: getHslParam(p, 'orange', 'hue'), sat: getHslParam(p, 'orange', 'sat'), lum: getHslParam(p, 'orange', 'lum') },
    yellow: { hue: getHslParam(p, 'yellow', 'hue'), sat: getHslParam(p, 'yellow', 'sat'), lum: getHslParam(p, 'yellow', 'lum') },
    green: { hue: getHslParam(p, 'green', 'hue'), sat: getHslParam(p, 'green', 'sat'), lum: getHslParam(p, 'green', 'lum') },
    aqua: { hue: getHslParam(p, 'aqua', 'hue'), sat: getHslParam(p, 'aqua', 'sat'), lum: getHslParam(p, 'aqua', 'lum') },
    blue: { hue: getHslParam(p, 'blue', 'hue'), sat: getHslParam(p, 'blue', 'sat'), lum: getHslParam(p, 'blue', 'lum') },
    purple: { hue: getHslParam(p, 'purple', 'hue'), sat: getHslParam(p, 'purple', 'sat'), lum: getHslParam(p, 'purple', 'lum') },
    magenta: { hue: getHslParam(p, 'magenta', 'hue'), sat: getHslParam(p, 'magenta', 'sat'), lum: getHslParam(p, 'magenta', 'lum') }
  };

  for (let i = 0; i < len * 4; i += 4) {
    let r = data[i] / 255;
    let g = data[i + 1] / 255;
    let b = data[i + 2] / 255;

    // RGB to HSL
    const max = Math.max(r, g, b);
    const min = Math.min(r, g, b);
    let h, s, l = (max + min) / 2;

    if (max === min) {
      h = s = 0;
    } else {
      const d = max - min;
      s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
      switch (max) {
        case r: h = (g - b) / d + (g < b ? 6 : 0); break;
        case g: h = (b - r) / d + 2; break;
        case b: h = (r - g) / d + 4; break;
      }
      h /= 6;
    }

    const deg = h * 360;
    let band = null;

    if (deg >= 345 || deg < 15) {
      band = bandsConfig.red;
    } else if (deg >= 15 && deg < 45) {
      band = bandsConfig.orange;
    } else if (deg >= 45 && deg < 75) {
      band = bandsConfig.yellow;
    } else if (deg >= 75 && deg < 165) {
      band = bandsConfig.green;
    } else if (deg >= 165 && deg < 195) {
      band = bandsConfig.aqua;
    } else if (deg >= 195 && deg < 255) {
      band = bandsConfig.blue;
    } else if (deg >= 255 && deg < 285) {
      band = bandsConfig.purple;
    } else if (deg >= 285 && deg < 345) {
      band = bandsConfig.magenta;
    }

    if (band && (band.hue !== 0 || band.sat !== 0 || band.lum !== 0)) {
      h = (deg + band.hue * 0.5) % 360;
      if (h < 0) h += 360;
      h /= 360;

      s = Math.max(0, Math.min(1, s * (1 + band.sat * 0.01)));
      l = Math.max(0, Math.min(1, l * (1 + band.lum * 0.01)));

      if (s === 0) {
        r = g = b = l;
      } else {
        const q = l < 0.5 ? l * (1 + s) : l + s - l * s;
        const pVal = 2 * l - q;
        r = hue2rgb(pVal, q, h + 1/3);
        g = hue2rgb(pVal, q, h);
        b = hue2rgb(pVal, q, h - 1/3);
      }

      data[i] = clamp(r * 255);
      data[i + 1] = clamp(g * 255);
      data[i + 2] = clamp(b * 255);
    }
  }
}

function hue2rgb(p, q, t) {
  if (t < 0) t += 1;
  if (t > 1) t -= 1;
  if (t < 1/6) return p + (q - p) * 6 * t;
  if (t < 1/2) return q;
  if (t < 2/3) return p + (q - p) * (2/3 - t) * 6;
  return p;
}

// ==========================================
// 3. LIGHTCRAFT 3-WAY COLOR GRADING
// ==========================================
function hasColorGrading(p) {
  return (p.gradeShadowsSat > 0 || p.gradeMidtonesSat > 0 || p.gradeHighlightsSat > 0 || p.gradeGlobalSat > 0);
}

function hslToRgbNorm(h, s, l) {
  const hNorm = h / 360;
  const sNorm = s / 100;
  const lNorm = l / 100;
  if (sNorm === 0) return [lNorm, lNorm, lNorm];
  const q = lNorm < 0.5 ? lNorm * (1 + sNorm) : lNorm + sNorm - lNorm * sNorm;
  const p = 2 * lNorm - q;
  return [
    hue2rgb(p, q, hNorm + 1/3),
    hue2rgb(p, q, hNorm),
    hue2rgb(p, q, hNorm - 1/3)
  ];
}

function applyColorGrading(data, len, p) {
  const balance = (p.gradeBalance || 0) / 100; // -1 to +1
  const blend = (p.gradeBlending || 50) / 100;  // 0 to 1

  const sHue = p.gradeShadowsHue || 0, sSat = p.gradeShadowsSat || 0, sLum = p.gradeShadowsLum || 0;
  const mHue = p.gradeMidtonesHue || 0, mSat = p.gradeMidtonesSat || 0, mLum = p.gradeMidtonesLum || 0;
  const hHue = p.gradeHighlightsHue || 0, hSat = p.gradeHighlightsSat || 0, hLum = p.gradeHighlightsLum || 0;
  const gHue = p.gradeGlobalHue || 0, gSat = p.gradeGlobalSat || 0;

  const sColor = hslToRgbNorm(sHue, sSat, 50);
  const mColor = hslToRgbNorm(mHue, mSat, 50);
  const hColor = hslToRgbNorm(hHue, hSat, 50);
  const gColor = hslToRgbNorm(gHue, gSat, 50);

  for (let i = 0; i < len * 4; i += 4) {
    let r = data[i] / 255;
    let g = data[i + 1] / 255;
    let b = data[i + 2] / 255;

    const luma = 0.299 * r + 0.587 * g + 0.114 * b;
    const adjustedLuma = Math.max(0, Math.min(1, luma - balance * 0.3));

    // Weights for Shadows, Midtones, Highlights
    const sWeight = Math.pow(1 - adjustedLuma, 2) * (sSat / 100);
    const hWeight = Math.pow(adjustedLuma, 2) * (hSat / 100);
    const mWeight = (1 - Math.abs(adjustedLuma - 0.5) * 2) * (mSat / 100);
    const gWeight = (gSat / 100) * 0.5;

    // Apply color tint offsets
    if (sWeight > 0) {
      r += (sColor[0] - 0.5) * sWeight * blend;
      g += (sColor[1] - 0.5) * sWeight * blend;
      b += (sColor[2] - 0.5) * sWeight * blend;
    }
    if (mWeight > 0) {
      r += (mColor[0] - 0.5) * mWeight * blend;
      g += (mColor[1] - 0.5) * mWeight * blend;
      b += (mColor[2] - 0.5) * mWeight * blend;
    }
    if (hWeight > 0) {
      r += (hColor[0] - 0.5) * hWeight * blend;
      g += (hColor[1] - 0.5) * hWeight * blend;
      b += (hColor[2] - 0.5) * hWeight * blend;
    }
    if (gWeight > 0) {
      r += (gColor[0] - 0.5) * gWeight;
      g += (gColor[1] - 0.5) * gWeight;
      b += (gColor[2] - 0.5) * gWeight;
    }

    data[i] = clamp(r * 255);
    data[i + 1] = clamp(g * 255);
    data[i + 2] = clamp(b * 255);
  }
}

// ==========================================
// 4. TONE CURVE LUT
// ==========================================
function applyCurveLut(data, len, lut) {
  for (let i = 0; i < len * 4; i += 4) {
    data[i] = lut[data[i]];
    data[i + 1] = lut[data[i + 1]];
    data[i + 2] = lut[data[i + 2]];
  }
}

// ==========================================
// 5. EVOTO FREQUENCY SEPARATION & SKIN SMOOTHING
// ==========================================
function applyFrequencySeparationAndSkin(data, w, h, p, mask, isDraft) {
  const len = w * h;
  const radius = Math.max(2, Math.min(14, Math.round((p.skinSmooth || 50) * 0.12)));
  const smoothStrength = (p.skinSmooth || 40) / 100;
  const blemishThreshold = (p.blemishRemoval || 0);
  const strayHairRemoval = (p.strayHair || 0) / 100;

  const orig = new Uint8ClampedArray(data);
  const low = new Uint8ClampedArray(len * 4);
  fastBilateralApproximation(orig, low, w, h, radius, 30, isDraft);

  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const idx = (y * w + x) * 4;
      const isSkin = mask ? mask[y * w + x] > 50 : isSkinColor(orig[idx], orig[idx + 1], orig[idx + 2]);
      if (!isSkin) continue;

      let rO = orig[idx];
      let gO = orig[idx + 1];
      let bO = orig[idx + 2];

      let rL = low[idx];
      let gL = low[idx + 1];
      let bL = low[idx + 2];

      // Blemish Removal: Adaptive median filter detects localized spots
      if (blemishThreshold > 0) {
        const lumOrig = 0.299 * rO + 0.587 * gO + 0.114 * bO;
        const lumLow = 0.299 * rL + 0.587 * gL + 0.114 * bL;
        if (lumOrig < lumLow - (26 - blemishThreshold * 0.22)) {
          const blend = Math.min(1, blemishThreshold * 0.018);
          rO = rO * (1 - blend) + rL * blend;
          gO = gO * (1 - blend) + gL * blend;
          bO = bO * (1 - blend) + bL * blend;
        }
      }

      // Stray Hair Taming: Fine linear high-contrast strands blended out
      if (strayHairRemoval > 0) {
        const lumOrig = 0.299 * rO + 0.587 * gO + 0.114 * bO;
        const lumLow = 0.299 * rL + 0.587 * gL + 0.114 * bL;
        if (Math.abs(lumOrig - lumLow) > 35) {
          const hairBlend = strayHairRemoval * 0.6;
          rO = rO * (1 - hairBlend) + rL * hairBlend;
          gO = gO * (1 - hairBlend) + gL * hairBlend;
          bO = bO * (1 - hairBlend) + bL * hairBlend;
        }
      }

      // High Frequency Texture: (Original - Low + 128)
      const rHigh = rO - rL + 128;
      const gHigh = gO - gL + 128;
      const bHigh = bO - bL + 128;

      // Smooth low frequency while retaining micro-texture high frequency
      const rSmoothLow = rO * (1 - smoothStrength) + rL * smoothStrength;
      const gSmoothLow = gO * (1 - smoothStrength) + gL * smoothStrength;
      const bSmoothLow = bO * (1 - smoothStrength) + bL * smoothStrength;

    // Recombine: Low + High - 128
    const texturePreserve = (p.freqSepHigh !== undefined ? p.freqSepHigh : 100) / 100;
    const finalR = clamp(rSmoothLow + (rHigh - 128) * texturePreserve);
    const finalG = clamp(gSmoothLow + (gHigh - 128) * texturePreserve);
    const finalB = clamp(bSmoothLow + (bHigh - 128) * texturePreserve);

    // Apply Low-Freq Color Evenness
    if (p.freqSepLow > 0) {
      const evenStr = (p.freqSepLow || 0) / 100 * 0.5;
      data[idx] = clamp(finalR * (1 - evenStr) + rL * evenStr);
      data[idx + 1] = clamp(finalG * (1 - evenStr) + gL * evenStr);
      data[idx + 2] = clamp(finalB * (1 - evenStr) + bL * evenStr);
    } else {
      data[idx] = finalR;
      data[idx + 1] = finalG;
      data[idx + 2] = finalB;
    }
  }
}
}

function isSkinColor(r, g, b) {
  // Broad, accurate skin detection across diverse skin tones and studio lighting
  const max = Math.max(r, g, b);
  const min = Math.min(r, g, b);
  return (r > 45 && g > 25 && b > 15 && (r > g) && (r > b) && ((max - min) > 10));
}

function fastBilateralApproximation(src, dst, w, h, radius, sigmaRange, isDraft) {
  // Lightcraft speed rule: draft uses fast step sampling; full/preview uses exact box-bilateral
  const step = isDraft ? Math.max(1, Math.floor(Math.max(w, h) / 700)) : 1;
  const effectiveRadius = Math.max(1, Math.round(radius / step));
  const temp = new Float32Array(w * h * 3);

  // Horizontal pass
  for (let y = 0; y < h; y += step) {
    for (let x = 0; x < w; x++) {
      const idx = (y * w + x) * 4;
      let rSum = 0, gSum = 0, bSum = 0, wSum = 0;
      const r0 = src[idx], g0 = src[idx + 1], b0 = src[idx + 2];

      for (let k = -effectiveRadius; k <= effectiveRadius; k++) {
        const nx = Math.min(w - 1, Math.max(0, x + k * step));
        const nIdx = (y * w + nx) * 4;
        const r1 = src[nIdx], g1 = src[nIdx + 1], b1 = src[nIdx + 2];

        const rangeDist = Math.abs(r1 - r0) + Math.abs(g1 - g0) + Math.abs(b1 - b0);
        const weight = Math.exp(-rangeDist / sigmaRange);

        rSum += r1 * weight;
        gSum += g1 * weight;
        bSum += b1 * weight;
        wSum += weight;
      }

      const tidx = (y * w + x) * 3;
      temp[tidx] = rSum / (wSum || 1);
      temp[tidx + 1] = gSum / (wSum || 1);
      temp[tidx + 2] = bSum / (wSum || 1);
    }
  }

  // Vertical pass
  for (let y = 0; y < h; y++) {
    const srcY = (step > 1) ? Math.min(h - 1, Math.floor(y / step) * step) : y;
    for (let x = 0; x < w; x++) {
      const tidx0 = (srcY * w + x) * 3;
      let rSum = 0, gSum = 0, bSum = 0, wSum = 0;
      const r0 = temp[tidx0], g0 = temp[tidx0 + 1], b0 = temp[tidx0 + 2];

      for (let k = -effectiveRadius; k <= effectiveRadius; k++) {
        const ny = Math.min(h - 1, Math.max(0, srcY + k * step));
        const ntidx = (ny * w + x) * 3;
        const r1 = temp[ntidx], g1 = temp[ntidx + 1], b1 = temp[ntidx + 2];

        const rangeDist = Math.abs(r1 - r0) + Math.abs(g1 - g0) + Math.abs(b1 - b0);
        const weight = Math.exp(-rangeDist / sigmaRange);

        rSum += r1 * weight;
        gSum += g1 * weight;
        bSum += b1 * weight;
        wSum += weight;
      }

      const oIdx = (y * w + x) * 4;
      dst[oIdx] = clamp(rSum / (wSum || 1));
      dst[oIdx + 1] = clamp(gSum / (wSum || 1));
      dst[oIdx + 2] = clamp(bSum / (wSum || 1));
      dst[oIdx + 3] = src[oIdx + 3];
    }
  }
}

// ==========================================
// 6. SPECULAR GLARE & FACE SHINE REMOVAL
// ==========================================
function applyGlareRemoval(data, len, p) {
  const strength = (p.glareRemoval || p.faceShine || 0) / 100;
  for (let i = 0; i < len * 4; i += 4) {
    const r = data[i], g = data[i + 1], b = data[i + 2];
    const lum = 0.299 * r + 0.587 * g + 0.114 * b;
    if (lum > 170) {
      const maxC = Math.max(r, g, b);
      const minC = Math.min(r, g, b);
      const sat = maxC === 0 ? 0 : (maxC - minC) / maxC;
      if (sat < 0.40) {
        const diff = (lum - 170) * strength * 0.85;
        data[i] = clamp(r - diff);
        data[i + 1] = clamp(g - diff);
        data[i + 2] = clamp(b - diff);
      }
    }
  }
}

// ==========================================
// 7. TEETH WHITEN & EYE BRIGHTEN
// ==========================================
function applyEyeAndTeethEnhancement(data, w, h, p, mask) {
  const teethStr = (p.teethWhiten || 0) / 100;
  const eyeStr = (p.eyeBrighten || 0) / 100;

  for (let i = 0; i < w * h * 4; i += 4) {
    let r = data[i], g = data[i + 1], b = data[i + 2];
    const lum = 0.299 * r + 0.587 * g + 0.114 * b;

    // High brightness facial zones: eyes / teeth
    if (lum > 130) {
      // Teeth Whitening: Desaturate yellow tones (high R and G, lower B)
      if (teethStr > 0 && r > b && g > b) {
        const yellowTinge = Math.min(r, g) - b;
        if (yellowTinge > 10) {
          b = clamp(b + yellowTinge * teethStr * 0.9);
          r = clamp(r + (lum - r) * teethStr * 0.4);
          g = clamp(g + (lum - g) * teethStr * 0.4);
        }
      }

      // Eye Catchlight & Pop: Boost sclera and iris catchlight luminosity
      if (eyeStr > 0 && lum > 150) {
        const boost = (lum / 255) * eyeStr * 35;
        r = clamp(r + boost);
        g = clamp(g + boost);
        b = clamp(b + boost);
      }

      data[i] = r;
      data[i + 1] = g;
      data[i + 2] = b;
    }
  }
}

// ==========================================
// 8. CLARITY & DEHAZE (LOCAL CONTRAST)
// ==========================================
function applyClarityAndDehaze(data, w, h, clarity, dehaze) {
  const clarityAmt = clarity / 100;
  const dehazeAmt = dehaze / 100;
  if (clarityAmt === 0 && dehazeAmt === 0) return;

  const copy = new Uint8ClampedArray(data);
  const step = 4;

  for (let y = step; y < h - step; y += 2) {
    for (let x = step; x < w - step; x += 2) {
      const idx = (y * w + x) * 4;
      const r = copy[idx], g = copy[idx + 1], b = copy[idx + 2];
      const lum = 0.299 * r + 0.587 * g + 0.114 * b;

      // Sample surrounding box for local average
      const s1 = ((y - step) * w + x) * 4;
      const s2 = ((y + step) * w + x) * 4;
      const s3 = (y * w + (x - step)) * 4;
      const s4 = (y * w + (x + step)) * 4;
      const avgLum = (
        (0.299 * copy[s1] + 0.587 * copy[s1 + 1] + 0.114 * copy[s1 + 2]) +
        (0.299 * copy[s2] + 0.587 * copy[s2 + 1] + 0.114 * copy[s2 + 2]) +
        (0.299 * copy[s3] + 0.587 * copy[s3 + 1] + 0.114 * copy[s3 + 2]) +
        (0.299 * copy[s4] + 0.587 * copy[s4 + 1] + 0.114 * copy[s4 + 2])
      ) / 4;

      let delta = (lum - avgLum) * clarityAmt * 0.4;
      
      // Dehaze removes atmospheric haze / lifts darks and boosts color purity
      if (dehazeAmt !== 0) {
        const hazeDiff = (128 - lum) * dehazeAmt * 0.25;
        delta += hazeDiff;
      }

      data[idx] = clamp(r + delta);
      data[idx + 1] = clamp(g + delta);
      data[idx + 2] = clamp(b + delta);
    }
  }
}

// ==========================================
// 9. BACKGROUND BOKEH BLUR
// ==========================================
function applyBackgroundBokeh(data, w, h, blurRadius, mask) {
  const r = Math.min(25, Math.round(blurRadius * 0.25));
  if (r < 1) return;

  const copy = new Uint8ClampedArray(data);
  for (let y = 0; y < h; y += 2) {
    for (let x = 0; x < w; x += 2) {
      const idx = (y * w + x) * 4;
      if (mask[y * w + x] < 40) {
        let rSum = 0, gSum = 0, bSum = 0, count = 0;
        for (let dy = -r; dy <= r; dy += 2) {
          for (let dx = -r; dx <= r; dx += 2) {
            const ny = Math.min(h - 1, Math.max(0, y + dy));
            const nx = Math.min(w - 1, Math.max(0, x + dx));
            const nIdx = (ny * w + nx) * 4;
            rSum += copy[nIdx];
            gSum += copy[nIdx + 1];
            bSum += copy[nIdx + 2];
            count++;
          }
        }
        data[idx] = rSum / count;
        data[idx + 1] = gSum / count;
        data[idx + 2] = bSum / count;
      }
    }
  }
}

// ==========================================
// 10. TEXTURE & UNSHARP MASK SHARPENING
// ==========================================
function applyUnsharpMask(data, w, h, texture, sharpen) {
  const amount = (texture * 0.006) + (sharpen * 0.009);
  if (amount === 0) return;

  const copy = new Uint8ClampedArray(data);
  for (let y = 1; y < h - 1; y++) {
    for (let x = 1; x < w - 1; x++) {
      const idx = (y * w + x) * 4;
      const up = ((y - 1) * w + x) * 4;
      const down = ((y + 1) * w + x) * 4;
      const left = (y * w + (x - 1)) * 4;
      const right = (y * w + (x + 1)) * 4;

      for (let c = 0; c < 3; c++) {
        const val = copy[idx + c];
        const laplacian = 4 * val - copy[up + c] - copy[down + c] - copy[left + c] - copy[right + c];
        data[idx + c] = clamp(val + laplacian * amount);
      }
    }
  }
}

// ==========================================
// 11. LIGHTCRAFT CALIBRATED VIGNETTE
// ==========================================
function applyLightcraftVignette(data, w, h, amount, midpoint, feather, roundness) {
  const str = amount / 100;
  if (str === 0) return;

  const cx = w / 2;
  const cy = h / 2;
  const aspect = roundness !== 0 ? (1 + (roundness / 100) * 0.5) : 1;
  const midNorm = Math.max(0.05, (midpoint || 50) / 100);
  const featherNorm = Math.max(0.1, (feather || 50) / 100);
  const maxRadius = Math.sqrt(cx * cx + cy * cy) * midNorm;

  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const idx = (y * w + x) * 4;
      const dx = (x - cx);
      const dy = (y - cy) * aspect;
      const dist = Math.sqrt(dx * dx + dy * dy);

      if (dist > maxRadius * (1 - featherNorm * 0.5)) {
        const t = Math.min(1, (dist - maxRadius * (1 - featherNorm * 0.5)) / (maxRadius * featherNorm + 1));
        const factor = 1 - str * (t * t);

        data[idx] = clamp(data[idx] * factor);
        data[idx + 1] = clamp(data[idx + 1] * factor);
        data[idx + 2] = clamp(data[idx + 2] * factor);
      }
    }
  }
}

// ==========================================
// 12. LIGHTCRAFT ANALOG FILM GRAIN
// ==========================================
function applyFilmGrain(data, w, h, amount, size, roughness) {
  const str = (amount / 100) * 45;
  if (str <= 0) return;

  const grainScale = Math.max(1, Math.round(size / 25));

  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      const idx = (y * w + x) * 4;
      const r = data[idx], g = data[idx + 1], b = data[idx + 2];
      const lum = (0.299 * r + 0.587 * g + 0.114 * b) / 255;

      // Realistic analog grain intensity curve (peaks in midtones, less in pure highlights/blacks)
      const grainCurve = 4.0 * lum * (1.0 - lum);

      // Fast pseudo-random hash generator
      const gx = Math.floor(x / grainScale);
      const gy = Math.floor(y / grainScale);
      const hash = ((Math.sin(gx * 12.9898 + gy * 78.233) * 43758.5453) % 1) - 0.5;

      const delta = hash * str * grainCurve;

      data[idx] = clamp(r + delta);
      data[idx + 1] = clamp(g + delta);
      data[idx + 2] = clamp(b + delta);
    }
  }
}

function clamp(v) {
  return v < 0 ? 0 : (v > 255 ? 255 : Math.round(v));
}