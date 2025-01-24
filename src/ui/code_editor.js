const monacoFolder = await dioxus.recv();
const script = document.createElement("script");
script.src = `${monacoFolder}/loader.js`;

script.onload = () => {
  require.config({ paths: { vs: monacoFolder } });
  require(["vs/editor/editor.main"], async () => {
    const editor = monaco.editor.create(document.querySelector(".monaco"), {
      automaticLayout: true,
      language: "javascript",
      theme: "vs-dark",
    });

    // TODO: Remove
    console.log(editor);

    editor.addAction({
      contextMenuGroupId: "custom",
      contextMenuOrder: 0,
      id: "save",
      label: "Save",
      keybindings: [monaco.KeyMod.CtrlCmd | monaco.KeyCode.F11],
      run: () => {
        dioxus.send(editor.getValue());
      },
    });

    const text = await dioxus.recv();
    editor.setValue(text);
  });
};

document.body.append(script);
