import init from './pkg/can_ban.js';
import {setupFormListener} from './formHandler.js';
import {fetchWorkItems} from "./apiHandler.js";
import {displayWorkItems} from "./ui.js";

async function run() {
    await init();
    setupFormListener();
    try {
        const workItems = await fetchWorkItems();
        console.log(workItems);
        displayWorkItems(workItems);
    } catch (error) {
        console.error('Failed to load board items:', error);
    }
}

run().catch(error => {
    console.error('Error during app initialization:', error);
});