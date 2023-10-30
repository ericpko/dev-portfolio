const { fontFamily } = require("tailwindcss/defaultTheme");

/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: ["./templates/*.html"],
  theme: {
    container: {
      center: true,
    },
    extend: {
      fontFamily: {
        serif: ["Merriweather", ...fontFamily.serif],
        sans: ["Inter var", ...fontFamily.sans],
      },
    },
  },
  plugins: [],
};
