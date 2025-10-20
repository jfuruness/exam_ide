// CodeMirror bundle - lightweight editor with syntax highlighting only
import { EditorView, keymap, lineNumbers, highlightActiveLine, highlightActiveLineGutter } from '@codemirror/view';
import { EditorState, EditorSelection } from '@codemirror/state';
import { python } from '@codemirror/lang-python';
import { defaultKeymap, indentWithTab, history, historyKeymap } from '@codemirror/commands';
import { syntaxHighlighting, defaultHighlightStyle, bracketMatching, indentUnit } from '@codemirror/language';

// Create a minimal setup without autocomplete
const minimalSetup = [
    lineNumbers(),
    highlightActiveLineGutter(),
    highlightActiveLine(),
    history(),
    bracketMatching(),
    syntaxHighlighting(defaultHighlightStyle),
    indentUnit.of("    "), // 4 spaces for indentation
    keymap.of([
        ...defaultKeymap,
        ...historyKeymap,
        indentWithTab
    ]),
    python(),
    EditorView.lineWrapping,
    EditorView.theme({
        "&": {
            height: "100%",
            fontSize: "14px"
        },
        ".cm-scroller": {
            overflow: "auto",
            fontFamily: "'Monaco', 'Menlo', 'Ubuntu Mono', 'Consolas', 'monospace'"
        },
        ".cm-content": {
            padding: "10px 0"
        },
        ".cm-gutters": {
            backgroundColor: "#f7f7f7",
            color: "#999",
            border: "none"
        },
        ".cm-activeLineGutter": {
            backgroundColor: "#e8f2ff"
        },
        ".cm-activeLine": {
            backgroundColor: "#e8f2ff40"
        }
    })
];

window.CodeMirrorSetup = {
    createEditor(parent, initialContent, onChange) {
        const startState = EditorState.create({
            doc: initialContent,
            extensions: [
                ...minimalSetup,
                EditorView.updateListener.of((update) => {
                    if (update.docChanged) {
                        onChange(update.state.doc.toString());
                    }
                })
            ]
        });

        const view = new EditorView({
            state: startState,
            parent: parent
        });

        return {
            view,
            getValue: () => view.state.doc.toString(),
            setValue: (content) => {
                view.dispatch({
                    changes: {
                        from: 0,
                        to: view.state.doc.length,
                        insert: content
                    }
                });
            },
            destroy: () => view.destroy()
        };
    }
};
