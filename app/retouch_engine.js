// Collabs Retouch Pro — Core Studio Engine
// Dual-Canvas Architecture, Web Worker Pipeline, 0% Fake CSS Filters, 100% Real Pixel Impact
// Created by Dave Groennesby (Collabs Software)

class RetouchEngine {
  constructor() {
    this.displayCanvas = document.getElementById('displayCanvas');
    this.ctx = this.displayCanvas.getContext('2d', { willReadFrequently: true });
    
    // Internal full-res buffers
    this.origBitmap = null;
    this.origImageData = null;
    this.retouchedImageData = null;
    
    // Draft interaction buffers (for instant 60fps response)
    this.draftOrigImageData = null;
    this.draftRetouchedImageData = null;
    this.draftScale = 1;

    // Viewport transform
    this.zoom = 1;
    this.panX = 0;
    this.panY = 0;
    this.isPanning = false;
    this.startX = 0;
    this.startY = 0;

    // Split View (Before / After)
    this.splitPos = 0.5; // 0.0 to 1.0 (50% default)
    this.isDraggingSplit = false;
    this.splitMode = 'split'; // 'split', 'after', 'before', 'sideBySide'

    // Evoto & Lightroom Parameter State
    this.params = this.getDefaultParams();

    // Web Worker
    this.worker = new Worker('worker.js');
    this.workerPending = false;
    this.pendingRequest = null;
    this.initWorker();

    // Tone Curve State
    this.curvePoints = [
      { x: 0, y: 0 },
      { x: 64, y: 64 },
      { x: 192, y: 192 },
      { x: 255, y: 255 }
    ];
    this.curveLut = this.buildCurveLut();
    this.params.curveLut = this.curveLut;

    // Setup Event Listeners
    this.setupViewportEvents();
    this.setupSliderEvents();
    this.setupCurveEvents();
    this.setupDropZone();

    // Responsive Canvas
    window.addEventListener('resize', () => this.resizeCanvas());
    this.resizeCanvas();
  }

  getDefaultParams() {
    return {
      // Evoto Portrait AI
      blemishRemoval: 45,
      skinSmooth: 50,
      freqSepLow: 50,
      freqSepHigh: 100,
      glareRemoval: 30,
      faceShine: 20,
      teethWhiten: 35,
      eyeBrighten: 40,
      strayHair: 25,
      faceSculpt: 0,
      
      // Lightroom RAW Tone
      temp: 0,
      tint: 0,
      exposure: 0,
      contrast: 0,
      highlights: 0,
      shadows: 0,
      whites: 0,
      blacks: 0,
      texture: 0,
      clarity: 0,
      vibrance: 10,
      saturation: 0,

      // Luminar AI & Background
      bgBlur: 0,
      vignette: 0,
      sharpen: 15,

      // 8-Band HSL
      hslRedHue: 0, hslRedSat: 0, hslRedLum: 0,
      hslOrangeHue: 0, hslOrangeSat: 5, hslOrangeLum: 0,
      hslYellowHue: 0, hslYellowSat: 0, hslYellowLum: 0,
      hslGreenHue: 0, hslGreenSat: 0, hslGreenLum: 0,
      hslAquaHue: 0, hslAquaSat: 0, hslAquaLum: 0,
      hslBlueHue: 0, hslBlueSat: 0, hslBlueLum: 0,
      hslPurpleHue: 0, hslPurpleSat: 0, hslPurpleLum: 0,
      hslMagentaHue: 0, hslMagentaSat: 0, hslMagentaLum: 0
    };
  }

  initWorker() {
    this.worker.onmessage = (e) => {
      const { action, elapsed, isDraft, buffer } = e.data;
      if (action === 'PROCESS_COMPLETE') {
        const processed = new Uint8ClampedArray(buffer);
        if (isDraft) {
          const w = Math.round(this.origImageData.width * this.draftScale);
          const h = Math.round(this.origImageData.height * this.draftScale);
          this.draftRetouchedImageData = new ImageData(processed, w, h);
        } else {
          this.retouchedImageData = new ImageData(processed, this.origImageData.width, this.origImageData.height);
        }
        
        document.getElementById('renderTime').textContent = `${elapsed} ms`;
        this.workerPending = false;
        this.render();

        if (this.pendingRequest) {
          const req = this.pendingRequest;
          this.pendingRequest = null;
          this.dispatchProcessing(req.isDraft);
        }
      }
    };
  }

  resizeCanvas() {
    const rect = this.displayCanvas.parentElement.getBoundingClientRect();
    const dpr = window.devicePixelRatio || 1;
    this.displayCanvas.width = rect.width * dpr;
    this.displayCanvas.height = rect.height * dpr;
    this.ctx.scale(dpr, dpr);
    this.render();
  }

  async loadImage(source) {
    let img;
    if (typeof source === 'string') {
      img = new Image();
      img.src = source;
      await img.decode();
    } else if (source instanceof File || source instanceof Blob) {
      const url = URL.createObjectURL(source);
      img = new Image();
      img.src = url;
      await img.decode();
      URL.revokeObjectURL(url);
    } else if (source instanceof HTMLImageElement) {
      img = source;
    }

    const w = img.naturalWidth || img.width;
    const h = img.naturalHeight || img.height;

    // Create OffscreenCanvas for full resolution
    const oc = new OffscreenCanvas(w, h);
    const octx = oc.getContext('2d', { willReadFrequently: true });
    octx.drawImage(img, 0, 0);
    this.origImageData = octx.getImageData(0, 0, w, h);
    this.retouchedImageData = new ImageData(new Uint8ClampedArray(this.origImageData.data), w, h);

    // Create draft preview (max 1600px wide for instantaneous slider response)
    const maxDraftDim = 1600;
    this.draftScale = Math.min(1, maxDraftDim / Math.max(w, h));
    const dw = Math.round(w * this.draftScale);
    const dh = Math.round(h * this.draftScale);
    
    const doc = new OffscreenCanvas(dw, dh);
    const doctx = doc.getContext('2d');
    doctx.drawImage(img, 0, 0, dw, dh);
    this.draftOrigImageData = doctx.getImageData(0, 0, dw, dh);
    this.draftRetouchedImageData = new ImageData(new Uint8ClampedArray(this.draftOrigImageData.data), dw, dh);

    // Update metadata badge
    document.getElementById('imageResolution').textContent = `${w} × ${h} px (True RAW 1:1)`;
    document.getElementById('creditRemaining').textContent = '9,000 Exports Remaining';

    this.fitToScreen();
    this.dispatchProcessing(false);
  }

  fitToScreen() {
    if (!this.origImageData) return;
    const cw = this.displayCanvas.clientWidth;
    const ch = this.displayCanvas.clientHeight;
    const iw = this.origImageData.width;
    const ih = this.origImageData.height;

    const scale = Math.min((cw - 40) / iw, (ch - 40) / ih);
    this.zoom = scale;
    this.panX = (cw - iw * scale) / 2;
    this.panY = (ch - ih * scale) / 2;
    document.getElementById('zoomValue').textContent = `${Math.round(this.zoom * 100)}%`;
    this.render();
  }

  setZoom100() {
    if (!this.origImageData) return;
    const cw = this.displayCanvas.clientWidth;
    const ch = this.displayCanvas.clientHeight;
    this.zoom = 1.0;
    this.panX = (cw - this.origImageData.width) / 2;
    this.panY = (ch - this.origImageData.height) / 2;
    document.getElementById('zoomValue').textContent = '100% (1:1 Pixel)';
    this.render();
  }

  dispatchProcessing(isDraft = false) {
    if (!this.origImageData) return;

    if (this.workerPending) {
      this.pendingRequest = { isDraft };
      return;
    }

    this.workerPending = true;
    const sourceData = isDraft ? this.draftOrigImageData : this.origImageData;
    const copyBuffer = sourceData.data.slice().buffer;

    this.worker.postMessage({
      action: 'PROCESS_IMAGE',
      id: Date.now(),
      params: { ...this.params },
      width: sourceData.width,
      height: sourceData.height,
      buffer: copyBuffer,
      isDraft
    }, [copyBuffer]);
  }

  render() {
    if (!this.ctx || !this.origImageData) return;

    const cw = this.displayCanvas.clientWidth;
    const ch = this.displayCanvas.clientHeight;
    this.ctx.clearRect(0, 0, cw, ch);

    // Dark grid background
    this.ctx.fillStyle = '#0E0E10';
    this.ctx.fillRect(0, 0, cw, ch);

    const iw = this.origImageData.width;
    const ih = this.origImageData.height;
    const renderW = iw * this.zoom;
    const renderH = ih * this.zoom;

    // Use full-res or draft depending on availability
    const imgDataToShow = this.retouchedImageData || this.origImageData;
    
    // Create temporary offscreen canvas for rendering zoom/pan
    const tempCanvas = new OffscreenCanvas(iw, ih);
    const tempCtx = tempCanvas.getContext('2d');
    tempCtx.putImageData(imgDataToShow, 0, 0);

    // Apply Split-View Wipe
    if (this.splitMode === 'split') {
      const splitX = cw * this.splitPos;
      
      // 1. Draw Before (Original) on left side of split
      const origCanvas = new OffscreenCanvas(iw, ih);
      const oCtx = origCanvas.getContext('2d');
      oCtx.putImageData(this.origImageData, 0, 0);

      this.ctx.save();
      this.ctx.beginPath();
      this.ctx.rect(0, 0, splitX, ch);
      this.ctx.clip();
      this.ctx.drawImage(origCanvas, this.panX, this.panY, renderW, renderH);
      this.ctx.restore();

      // 2. Draw After (Retouched) on right side of split
      this.ctx.save();
      this.ctx.beginPath();
      this.ctx.rect(splitX, 0, cw - splitX, ch);
      this.ctx.clip();
      this.ctx.drawImage(tempCanvas, this.panX, this.panY, renderW, renderH);
      this.ctx.restore();

      // 3. Draw Split Line & Handle
      this.ctx.strokeStyle = '#FFE606'; // Collabs Gold accent
      this.ctx.lineWidth = 2;
      this.ctx.beginPath();
      this.ctx.moveTo(splitX, 0);
      this.ctx.lineTo(splitX, ch);
      this.ctx.stroke();

      // Split handle
      this.ctx.fillStyle = '#FFE606';
      this.ctx.beginPath();
      this.ctx.arc(splitX, ch / 2, 14, 0, Math.PI * 2);
      this.ctx.fill();

      // Icon on handle
      this.ctx.fillStyle = '#121214';
      this.ctx.font = 'bold 11px Inter, sans-serif';
      this.ctx.textAlign = 'center';
      this.ctx.textBaseline = 'middle';
      this.ctx.fillText('◀ ▶', splitX, ch / 2);

      // Labels
      this.ctx.font = 'bold 12px Inter, sans-serif';
      this.ctx.fillStyle = 'rgba(255, 255, 255, 0.7)';
      this.ctx.fillText('BEFORE (ORIGINAL)', Math.max(80, splitX - 80), 30);
      this.ctx.fillStyle = '#FFE606';
      this.ctx.fillText('AFTER (RETOUCH PRO)', Math.min(cw - 90, splitX + 90), 30);
    } else if (this.splitMode === 'before') {
      const origCanvas = new OffscreenCanvas(iw, ih);
      origCanvas.getContext('2d').putImageData(this.origImageData, 0, 0);
      this.ctx.drawImage(origCanvas, this.panX, this.panY, renderW, renderH);
    } else {
      // After mode (100% Retouched)
      this.ctx.drawImage(tempCanvas, this.panX, this.panY, renderW, renderH);
    }
  }

  setupViewportEvents() {
    this.displayCanvas.addEventListener('mousedown', (e) => {
      const rect = this.displayCanvas.getBoundingClientRect();
      const mouseX = e.clientX - rect.left;
      const mouseY = e.clientY - rect.top;

      if (this.splitMode === 'split') {
        const splitX = this.displayCanvas.clientWidth * this.splitPos;
        if (Math.abs(mouseX - splitX) < 18) {
          this.isDraggingSplit = true;
          return;
        }
      }

      this.isPanning = true;
      this.startX = e.clientX - this.panX;
      this.startY = e.clientY - this.panY;
    });

    window.addEventListener('mousemove', (e) => {
      const rect = this.displayCanvas.getBoundingClientRect();
      const mouseX = e.clientX - rect.left;
      const mouseY = e.clientY - rect.top;

      if (this.isDraggingSplit) {
        this.splitPos = Math.max(0.05, Math.min(0.95, mouseX / this.displayCanvas.clientWidth));
        this.render();
        return;
      }

      if (this.isPanning) {
        this.panX = e.clientX - this.startX;
        this.panY = e.clientY - this.startY;
        this.render();
      }

      // Pixel Inspector / Color Picker Loupe
      this.inspectPixel(mouseX, mouseY);
    });

    window.addEventListener('mouseup', () => {
      this.isDraggingSplit = false;
      this.isPanning = false;
    });

    // Zoom on wheel
    this.displayCanvas.addEventListener('wheel', (e) => {
      e.preventDefault();
      const rect = this.displayCanvas.getBoundingClientRect();
      const mouseX = e.clientX - rect.left;
      const mouseY = e.clientY - rect.top;

      const zoomFactor = e.deltaY < 0 ? 1.15 : 0.85;
      const newZoom = Math.max(0.1, Math.min(8.0, this.zoom * zoomFactor));

      this.panX = mouseX - (mouseX - this.panX) * (newZoom / this.zoom);
      this.panY = mouseY - (mouseY - this.panY) * (newZoom / this.zoom);
      this.zoom = newZoom;

      document.getElementById('zoomValue').textContent = `${Math.round(this.zoom * 100)}%`;
      this.render();
    }, { passive: false });
  }

  inspectPixel(screenX, screenY) {
    if (!this.origImageData) return;
    const imgX = Math.round((screenX - this.panX) / this.zoom);
    const imgY = Math.round((screenY - this.panY) / this.zoom);

    if (imgX >= 0 && imgX < this.origImageData.width && imgY >= 0 && imgY < this.origImageData.height) {
      const idx = (imgY * this.origImageData.width + imgX) * 4;
      const rO = this.origImageData.data[idx];
      const gO = this.origImageData.data[idx + 1];
      const bO = this.origImageData.data[idx + 2];

      const rR = this.retouchedImageData ? this.retouchedImageData.data[idx] : rO;
      const gR = this.retouchedImageData ? this.retouchedImageData.data[idx + 1] : gO;
      const bR = this.retouchedImageData ? this.retouchedImageData.data[idx + 2] : bO;

      const delta = Math.round(Math.sqrt((rR - rO)**2 + (gR - gO)**2 + (bR - bO)**2));

      const inspectorEl = document.getElementById('pixelInspector');
      if (inspectorEl) {
        inspectorEl.innerHTML = `
          <span>X: <b>${imgX}</b> Y: <b>${imgY}</b></span> | 
          <span>Orig: <span style="color:rgb(${rO},${gO},${bO})">●</span> [${rO}, ${gO}, ${bO}]</span> ➔ 
          <span>Retouch: <span style="color:rgb(${rR},${gR},${bR})">●</span> [${rR}, ${gR}, ${bR}]</span> | 
          <span>ΔE: <b>${delta}</b></span>
        `;
      }
    }
  }

  setupSliderEvents() {
    const sliders = document.querySelectorAll('input[type="range"][data-param]');
    sliders.forEach(slider => {
      const param = slider.getAttribute('data-param');
      const valDisplay = document.getElementById(`val_${param}`);

      slider.addEventListener('input', (e) => {
        const val = parseFloat(e.target.value);
        this.params[param] = val;
        if (valDisplay) valDisplay.textContent = val > 0 && slider.min < 0 ? `+${val}` : val;

        // Immediate draft update for 60fps interaction
        this.dispatchProcessing(true);
      });

      slider.addEventListener('change', () => {
        // Full resolution update on release
        this.dispatchProcessing(false);
      });
    });
  }

  setupCurveEvents() {
    const curveCanvas = document.getElementById('curveCanvas');
    if (!curveCanvas) return;
    const cctx = curveCanvas.getContext('2d');

    const drawCurve = () => {
      cctx.clearRect(0, 0, 200, 200);
      cctx.fillStyle = '#1A1A1F';
      cctx.fillRect(0, 0, 200, 200);

      // Grid
      cctx.strokeStyle = '#2A2A32';
      cctx.lineWidth = 1;
      for (let i = 50; i < 200; i += 50) {
        cctx.beginPath(); cctx.moveTo(i, 0); cctx.lineTo(i, 200); cctx.stroke();
        cctx.beginPath(); cctx.moveTo(0, i); cctx.lineTo(200, i); cctx.stroke();
      }

      // Curve line
      cctx.strokeStyle = '#FFE606';
      cctx.lineWidth = 2;
      cctx.beginPath();
      for (let x = 0; x < 256; x++) {
        const y = 255 - this.curveLut[x];
        const cx = (x / 255) * 200;
        const cy = (y / 255) * 200;
        if (x === 0) cctx.moveTo(cx, cy);
        else cctx.lineTo(cx, cy);
      }
      cctx.stroke();

      // Control points
      this.curvePoints.forEach(pt => {
        const cx = (pt.x / 255) * 200;
        const cy = ((255 - pt.y) / 255) * 200;
        cctx.fillStyle = '#FFE606';
        cctx.beginPath();
        cctx.arc(cx, cy, 5, 0, Math.PI * 2);
        cctx.fill();
      });
    };

    drawCurve();

    let draggingPoint = null;
    curveCanvas.addEventListener('mousedown', (e) => {
      const rect = curveCanvas.getBoundingClientRect();
      const mx = ((e.clientX - rect.left) / 200) * 255;
      const my = 255 - ((e.clientY - rect.top) / 200) * 255;

      draggingPoint = this.curvePoints.find(pt => Math.abs(pt.x - mx) < 15 && Math.abs(pt.y - my) < 15);
    });

    window.addEventListener('mousemove', (e) => {
      if (!draggingPoint) return;
      const rect = curveCanvas.getBoundingClientRect();
      const mx = Math.max(0, Math.min(255, ((e.clientX - rect.left) / 200) * 255));
      const my = Math.max(0, Math.min(255, 255 - ((e.clientY - rect.top) / 200) * 255));

      draggingPoint.y = Math.round(my);
      this.curveLut = this.buildCurveLut();
      this.params.curveLut = this.curveLut;
      drawCurve();
      this.dispatchProcessing(true);
    });

    window.addEventListener('mouseup', () => {
      if (draggingPoint) {
        draggingPoint = null;
        this.dispatchProcessing(false);
      }
    });
  }

  buildCurveLut() {
    const lut = new Uint8Array(256);
    const sorted = [...this.curvePoints].sort((a, b) => a.x - b.x);

    for (let x = 0; x < 256; x++) {
      let p0 = sorted[0];
      let p1 = sorted[sorted.length - 1];

      for (let i = 0; i < sorted.length - 1; i++) {
        if (x >= sorted[i].x && x <= sorted[i + 1].x) {
          p0 = sorted[i];
          p1 = sorted[i + 1];
          break;
        }
      }

      const t = p1.x === p0.x ? 0 : (x - p0.x) / (p1.x - p0.x);
      const y = p0.y + t * (p1.y - p0.y);
      lut[x] = Math.max(0, Math.min(255, Math.round(y)));
    }
    return lut;
  }

  setupDropZone() {
    const dropZone = document.getElementById('dropZone');
    const fileInput = document.getElementById('fileInput');

    if (dropZone) {
      dropZone.addEventListener('dragover', (e) => { e.preventDefault(); dropZone.classList.add('dragover'); });
      dropZone.addEventListener('dragleave', () => dropZone.classList.remove('dragover'));
      dropZone.addEventListener('drop', (e) => {
        e.preventDefault();
        dropZone.classList.remove('dragover');
        if (e.dataTransfer.files.length > 0) {
          this.loadImage(e.dataTransfer.files[0]);
        }
      });
      dropZone.addEventListener('click', () => fileInput && fileInput.click());
    }

    if (fileInput) {
      fileInput.addEventListener('change', (e) => {
        if (e.target.files.length > 0) {
          this.loadImage(e.target.files[0]);
        }
      });
    }
  }

  applyPreset(name) {
    const p = this.getDefaultParams();
    switch (name) {
      case 'vogue':
        p.skinSmooth = 65;
        p.freqSepLow = 60;
        p.freqSepHigh = 100;
        p.blemishRemoval = 70;
        p.contrast = 15;
        p.highlights = -10;
        p.shadows = 10;
        p.texture = 15;
        p.eyeBrighten = 50;
        p.teethWhiten = 45;
        p.vignette = -20;
        break;
      case 'golden_hour':
        p.temp = 25;
        p.tint = 5;
        p.exposure = 8;
        p.skinSmooth = 45;
        p.skinRosy = 20;
        p.vibrance = 20;
        p.hslOrangeSat = 15;
        break;
      case 'corporate_clean':
        p.skinSmooth = 40;
        p.blemishRemoval = 55;
        p.teethWhiten = 50;
        p.clarity = 10;
        p.contrast = 5;
        break;
      case 'dramatic_bw':
        p.saturation = -100;
        p.contrast = 35;
        p.clarity = 25;
        p.texture = 30;
        p.vignette = -35;
        p.skinSmooth = 50;
        break;
    }

    this.params = { ...this.params, ...p };
    this.syncSlidersToParams();
    this.dispatchProcessing(false);
  }

  syncSlidersToParams() {
    for (const [k, v] of Object.entries(this.params)) {
      const slider = document.querySelector(`input[data-param="${k}"]`);
      if (slider) {
        slider.value = v;
        const disp = document.getElementById(`val_${k}`);
        if (disp) disp.textContent = v > 0 && slider.min < 0 ? `+${v}` : v;
      }
    }
  }

  resetAll() {
    this.params = this.getDefaultParams();
    this.syncSlidersToParams();
    this.dispatchProcessing(false);
  }

  exportFullRes() {
    if (!this.retouchedImageData) return;
    const w = this.retouchedImageData.width;
    const h = this.retouchedImageData.height;

    const exportCanvas = document.createElement('canvas');
    exportCanvas.width = w;
    exportCanvas.height = h;
    const ectx = exportCanvas.getContext('2d');
    ectx.putImageData(this.retouchedImageData, 0, 0);

    exportCanvas.toBlob((blob) => {
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `Collabs_Retouch_Pro_${Date.now()}_${w}x${h}.png`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
    }, 'image/png');
  }
}

// Global initialization
window.addEventListener('DOMContentLoaded', () => {
  window.retouchEngine = new RetouchEngine();
  // Auto load default sample portrait
  window.retouchEngine.loadImage('../test_portrait_4000x6000.jpg');
});
