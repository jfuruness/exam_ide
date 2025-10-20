// Web Worker for running MicroPython with timeout

let mp = null;
let executionTimeout = null;
let shouldStop = false;
const TIMEOUT_MS = 30000; // 30 second timeout

// Initialize MicroPython
async function initMicroPython() {
    if (mp) return;

    try {
        // Import MicroPython module
        const baseUrl = self.location.origin;
        const { loadMicroPython } = await import(baseUrl + '/static/micropython.mjs');

        // Capture stdout/stderr
        const outputBuffer = [];

        mp = await loadMicroPython({
            stdout: (line) => {
                self.postMessage({
                    type: 'output',
                    text: line + '\n'
                });
            },
            stderr: (line) => {
                self.postMessage({
                    type: 'output',
                    text: 'Error: ' + line + '\n'
                });
            }
        });

        self.postMessage({
            type: 'output',
            text: 'Python interpreter ready.\n'
        });
    } catch (error) {
        self.postMessage({
            type: 'error',
            text: 'Failed to initialize Python: ' + error.message
        });
    }
}

// Execute Python code
async function runPythonCode(code) {
    if (!mp) {
        await initMicroPython();
    }

    if (!mp) {
        self.postMessage({
            type: 'error',
            text: 'Python interpreter not initialized'
        });
        return;
    }

    shouldStop = false;

    // Set up timeout
    executionTimeout = setTimeout(() => {
        shouldStop = true;
        self.postMessage({
            type: 'timeout'
        });
        self.close();
    }, TIMEOUT_MS);

    try {
        // Execute the code using MicroPython's runPython method
        mp.runPython(code);

        clearTimeout(executionTimeout);

        if (!shouldStop) {
            self.postMessage({
                type: 'done'
            });
        }
    } catch (error) {
        clearTimeout(executionTimeout);

        if (!shouldStop) {
            // Extract error message - MicroPython throws PythonError objects
            const errorMsg = error.message || String(error);
            self.postMessage({
                type: 'error',
                text: errorMsg
            });
        }
    }
}

// Handle messages from main thread
self.onmessage = async function(e) {
    const message = e.data;

    if (message.type === 'run') {
        await runPythonCode(message.code);
    } else if (message.type === 'stop') {
        shouldStop = true;
        clearTimeout(executionTimeout);
        self.postMessage({
            type: 'error',
            text: 'Execution stopped by user\n'
        });
        self.close();
    }
};

// Initialize on worker start
initMicroPython().catch(error => {
    self.postMessage({
        type: 'error',
        text: 'Worker initialization failed: ' + error.message
    });
});
