import type { Config } from 'tailwindcss';

const config: Config = {
  content: [
    './src/**/*.{ts,tsx}',
    './app/**/*.{ts,tsx}',
    './components/**/*.{ts,tsx}',
  ],
  theme: {
    extend: {
      colors: {
        'ocean-dark': '#0a1e2d',
        'ocean-light': '#9ed0e6',
        'deep-blue': '#001f3f',
        'sonar-green': '#00ffcc',
      },
      fontFamily: {
        display: ['"Unica One"', 'sans-serif'],
      },
    },
  },
  plugins: [],
};
export default config;

