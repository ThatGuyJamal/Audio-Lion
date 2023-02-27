/** @type {import('tailwindcss').Config} */
module.exports = {
	darkMode: "class",
	content: ["./src/**/*.{html,js,svelte,ts}"],
	theme: {
		screens: {
			sm: "480px",
			md: "768px",
			lg: "976px",
			xl: "1440px",
		},
		colors: {
			blue: "#1fb6ff",
			purple: "#7e5bef",
			pink: "#ff49db",
			orange: "#ff7849",
			green: "#13ce66",
			yellow: "#ffc82c",
			gray: "#8492a6",
			red: "#ff4949",
		},
		fontFamily: {},
		extend: {},
	},
	plugins: [require("daisyui")],
	/** see https://daisyui.com/docs/config/ */
	daisyui: {
		styled: true,
		themes: true,
		base: true,
		utils: true,
		logs: true,
		rtl: false,
		prefix: "",
		darkTheme: "dark",
	},
};
