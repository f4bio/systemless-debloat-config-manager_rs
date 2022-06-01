module.exports = (ctx) => ({
  parser: ctx.ext === ".sml" ? "posthtml-sugarml" : false,
  from: ctx.from,
  to: ctx.to,
  plugins: {
    "posthtml-include": {},
    "posthtml-expressions": { locals: ctx.locals },
    htmlnano: ctx.env === "production" ? {} : false
  }
});
