// Collabs Retouch Pro — Native Desktop Application Main Process
// Cross-Platform: Windows, macOS, Linux
// Created by Dave Groennesby (Collabs Software)

const { app, BrowserWindow, Menu, dialog, shell } = require('electron');
const path = require('path');
const http = require('http');
const fs = require('fs');

let splashWindow = null;
let mainWindow = null;
let localServer = null;
let serverPort = 8484;

// Determine platform-specific app icon
function getAppIcon() {
  if (process.platform === 'win32') {
    return path.join(__dirname, 'icon.ico');
  } else if (process.platform === 'darwin') {
    return path.join(__dirname, 'icon.icns');
  }
  return path.join(__dirname, 'icon.png');
}

// Start local HTTP server to host app with full OffscreenCanvas / Worker support
function startLocalServer() {
  return new Promise((resolve) => {
    const mimeTypes = {
      '.html': 'text/html',
      '.js': 'application/javascript',
      '.css': 'text/css',
      '.png': 'image/png',
      '.jpg': 'image/jpeg',
      '.jpeg': 'image/jpeg',
      '.svg': 'image/svg+xml',
      '.ico': 'image/x-icon',
      '.wasm': 'application/wasm'
    };

    localServer = http.createServer((req, res) => {
      let parsedUrl = req.url.split('?')[0];
      if (parsedUrl === '/') parsedUrl = '/app/index.html';

      let filePath = path.join(__dirname, parsedUrl);

      // Security check
      if (!filePath.startsWith(__dirname)) {
        res.writeHead(403);
        res.end('Forbidden');
        return;
      }

      fs.stat(filePath, (err, stats) => {
        if (err || !stats.isFile()) {
          res.writeHead(404);
          res.end('Not Found');
          return;
        }

        const ext = path.extname(filePath).toLowerCase();
        const contentType = mimeTypes[ext] || 'application/octet-stream';

        res.writeHead(200, {
          'Content-Type': contentType,
          'Cross-Origin-Opener-Policy': 'same-origin',
          'Cross-Origin-Embedder-Policy': 'require-corp'
        });

        fs.createReadStream(filePath).pipe(res);
      });
    });

    localServer.listen(0, '127.0.0.1', () => {
      serverPort = localServer.address().port;
      resolve(serverPort);
    });
  });
}

function createSplashWindow() {
  splashWindow = new BrowserWindow({
    width: 800,
    height: 450,
    resizable: false,
    frame: false,
    center: true,
    alwaysOnTop: true,
    show: true,
    icon: getAppIcon(),
    backgroundColor: '#0E0E10',
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true
    }
  });

  splashWindow.loadFile(path.join(__dirname, 'app', 'splash.html'));
  splashWindow.on('closed', () => {
    splashWindow = null;
  });
}

function createMainWindow(port) {
  mainWindow = new BrowserWindow({
    width: 1540,
    height: 980,
    minWidth: 1100,
    minHeight: 700,
    center: true,
    show: false,
    icon: getAppIcon(),
    title: 'Collabs Retouch Pro',
    backgroundColor: '#0E0E10',
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      webSecurity: true
    }
  });

  buildMenu();

  mainWindow.loadURL(`http://127.0.0.1:${port}/app/index.html`);

  mainWindow.once('ready-to-show', () => {
    // Show splash for 3.2 seconds then fade in main studio
    setTimeout(() => {
      if (splashWindow) {
        splashWindow.close();
      }
      mainWindow.show();
      mainWindow.focus();
    }, 3200);
  });

  mainWindow.on('closed', () => {
    mainWindow = null;
  });
}

function buildMenu() {
  const isMac = process.platform === 'darwin';
  const template = [
    ...(isMac ? [{
      label: app.name,
      submenu: [
        { role: 'about', label: 'About Collabs Retouch Pro' },
        { type: 'separator' },
        { role: 'services' },
        { type: 'separator' },
        { role: 'hide' },
        { role: 'hideOthers' },
        { role: 'unhide' },
        { type: 'separator' },
        { role: 'quit' }
      ]
    }] : []),
    {
      label: 'File',
      submenu: [
        {
          label: 'Open Portrait Photo...',
          accelerator: 'CmdOrCtrl+O',
          click: async () => {
            if (!mainWindow) return;
            const res = await dialog.showOpenDialog(mainWindow, {
              title: 'Open Portrait Photo',
              filters: [
                { name: 'Images', extensions: ['jpg', 'jpeg', 'png', 'webp', 'tiff', 'dng', 'cr2', 'cr3', 'nef', 'arw'] }
              ],
              properties: ['openFile']
            });
            if (!res.canceled && res.filePaths.length > 0) {
              const fileUrl = `file://${res.filePaths[0]}`;
              mainWindow.webContents.executeJavaScript(`window.retouchEngine && window.retouchEngine.loadImage('${fileUrl}')`);
            }
          }
        },
        {
          label: 'Export 4K Full-Res...',
          accelerator: 'CmdOrCtrl+S',
          click: () => {
            if (mainWindow) {
              mainWindow.webContents.executeJavaScript(`window.retouchEngine && window.retouchEngine.exportFullRes()`);
            }
          }
        },
        { type: 'separator' },
        {
          label: 'Reset All Sliders',
          accelerator: 'CmdOrCtrl+R',
          click: () => {
            if (mainWindow) {
              mainWindow.webContents.executeJavaScript(`window.retouchEngine && window.retouchEngine.resetAll()`);
            }
          }
        },
        { type: 'separator' },
        isMac ? { role: 'close' } : { role: 'quit' }
      ]
    },
    {
      label: 'View',
      submenu: [
        {
          label: 'Fit to Window',
          accelerator: 'CmdOrCtrl+0',
          click: () => mainWindow && mainWindow.webContents.executeJavaScript(`window.retouchEngine && window.retouchEngine.fitToScreen()`)
        },
        {
          label: '100% 1:1 Pixel Zoom',
          accelerator: 'CmdOrCtrl+1',
          click: () => mainWindow && mainWindow.webContents.executeJavaScript(`window.retouchEngine && window.retouchEngine.setZoom100()`)
        },
        { type: 'separator' },
        { role: 'togglefullscreen' },
        { role: 'toggleDevTools' }
      ]
    },
    {
      label: 'Help',
      submenu: [
        {
          label: 'Collabs Models Online',
          click: () => shell.openExternal('https://www.collabs-models.com')
        },
        {
          label: 'About Collabs Retouch Pro',
          click: () => {
            dialog.showMessageBox({
              type: 'info',
              title: 'Collabs Retouch Pro',
              message: 'Collabs Retouch Pro Studio v1.0.0',
              detail: 'Fused Lightcraft RAW Develop + Evoto AI Portrait Retouch Suite.\n100% Real Pixel Math — Zero Fake CSS Filters.\nCreated by Dave Groennesby (Collabs Software).',
              icon: getAppIcon()
            });
          }
        }
      ]
    }
  ];

  const menu = Menu.buildFromTemplate(template);
  Menu.setApplicationMenu(menu);
}

app.whenReady().then(async () => {
  createSplashWindow();
  const port = await startLocalServer();
  createMainWindow(port);

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createMainWindow(port);
    }
  });
});

app.on('window-all-closed', () => {
  if (localServer) localServer.close();
  if (process.platform !== 'darwin') {
    app.quit();
  }
});