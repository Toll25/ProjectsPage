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
      },
    },
    
  },
  variants: {},
  plugins: [],
};
