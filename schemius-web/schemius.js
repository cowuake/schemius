import init, { evaluate } from "./pkg/schemius_web.js";

class Schemius {
  static terminal = null;
  static xStart = null;
  static yStart = null;

  static get prompt() {
    return "λ> ";
  }

  static get welcomeMessage() {
    return `
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
  }

  static get keymap() {
    return `
        Keymap:
          [arrow keys | swipe]    -> Move cursor | Navigate history
          [Ctrl + F / B / J / P]  -> Move cursor | Navigate history
          [Shift + Enter]         -> Enter multiline insert mode
          [Ctrl + H]              -> Show help message
          [Ctrl + K]              -> Show keymap
          [Ctrl + Shift + F]      -> Switch font
          [Ctrl + Shift + T]      -> Switch color theme
    `;
  }

  static get fonts() {
    return [
      "Source Code Pro",
      "Cascadia Code",
      "Fira Code",
      "JetBrains Mono",
      "Consolas",
      "monospace",
    ];
  }

  static defaultFont = localStorage.getItem("font") ?? Schemius.fonts[0];
  static defaultTheme = JSON.parse(localStorage.getItem("theme")) ?? Schemius.themes[0];

  static get matchingChars() {
    return {
      "(": ")",
      "[": "]",
      "{": "}",
      '"': '"',
    };
  }

  static getFont() {
    return document.documentElement.style.getPropertyValue("--font");
  }

  static setFont(font) {
    document.documentElement.style.setProperty("--font", font);
    localStorage.setItem("font", font);
  }

  static async switchFont() {
    let currentFont = Schemius.getFont();
    let keepSearching = true;
    let nVisited = 0;

    do {
      currentFont =
        Schemius.fonts[(Schemius.fonts.indexOf(currentFont) + 1) % Schemius.fonts.length];
      let fontFaces = await document.fonts.load(`12pt ${currentFont}`);
      keepSearching = fontFaces.length === 0 && currentFont !== "monospace";
    } while (keepSearching && nVisited++ < Schemius.fonts.length);

    console.log("Setting font to", currentFont);
    Schemius.setFont(currentFont);
  }

  static get themes() {
    return [
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
  }

  static getTheme() {
    const color = document.documentElement.style.getPropertyValue("--color");
    const background = document.documentElement.style.getPropertyValue("--background");
    const linkColor = document.documentElement.style.getPropertyValue("--link-color");

    return {
      color: color,
      background: background,
      linkColor: linkColor,
    };
  }

  static setTheme(theme) {
    document.documentElement.style.setProperty("--color", theme.color);
    document.documentElement.style.setProperty("--background", theme.background);
    document.documentElement.style.setProperty("--link-color", theme.linkColor);
    localStorage.setItem("theme", JSON.stringify(theme));
  }

  static switchTheme() {
    let currentTheme = Schemius.getTheme();
    console.log("Previous theme", currentTheme);
    let index = Schemius.themes.findIndex(
      (theme) => JSON.stringify(theme) === JSON.stringify(currentTheme)
    );
    index = ++index % Schemius.themes.length;
    currentTheme = Schemius.themes[index];
    console.log("Setting theme to", currentTheme);
    Schemius.setTheme(currentTheme);
  }

  static isMobile() {
    return Schemius.terminal.hasClass("terminal-mobile");
  }

  static checkMobile() {
    alert(
      Schemius.isMobile()
        ? "You seem to be on a mobile device"
        : "You don't seem to be on a mobile device"
    );
  }

  static fakeProcedures = {
    "(mobile?)": Schemius.checkMobile,
    "(switch-font)": Schemius.switchFont,
    "(switch-theme)": Schemius.switchTheme,
  };

  static handleTouchStart(event) {
    const touch = (event.touches || event.originalEvent.touches)[0];
    Schemius.xStart = touch.clientX;
    Schemius.yStart = touch.clientY;
    return false;
  }

  static dispatchKeyEvent(key) {
    Schemius.terminal.trigger($.Event("keydown", { key: key }));
  }

  static handleTouchMove(event) {
    if (!Schemius.xStart || !Schemius.yStart) {
      return;
    }

    const touch = (event.touches || event.originalEvent.touches)[0];
    const xDelta = touch.clientX - Schemius.xStart;
    const yDelta = touch.clientY - Schemius.yStart;

    const key =
      Math.abs(xDelta) > Math.abs(yDelta)
        ? xDelta > 0
          ? "ArrowRight"
          : "ArrowLeft"
        : yDelta > 0
        ? "ArrowDown"
        : "ArrowUp";

    // Schemius.xStart = null;
    // Schemius.yStart = null;

    Schemius.dispatchKeyEvent(key);
    return false;
  }

  static matchChar(opening) {
    const closing = Schemius.matchingChars[opening];
    Schemius.terminal.insert(opening);
    Schemius.terminal.insert(closing);
    Schemius.dispatchKeyEvent("ArrowLeft");
  }

  static handleDelete() {
    const position = Schemius.terminal.get_position();
    const char = Schemius.terminal.cmd().get()[position - 1];

    if (
      Schemius.matchingChars[char] &&
      Schemius.terminal.cmd().get()[position] === Schemius.matchingChars[char]
    ) {
      Schemius.terminal.cmd().delete(1);
    }
  }

  static handleKeyDown(e) {
    if (e.ctrlKey) {
      if (e.key !== "V") {
        e.preventDefault();
      }
      if (e.shiftKey) {
        switch (e.key) {
          case "F": // Ctrl + Shift + F
            Schemius.switchFont();
            return false;
          case "T": // Ctrl + Shift + T
            Schemius.switchTheme();
            return false;
        }
      } else {
        switch (e.key) {
          case "F": // Ctrl + F
            Schemius.dispatchKeyEvent("ArrowRight");
            return false;
          case "B": // Ctrl + B
            Schemius.dispatchKeyEvent("ArrowLeft");
            return false;
          case "J": // Ctrl + J
            Schemius.dispatchKeyEvent("ArrowDown");
            return false;
          case "P": // Ctrl + P
            Schemius.dispatchKeyEvent("ArrowUp");
            return false;
          case "G": // Ctrl + G
            window.getSelection().removeAllRanges();
            return false;
          case "H": // Ctrl + H
            Schemius.terminal.echo(Schemius.welcomeMessage);
            return false;
          case "K": // Ctrl + K
            Schemius.terminal.echo(Schemius.keymap);
            return false;
        }
      }
    } else if (e.key in Schemius.matchingChars) {
      Schemius.matchChar(e.key);
      return false;
    } else if (e.key === "BACKSPACE") {
      Schemius.handleDelete();
    } else if (e.isComposing || e.keyCode === 229) {
      // Handle keydown events during IME composition
      if (Schemius.isMobile()) {
        return false;
      }
    }
  }

  static initTerminal() {
    Schemius.setFont(Schemius.defaultFont);
    Schemius.setTheme(Schemius.defaultTheme);

    Schemius.terminal = $("body").terminal(
      function (expression) {
        expression = expression.replace(/\r?\n|\r/g, " ").trim();
        if (expression) {
          if (Schemius.fakeProcedures[expression]) {
            this.echo(Schemius.fakeProcedures[expression]());
          } else {
            try {
              this.echo(evaluate(expression));
            } catch (e) {
              this.echo("Ooops... Something went wrong! :(");
              this.read("Press [Enter] to reload\n\t").then(() => {
                location.reload();
              });
            }
          }
        }
      },
      {
        greetings: Schemius.welcomeMessage,
        keydown: Schemius.handleKeyDown,
        prompt: Schemius.prompt,
      }
    );

    $(document)
      .on("touchstart", Schemius.terminal, Schemius.handleTouchStart)
      .on("touchmove", Schemius.terminal, Schemius.handleTouchMove);
  }
}

init().then(() => {
  Schemius.initTerminal();
});