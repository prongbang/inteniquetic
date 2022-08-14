import type { Config } from "tailwindcss";

const config: Config = {
  darkMode: "class",
  content: ["./app.html", "./src/**/*.{svelte,js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {},
    },
  },
  plugins: [],
};

export default config;
