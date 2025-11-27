/** @type {import('tailwindcss').Config} */
module.exports = {
	content: [
		'./src/**/*.rs',
		'./src/**/*.html',
		'./src/**/*.css',
		'./index.html',
	],
	theme: {
		extend: {
			fontFamily: {
				'geist-sans': ['Geist', 'system-ui', 'sans-serif'],
				'geist-mono': ['Geist Mono', 'monospace'],
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

