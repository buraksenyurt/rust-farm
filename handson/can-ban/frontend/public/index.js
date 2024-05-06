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
        manager.create(title, duration, durationType, size)
            .then(success => {
                console.log("A new work item created");
                form.reset();
            })
            .catch(error => console.log("API call failed"));
    });

}

run();