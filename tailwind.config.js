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
      sans: ["system-ui", "sans-serif"],
      //Charter,Bitstream Charter,Sitka Text,Cambria,serif;
      serif: ["Charter", "Bitstream Charter", "Sitka Text", "Cambria", "serif"],
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
    require("./aria-plugin"),
    theme.create(),
  ],
};
