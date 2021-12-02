const fs = require('fs');
const cheerio = require("cheerio");
const http = require('http');

let content = fs.readFileSync('index.html');
const $ = cheerio.load(content.toString());

$('body > script').map((i, script) => {
  console.log(script.attribs)
})
