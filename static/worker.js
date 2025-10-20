// Web Worker for running MicroPython

let mp = null;

// Initialize MicroPython
async function initMicroPython() {
    if (mp) return;

    try {
        // Import MicroPython module
        const baseUrl = self.location.origin;
        const { loadMicroPython } = await import(baseUrl + '/static/micropython.mjs');

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

    try {
        // Execute the code using MicroPython's runPython method
        mp.runPython(code);

        self.postMessage({
            type: 'done'
        });
    } catch (error) {
        // Extract error message - MicroPython throws PythonError objects
        const errorMsg = error.message || String(error);
        self.postMessage({
            type: 'error',
            text: errorMsg
        });
    }
}

// Handle messages from main thread
self.onmessage = async function(e) {
    const message = e.data;

    if (message.type === 'run') {
        await runPythonCode(message.code);
    } else if (message.type === 'stop') {
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
