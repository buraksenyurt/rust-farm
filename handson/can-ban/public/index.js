import init, { WorkItemManager } from './pkg/can_ban.js';

async function run() {
    await init();

    const form = document.getElementById('formNewWorkItem');
    form.addEventListener('submit', (event) => {
        event.preventDefault();

        const title = document.getElementById('inputTitle').value;
        const duration = document.getElementById('inputDuration').value;
        const durationType = document.getElementById('inputDurationType').value;
        const size = document.getElementById('inputSize').value;

        const manager = WorkItemManager.new();
        let createdId = manager.create(title, duration, durationType, size);
        console.log("A new work item created..." + createdId);
        form.reset();
    });

}

run();