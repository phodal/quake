const fs = require('fs');
const cheerio = require("cheerio");
const fse = require('fs-extra');

let content = fs.readFileSync('index.html');
let text = content.toString();
const $ = cheerio.load(text);

fse.removeSync("dist");

if (!fs.existsSync("dist")){
  fs.mkdirSync("dist");
  fs.mkdirSync("dist/js");
  fs.mkdirSync("dist/css");
  fs.mkdirSync("dist/js/dashboard");
}

const srcDir = `./dashboard/dist/dashboard/`;
const destDir = `./dist/js/dashboard/`;

// To copy a folder or file
fse.copySync(srcDir, destDir, { overwrite: true }, function (err) {
  if (err) {
    console.error(err);
  } else {
    console.log("success!");
  }
});

$('link').map((i, link) => {
  let attribs = link.attribs;
  if (!(!!attribs.href && (!attribs.href.startsWith('http') && !attribs.href.startsWith('//')))) {
    return;
  }

  let split = attribs.href.split("/");
  let file_name = split[split.length - 1];

  text = text.replace(attribs.href, `css/${file_name}`);

  fs.copyFile(attribs.href, `dist/css/${file_name}`, (err) => {
    if (err) throw err;
  });
});

$('body > script').map((i, script) => {
  let attribs = script.attribs;

  if (!(!!attribs.src && (!attribs.src.startsWith('http') && !attribs.src.startsWith('//')))) {
    return;
  }

  let split = attribs.src.split("/");
  let file_name = split[split.length - 1];
  if (attribs.src.startsWith('/transflow/')) {
    return;
  }
  if (attribs.type !== 'module') {
    text = text.replace(attribs.src, `js/${file_name}`);

    fs.copyFile(attribs.src, `dist/js/${file_name}`, (err) => {
      if (err) throw err;
    });
  } else {
    let dir = file_name.split(".")[0];
    text = text.replace(attribs.src, `js/${dir}/${file_name}`);
  }
})

fs.writeFileSync('dist/index.html', text);
