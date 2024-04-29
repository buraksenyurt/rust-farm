const express = require('express');
const app = express();
const port = 4447;

app.use(express.static('public'));
app.use('/pkg', express.static('pkg')); 

app.listen(port, () => {
    console.log(`Server is running on http://localhost:${port}`);
});