/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {},
  },
  daisyui: {
    themes: false,
    darkTheme: false,
  },
  plugins: [require("daisyui")],
};
