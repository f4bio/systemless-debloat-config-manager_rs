module.exports = (ctx) => ({
  ident: "posthtml",
  parser: "PostHTML Parser",
  plugins: [
    /* PostHTML Plugins */
    require("posthtml-plugin")(options)
  ]
});
