import { addTodoCard, showAlert } from './ui.js';
import { createWorkItem } from './apiHandler.js';

export function setupFormListener() {
    const form = document.getElementById('formNewWorkItem');
    form.addEventListener('submit', async (event) => {
        event.preventDefault();
        const formData = new FormData(form);
        console.log(formData);
        const title = formData.get('inputTitle');
        const duration = formData.get('inputDuration');
        const durationType = formData.get('inputDurationType');
        const size = formData.get('inputSize');

        try {
            const workItem = await createWorkItem({ title, duration, durationType, size });
            addTodoCard(workItem);
            form.reset();
            showAlert('A new work item was successfully created!', 'success');
        } catch (error) {
            console.error('API call failed:', error);
            showAlert(`Failed to create a new work item. Reason: ${error}`, 'danger');
        }
    });
}