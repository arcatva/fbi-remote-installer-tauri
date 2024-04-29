const {withMaterialColors} = require('tailwind-material-colors');

/** @type {import('tailwindcss').Config} */

const config = {
	content: [
		"./index.html",
		"./src/**/*.{js,ts,jsx,tsx}",
	],
	theme: {
		extend: {},
	},
	plugins: [],
};


module.exports = withMaterialColors(config, {
// Your base colors as HEX values. 'primary' is required.
		primary: '#A1DEF2',
	},
	{
		scheme: 'neutral',
		contrast: 0,
	}
);


