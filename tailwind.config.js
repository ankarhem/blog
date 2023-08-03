const colors = require("tailwindcss/colors");
const { Theme } = require("tailwind-easy-theme");

const theme = new Theme({
  stone: colors.stone,
});

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    fontFamily: {
      inter: ["Inter", "sans-serif"],
      iosevka: ["Iosevka", "monospace"],
    },
    extend: {
      typography: (theme) => ({
        DEFAULT: {
          css: {
            a: {
              "text-decoration-color": theme("colors.stone.400"),
              "&:hover": {
                "text-decoration-color": theme("colors.stone.900"),
              },
            },
          },
        },
      }),
    },
  },
  plugins: [
    require("@tailwindcss/typography"),
    require("@tailwindcss/forms"),
    theme.create(),
  ],
};
