const testFolder = './dbg/';
const fs = require('fs');
const { exec } = require("child_process");

fs.readdir(testFolder, (err, files) => {
    files.forEach(file => {
        const name = file.split(".dot")[0];
        exec(`dot -Tsvg ${testFolder}${name}.dot > ${testFolder}${name}.svg`);
    });
});