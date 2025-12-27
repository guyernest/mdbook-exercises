/**
 * mdbook-exercises JavaScript
 *
 * Interactive functionality for exercise blocks in mdBook.
 */

(function() {
    'use strict';

    // ============================================
    // Configuration
    // ============================================
    const PLAYGROUND_URL = 'https://play.rust-lang.org';
    const STORAGE_KEY = 'mdbook-exercises-progress';

    // ============================================
    // Utility Functions
    // ============================================

    /**
     * Get exercise progress from localStorage.
     */
    function getProgress() {
        try {
            const stored = localStorage.getItem(STORAGE_KEY);
            return stored ? JSON.parse(stored) : {};
        } catch (e) {
            console.warn('Failed to load exercise progress:', e);
            return {};
        }
    }

    /**
     * Save exercise progress to localStorage.
     */
    function saveProgress(progress) {
        try {
            localStorage.setItem(STORAGE_KEY, JSON.stringify(progress));
        } catch (e) {
            console.warn('Failed to save exercise progress:', e);
        }
    }

    /**
     * Get the exercise ID from a container element.
     */
    function getExerciseId(container) {
        const idEl = container.querySelector('.exercise-id');
        return idEl ? idEl.textContent.trim() : null;
    }

    /**
     * Show a temporary notification.
     */
    function showNotification(message, type) {
        const notification = document.createElement('div');
        notification.className = `exercise-notification ${type}`;
        notification.textContent = message;
        notification.style.cssText = `
            position: fixed;
            bottom: 20px;
            right: 20px;
            padding: 12px 20px;
            border-radius: 4px;
            font-size: 14px;
            z-index: 10000;
            animation: slideIn 0.3s ease;
        `;

        if (type === 'success') {
            notification.style.background = '#d4edda';
            notification.style.color = '#155724';
            notification.style.border = '1px solid #c3e6cb';
        } else if (type === 'error') {
            notification.style.background = '#f8d7da';
            notification.style.color = '#721c24';
            notification.style.border = '1px solid #f5c6cb';
        } else {
            notification.style.background = '#fff3cd';
            notification.style.color = '#856404';
            notification.style.border = '1px solid #ffeeba';
        }

        document.body.appendChild(notification);

        setTimeout(() => {
            notification.style.animation = 'slideOut 0.3s ease';
            setTimeout(() => notification.remove(), 300);
        }, 2000);
    }

    // ============================================
    // Copy Button
    // ============================================

    function initCopyButtons() {
        document.querySelectorAll('.copy-btn').forEach(button => {
            button.addEventListener('click', async function() {
                const container = this.closest('.exercise-starter');
                const codeEl = container.querySelector('.starter-code code, .starter-code textarea');

                if (!codeEl) return;

                const code = codeEl.tagName === 'TEXTAREA' ? codeEl.value : codeEl.textContent;

                try {
                    await navigator.clipboard.writeText(code);
                    const originalText = this.textContent;
                    this.textContent = 'Copied!';
                    setTimeout(() => {
                        this.textContent = originalText;
                    }, 1500);
                } catch (e) {
                    console.error('Failed to copy:', e);
                    showNotification('Failed to copy code', 'error');
                }
            });
        });
    }

    // ============================================
    // Reset Button
    // ============================================

    function initResetButtons() {
        document.querySelectorAll('.reset-btn').forEach(button => {
            button.addEventListener('click', function() {
                const container = this.closest('.exercise-starter');
                const textarea = container.querySelector('.starter-code textarea');
                const original = container.querySelector('.starter-code code');

                if (textarea && original) {
                    if (confirm('Reset code to original? Your changes will be lost.')) {
                        textarea.value = original.textContent;
                        showNotification('Code reset to original', 'success');
                    }
                } else if (original) {
                    showNotification('Code is already at original state', 'info');
                }
            });
        });
    }

    // ============================================
    // Hint Toggles
    // ============================================

    function initHintToggles() {
        document.querySelectorAll('.hint-toggle').forEach(toggle => {
            toggle.addEventListener('click', function() {
                const content = this.nextElementSibling;
                const isExpanded = this.getAttribute('aria-expanded') === 'true';

                this.setAttribute('aria-expanded', !isExpanded);
                content.classList.toggle('show');
            });
        });
    }

    // ============================================
    // Solution Toggles
    // ============================================

    function initSolutionToggles() {
        document.querySelectorAll('.solution-toggle').forEach(toggle => {
            toggle.addEventListener('click', function() {
                const content = this.nextElementSibling;

                if (!this.dataset.confirmed) {
                    if (!confirm('Are you sure you want to reveal the solution? Try the hints first!')) {
                        return;
                    }
                    this.dataset.confirmed = 'true';
                }

                const isExpanded = this.getAttribute('aria-expanded') === 'true';
                this.setAttribute('aria-expanded', !isExpanded);
                content.classList.toggle('show');
            });
        });
    }

    // ============================================
    // Progress Tracking
    // ============================================

    function initProgressTracking() {
        const progress = getProgress();

        document.querySelectorAll('.exercise-container').forEach(container => {
            const exerciseId = getExerciseId(container);
            if (!exerciseId) return;

            const checkbox = container.querySelector('.progress-indicator input[type="checkbox"]');
            if (!checkbox) return;

            // Restore saved state
            if (progress[exerciseId]) {
                checkbox.checked = true;
                checkbox.closest('.progress-indicator')?.classList.add('completed');
            }

            // Handle changes
            checkbox.addEventListener('change', function() {
                const currentProgress = getProgress();

                if (this.checked) {
                    currentProgress[exerciseId] = {
                        completed: true,
                        timestamp: new Date().toISOString()
                    };
                    this.closest('.progress-indicator')?.classList.add('completed');
                    showNotification('Exercise marked as complete!', 'success');
                } else {
                    delete currentProgress[exerciseId];
                    this.closest('.progress-indicator')?.classList.remove('completed');
                }

                saveProgress(currentProgress);
            });
        });
    }

    // ============================================
    // Run Tests (Rust Playground Integration)
    // ============================================

    function initRunTests() {
        document.querySelectorAll('.run-tests-btn').forEach(button => {
            button.addEventListener('click', async function() {
                const container = this.closest('.exercise-container');
                const starterContainer = container.querySelector('.exercise-starter');
                const testsContainer = container.querySelector('.exercise-tests');

                // Get the user's code
                let userCode = '';
                const textarea = starterContainer?.querySelector('textarea');
                const codeEl = starterContainer?.querySelector('.starter-code code');

                if (textarea) {
                    userCode = textarea.value;
                } else if (codeEl) {
                    userCode = codeEl.textContent;
                }

                // Get the test code
                const testCodeEl = testsContainer?.querySelector('.tests-code code');
                const testCode = testCodeEl ? testCodeEl.textContent : '';

                if (!userCode && !testCode) {
                    showNotification('No code to run', 'error');
                    return;
                }

                // Combine user code and test code
                const fullCode = combineCodeForTests(userCode, testCode);

                // Show loading state
                this.classList.add('loading');
                this.disabled = true;
                this.textContent = 'Running...';

                const resultsEl = testsContainer?.querySelector('.test-results');
                if (resultsEl) {
                    resultsEl.classList.remove('success', 'error', 'pending', 'show');
                    resultsEl.classList.add('pending', 'show');
                    resultsEl.textContent = 'Compiling and running tests...';
                }

                try {
                    const result = await runOnPlayground(fullCode);

                    if (resultsEl) {
                        resultsEl.classList.remove('pending');

                        if (result.success) {
                            resultsEl.classList.add('success');
                            resultsEl.textContent = result.output || 'All tests passed!';
                        } else {
                            resultsEl.classList.add('error');
                            resultsEl.textContent = result.output || result.error || 'Tests failed';
                        }
                    }

                    if (result.success) {
                        showNotification('Tests passed!', 'success');
                    } else {
                        showNotification('Tests failed', 'error');
                    }

                } catch (e) {
                    console.error('Failed to run tests:', e);
                    if (resultsEl) {
                        resultsEl.classList.remove('pending');
                        resultsEl.classList.add('error');
                        resultsEl.textContent = 'Failed to connect to Rust Playground: ' + e.message;
                    }
                    showNotification('Failed to run tests', 'error');
                } finally {
                    this.classList.remove('loading');
                    this.disabled = false;
                    this.textContent = 'Run Tests';
                }
            });
        });
    }

    /**
     * Combine user code and test code for Playground execution.
     */
    function combineCodeForTests(userCode, testCode) {
        // If the test code includes #[cfg(test)], we need to handle it specially
        // Otherwise, just combine them

        // Remove any existing main function from user code if tests have their own
        let combined = userCode;

        if (testCode) {
            // Check if user code has a main function
            const hasMain = /fn\s+main\s*\(/.test(userCode);
            const testHasMain = /fn\s+main\s*\(/.test(testCode);

            if (hasMain && testHasMain) {
                // Remove main from test code if user code has one
                combined = userCode + '\n\n' + testCode.replace(/fn\s+main\s*\([^)]*\)\s*\{[^}]*\}/g, '');
            } else {
                combined = userCode + '\n\n' + testCode;
            }
        }

        // Wrap in test harness if needed
        if (!combined.includes('fn main')) {
            combined = combined + '\n\nfn main() { /* test harness */ }';
        }

        return combined;
    }

    /**
     * Run code on the Rust Playground.
     */
    async function runOnPlayground(code) {
        const response = await fetch(PLAYGROUND_URL + '/execute', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                channel: 'stable',
                mode: 'debug',
                edition: '2021',
                crateType: 'bin',
                tests: true,
                code: code,
                backtrace: false
            })
        });

        if (!response.ok) {
            throw new Error(`Playground returned ${response.status}`);
        }

        const result = await response.json();

        return {
            success: result.success,
            output: result.stdout + result.stderr,
            error: result.stderr
        };
    }

    // ============================================
    // Editable Code (make code blocks editable)
    // ============================================

    function initEditableCode() {
        document.querySelectorAll('.exercise-starter[data-editable="true"]').forEach(container => {
            const codeBlock = container.querySelector('.starter-code pre');
            const codeEl = container.querySelector('.starter-code code');

            if (!codeBlock || !codeEl) return;

            // Create textarea for editing
            const textarea = document.createElement('textarea');
            textarea.value = codeEl.textContent;
            textarea.className = 'code-editor';
            textarea.spellcheck = false;

            // Store original code for reset
            container.dataset.originalCode = codeEl.textContent;

            // Replace code block with textarea
            codeBlock.style.display = 'none';
            codeBlock.insertAdjacentElement('afterend', textarea);

            // Auto-resize textarea
            textarea.addEventListener('input', function() {
                this.style.height = 'auto';
                this.style.height = this.scrollHeight + 'px';
            });

            // Initial resize
            textarea.style.height = textarea.scrollHeight + 'px';
        });
    }

    // ============================================
    // Keyboard Shortcuts
    // ============================================

    function initKeyboardShortcuts() {
        document.addEventListener('keydown', function(e) {
            // Ctrl/Cmd + Enter to run tests
            if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
                const activeElement = document.activeElement;
                if (activeElement && activeElement.closest('.exercise-container')) {
                    const container = activeElement.closest('.exercise-container');
                    const runBtn = container.querySelector('.run-tests-btn');
                    if (runBtn && !runBtn.disabled) {
                        e.preventDefault();
                        runBtn.click();
                    }
                }
            }
        });
    }

    // ============================================
    // Add CSS Animation Styles
    // ============================================

    function addAnimationStyles() {
        const style = document.createElement('style');
        style.textContent = `
            @keyframes slideIn {
                from {
                    transform: translateX(100%);
                    opacity: 0;
                }
                to {
                    transform: translateX(0);
                    opacity: 1;
                }
            }
            @keyframes slideOut {
                from {
                    transform: translateX(0);
                    opacity: 1;
                }
                to {
                    transform: translateX(100%);
                    opacity: 0;
                }
            }
        `;
        document.head.appendChild(style);
    }

    // ============================================
    // Initialize
    // ============================================

    function init() {
        addAnimationStyles();
        initCopyButtons();
        initResetButtons();
        initHintToggles();
        initSolutionToggles();
        initProgressTracking();
        initRunTests();
        initEditableCode();
        initKeyboardShortcuts();

        console.log('mdbook-exercises initialized');
    }

    // Run on DOM ready
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }

    // Export for testing
    if (typeof window !== 'undefined') {
        window.mdbookExercises = {
            getProgress,
            saveProgress,
            runOnPlayground
        };
    }
})();
