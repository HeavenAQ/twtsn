/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs", "./cases/**/*.md"],
  theme: {
    extend: {},
  },
  daisyui: {
    themes: false,
    darkTheme: false,
  },
  plugins: [require("daisyui"), require("@tailwindcss/typography")],
};
