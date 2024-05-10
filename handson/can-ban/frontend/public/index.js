import init, {WorkItemManager} from './pkg/can_ban.js';

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
            .then(payload => {
                const workItem = JSON.parse(payload);
                console.log('A new work item was successfully created!' + payload);
                addTodoCard(workItem);
                form.reset();
                showAlert('A new work item was successfully created!', 'success');
            })
            .catch(error => {
                console.log('API call failed');
                showAlert("Failed to create a new work item. Reason is '" + error + "'", "danger");
            });
    });

}

function addTodoCard(workItem) {
    const divInProgress = document.getElementById('divTodo');

    const card = document.createElement("div");
    card.className = "card mb-3 bg-post-it-yellow";
    card.id = 'card' + workItem.id;

    const cardBody = document.createElement("div");
    cardBody.className = "card-body d-flex justify-content-between align-items-center";

    const moveLeft = document.createElement("i");
    moveLeft.className = "bi bi-arrow-left-circle-fill";
    moveLeft.style.cursor = "pointer";
    moveLeft.onclick = () => moveCard(card, 'left');

    const moveRight = document.createElement("i");
    moveRight.className = "bi bi-arrow-right-circle-fill";
    moveRight.style.cursor = "pointer";
    moveRight.onclick = () => moveCard(card, 'right');

    const cardContent = document.createElement("div");
    cardContent.innerHTML = `
        <h5 class="card-title">${workItem.title}</h5>
        <p class="card-text">Duration: ${workItem.duration} ${workItem.duration_type}</p>
        <p class="card-text">Size: ${workItem.size}</p>
    `;

    cardBody.appendChild(moveLeft);
    cardBody.appendChild(cardContent);
    cardBody.appendChild(moveRight);

    card.appendChild(cardBody);

    divInProgress.appendChild(card);
}

function moveCard(card, direction) {
    const currentColumn = card.parentElement;
    const allColumns = Array.from(document.querySelectorAll('.list-group'));
    const currentIndex = allColumns.indexOf(currentColumn);
    let targetIndex;

    if (direction === 'right' && currentIndex < allColumns.length - 1) {
        targetIndex = currentIndex + 1;
    } else if (direction === 'left' && currentIndex > 0) {
        targetIndex = currentIndex - 1;
    }

    if (targetIndex !== undefined) {
        allColumns[targetIndex].appendChild(card);
        changeCardStyle(card, targetIndex);
    }
}

function changeCardStyle(card, columnIndex) {
    switch (columnIndex) {
        case 0:
            card.style.backgroundColor = '#fff3cd';
            break;
        case 1:
            card.style.backgroundColor = '#add8e6';
            break;
        case 2:
            card.style.backgroundColor = '#77dd77';
            break;
    }
}

function showAlert(message, type) {
    const alertPlaceholder = document.getElementById('alertPlaceholder');
    const alert = document.createElement('div');
    alert.className = `alert alert-${type} alert-dismissible fade show`;
    alert.role = 'alert';
    alert.innerHTML = `
        ${message}
        <button type='button' class='btn-close' data-dismiss='alert' aria-label='Close'></button>
    `;
    alertPlaceholder.appendChild(alert);
    setTimeout(() => alert.remove(), 5000);
}

run();