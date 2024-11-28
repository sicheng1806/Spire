/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./components/**/*.{rs,html}","./src/**/*.{rs,html}","./dist/**/*.{js,html}"],
  theme: {
    extend: {},
  },
  plugins: [require('daisyui'),],
}
