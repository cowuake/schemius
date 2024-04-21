var terminal = null;

const welcomeMessage = `
    ███████╗ ██████╗██╗  ██╗███████╗███╗   ███╗██╗██╗   ██╗███████╗
    ██╔════╝██╔════╝██║  ██║██╔════╝████╗ ████║██║██║   ██║██╔════╝
    ███████╗██║     ███████║█████╗  ██╔████╔██║██║██║   ██║███████╗
    ╚════██║██║     ██╔══██║██╔══╝  ██║╚██╔╝██║██║██║   ██║╚════██║
    ███████║╚██████╗██║  ██║███████╗██║ ╚═╝ ██║██║╚██████╔╝███████║
    ╚══════╝ ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝     ╚═╝╚═╝ ╚═════╝ ╚══════╝

      Welcome to Schemius!
        Press [Ctrl + H]        to show this welcome message
        Press [Ctrl + K]        to show essential keymap
        (environment-bindings)  -> Show bindings in current env
        (fact 2000)             -> If you'd like to see a big number :)

      Go through the code at https://github.com/cowuake/schemius
`;

const keymap = `
      Keymap:
        [arrow keys | swipe]    -> Move cursor | Navigate history
        [Shift + Enter]         -> Enter multiline insert mode
        [Ctrl + H]              -> Show help message
        [Ctrl + K]              -> Show keymap
        [Ctrl + Shift + F]      -> Switch font
        [Ctrl + Shift + T]      -> Switch color theme
`;

const fonts = [
  "Source Code Pro",
  "Cascadia Code",
  "Fira Code",
  "JetBrains Mono",
  "Consolas",
  "monospace",
];
const localFont = localStorage.getItem("font");
const defaultFont = localFont ?? fonts[0];

const matchingChars = {
  "(": ")",
  "[": "]",
  "{": "}",
  '"': '"',
  "'": "'",
};

function getFont() {
  return document.documentElement.style.getPropertyValue("--font");
}

function setFont(font) {
  document.documentElement.style.setProperty("--font", font);
  localStorage.setItem("font", font);
}

async function switchFont() {
  let currentFont = getFont();
  let keepSearching = true;
  let nVisited = 0;

  do {
    currentFont = fonts[(fonts.indexOf(currentFont) + 1) % fonts.length];
    let fontFaces = await document.fonts.load(`12pt ${currentFont}`);
    keepSearching = fontFaces.length === 0 && currentFont !== "monospace";
  } while (keepSearching && nVisited++ < fonts.length);

  console.log("Setting font to", currentFont);
  setFont(currentFont);
}

const themes = [
  {
    // gruvbox dark
    color: "#ebdbb2",
    background: "#32302f",
    linkColor: "#b8bb26",
  },
  {
    // everforest light
    color: "#5C6a72",
    background: "#fdf6e3",
    linkColor: "#88C0D0",
  },
  {
    // dracula
    color: "#f8f8f2",
    background: "#282a36",
    linkColor: "#ff79c6",
  },
  {
    // gruvbox light
    color: "#504945",
    background: "#f2e5bc",
    linkColor: "#689d6a",
  },
];
const localTheme = JSON.parse(localStorage.getItem("theme"));
const defaultTheme = localTheme ?? themes[0];

function getTheme() {
  const color = document.documentElement.style.getPropertyValue("--color");
  const background =
    document.documentElement.style.getPropertyValue("--background");
  const linkColor =
    document.documentElement.style.getPropertyValue("--link-color");

  return {
    color: color,
    background: background,
    linkColor: linkColor,
  };
}

function setTheme(theme) {
  document.documentElement.style.setProperty("--color", theme.color);
  document.documentElement.style.setProperty("--background", theme.background);
  document.documentElement.style.setProperty("--link-color", theme.linkColor);
  localStorage.setItem("theme", JSON.stringify(theme));
}

function switchTheme() {
  let currentTheme = getTheme();
  console.log("Previous theme", currentTheme);
  let index = themes.indexOf(
    themes.filter(
      (theme) => JSON.stringify(theme) == JSON.stringify(currentTheme)
    )[0]
  );
  index = ++index % themes.length;
  currentTheme = themes[index];
  console.log("Setting theme to", currentTheme);
  setTheme(currentTheme);
}

const fakeProcedures = {
  "(switch-font)": switchFont,
  "(switch-theme)": switchTheme,
};

var xStart = null;
var yStart = null;

function handleTouchStart(event) {
  const touch = (event.touches || event.originalEvent.touches)[0];
  xStart = touch.clientX;
  yStart = touch.clientY;
  return false;
}

function dispatchKeyEvent(key) {
  $(".terminal").trigger($.Event("keydown", { key: key }));
}

function handleTouchMove(event) {
  if (!xStart || !yStart) {
    return;
  }

  const touch = (event.touches || event.originalEvent.touches)[0];
  var xDelta = touch.clientX - xStart;
  var yDelta = touch.clientY - yStart;

  const key =
    Math.abs(xDelta) > Math.abs(yDelta)
      ? xDelta > 0
        ? "ArrowRight"
        : "ArrowLeft"
      : yDelta > 0
      ? "ArrowDown"
      : "ArrowUp";

  dispatchKeyEvent(key);

  xStart = null;
  yStart = null;
  return false;
}

function matchChar(opening) {
  const closing = matchingChars[opening];
  terminal.insert(opening);
  terminal.insert(closing);
  dispatchKeyEvent("ArrowLeft");
}

function handleDelete() {
  const position = terminal.get_position();
  const char = terminal.cmd().get()[position - 1];

  if (
    matchingChars[char] &&
    terminal.cmd().get()[position] === matchingChars[char]
  ) {
    terminal.cmd().delete(1);
  }
}
