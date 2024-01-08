/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        wooden: "#8B4513",
      },
      fontFamily: {
        mono: ["monospace"], // Replace with your font stack
      },
      letterSpacing: {
        custom: ".3rem", // Custom letter spacing
      },
    },
  },
  plugins: [],
};
