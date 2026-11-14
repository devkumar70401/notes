document$.subscribe(() => {
  const renderMath = () => {
    if (typeof renderMathInElement !== "undefined") {
      renderMathInElement(document.body, {
        delimiters: [
          { left: "$$", right: "$$", display: true },
          { left: "$", right: "$", display: false },
          { left: "\\(", right: "\\)", display: false },
          { left: "\\[", right: "\\]", display: true }
        ],
        ignoredTags: ["script", "noscript", "style", "textarea", "pre", "code"],
        throwOnError: false
      });
    }
  };

  if (typeof renderMathInElement !== "undefined") {
    renderMath();
  } else {
    window.addEventListener("load", renderMath);
  }
});
