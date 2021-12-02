const fs = require('fs');
const cheerio = require("cheerio");
const http = require('http');

let content = fs.readFileSync('index.html');
let text = content.toString();
const $ = cheerio.load(text);

if (!fs.existsSync("dist")){
  fs.mkdirSync("dist");
}

if (!fs.existsSync("dist/js")){
  fs.mkdirSync("dist/js");
}

$('body > script').map((i, script) => {
  let attribs = script.attribs;
  if (!attribs.src.startsWith('http')) {
    let split = attribs.src.split("/");
    let file_name = split[split.length - 1];

    text = text.replace(attribs.src, `js/${file_name}`);

    fs.copyFile(attribs.src, `dist/js/${file_name}`, (err) => {
      if (err) throw err;
      console.log('source.txt was copied to destination.txt');
    });
  }
})

fs.writeFileSync('dist/index.html', text);
