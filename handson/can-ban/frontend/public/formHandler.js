import {addTodoCard, showAlert} from './ui.js';
import {createWorkItem, getWorkItemsCount} from './apiHandler.js';

export function setupFormListener() {
    const form = document.getElementById('formNewWorkItem');
    form.addEventListener('submit', async (event) => {
        event.preventDefault();
        const formData = new FormData(form);
        // console.log(formData);
        const title = formData.get('inputTitle');
        const duration = formData.get('inputDuration');
        const durationType = formData.get('inputDurationType');
        const size = formData.get('inputSize');

        if (title.length < 10 || title.length > 100) {
            showAlert('Title must be between 10 and 100 characters long', 'danger');
            return;
        }

        if (!duration) {
            showAlert('Please give me an estimated duration', 'danger');
            return;
        }

        if (!durationType) {
            showAlert('Duration type must be selected', 'danger');
            return;
        }

        if (!size) {
            showAlert('Size must be selected', 'danger');
            return;
        }

        try {
            const itemsCount = await getWorkItemsCount();
            if (itemsCount >= 5) {
                showAlert('There can be only five(5) work items at the board...','danger');
                return;
            }

            const workItem = await createWorkItem({title, duration, durationType, size});
            addTodoCard(workItem);
            form.reset();
            showAlert('A new work item was successfully created!', 'success');
        } catch (error) {
            console.error('API call failed:', error);
            showAlert(`Failed to create a new work item. Reason: ${error}`, 'danger');
        }
    });
}