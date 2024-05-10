import init from './pkg/can_ban.js';
import {setupFormListener} from './formHandler.js';

async function run() {
    await init();
    setupFormListener();
}

run().then(() => {

}).catch(error => {
    console.log(error);
})