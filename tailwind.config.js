module.exports = {
  content: [
    "./src/**/*.{rs,html}",
    "./index.html",
    "./tailwind.css",
  ],
  theme: {
    extend: {
      backgroundImage: {
        'flower': "url('./img/flower.jpg')",
        'github': "url('img/github.svg')",
      },
      aspectRatio: {
        '2/1': '2 / 1',
      },
      colors: {
        background: '#161617',
        transperent: 'transparent',
        github_purple: '#6e5494',
        text: '#ccc',
        background_brighter: '#1b1b1e'
      },
    },
    
  },
  variants: {},
  plugins: [],
};
