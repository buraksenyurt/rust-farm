const express = require('express');
const https = require('https');
const fs = require('fs');
const app = express();
const port = 4447;

const options = {
    key: fs.readFileSync('../cert/key.pem'),
    cert: fs.readFileSync('../cert/cert.pem'),
    passphrase: 'P@ssw0rd'
};

app.use(express.static('public'));
app.use('/pkg', express.static('pkg'));

https.createServer(options, app).listen(port, () => {
    console.log(`Server is running on https://localhost:${port}`);
});