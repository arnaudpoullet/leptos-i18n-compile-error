/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./app/src/**/*.rs"],
    theme: {
        extend: {
            colors: {
                'main-color': "#5E9B8C",
                'light-color': "#D4E4DE",
                'base-color': "#4C4C4E",
                'accent-color': "#F1C232",
            },
            aria: {
                current: 'current="page"',
            },
            screens: {
                'xs': '480px',
            },
        },
    },
    plugins: [
        require('@tailwindcss/forms'),
    ],
}
