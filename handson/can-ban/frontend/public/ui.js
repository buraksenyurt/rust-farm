import {WorkItemManager} from './pkg/can_ban.js';

function formatDateTime(value) {
    const date = new Date(value);
    const options = {
        day: '2-digit',
        month: '2-digit',
        year: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
        hour12: false
    };
    return date.toLocaleString('tr-TR', options).replace(',', '');
}

export function bindCard(workItem) {
    const card = document.createElement("div");
    card.className = "card mb-3 bg-post-it-yellow";
    card.id = 'card' + workItem.id;
    card.style.backgroundColor = changeCardStyleFromWorkItem(workItem);

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
    <div class="card-body">
        <h5 class="card-title text-center">${workItem.id} - ${workItem.title}</h5>
        <p class="card-text">${workItem.duration} ${workItem.duration_type}</p>
        <p class="card-text">${workItem.size}</p>
        <p class="card-text text-end fw-bold">${formatDateTime(workItem.finish_date)}</p>
    </div>
`;

    cardBody.appendChild(moveLeft);
    cardBody.appendChild(cardContent);
    cardBody.appendChild(moveRight);

    const archiveDiv = document.createElement("div");
    archiveDiv.className = "d-flex justify-content-center";
    archiveDiv.style.marginTop = "10px";

    const archiveElement = document.createElement("i");
    archiveElement.className = "bi bi-archive";
    archiveElement.style.cursor = "pointer";
    archiveElement.onclick = () => moveToArchive(card);

    archiveDiv.appendChild(archiveElement);

    card.appendChild(cardBody);
    card.appendChild(archiveDiv);

    return card;
}

export function addTodoCard(workItem) {
    const divInProgress = document.getElementById('divTodo');
    const card = bindCard(workItem)
    divInProgress.appendChild(card);
}

export function addInProgressCard(workItem) {
    const divInProgress = document.getElementById('divInProgress');
    const card = bindCard(workItem)
    divInProgress.appendChild(card);
}

export function addCompletedCard(workItem) {
    const divInProgress = document.getElementById('divCompleted');
    const card = bindCard(workItem)
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
        changeStatus(card, targetIndex);
        changeCardStyle(card, targetIndex);
    }
}

function moveToArchive(card) {
    const manager = WorkItemManager.new();
    manager.move_to_archive(parseInt(card.id.toString().substring(4,)))
        .then(async _response => {
            showAlert('Work item was successfully moved to archive!', 'success');
            card.remove();
            await displayBoardReport();
        })
        .catch(error => {
            showAlert("Failed to move to archive. Reason is '" + error + "'", "danger");
        });
}

function changeStatus(card, columnIndex) {
    const manager = WorkItemManager.new();
    let status = "ToDo";
    switch (columnIndex) {
        case 0:
            status = "ToDo";
            break;
        case 1:
            status = "InProgress";
            break;
        case 2:
            status = "Completed";
            break;
    }

    manager.change_status(parseInt(card.id.toString().substring(4,)), status)
        .then(async _response => {
            console.log('Work item status was successfully changed!');
            showAlert('Work item status was successfully changed!', 'success');
            await displayBoardReport();
        })
        .catch(error => {
            console.log('API call failed on changing status!');
            showAlert("Failed to status update. Reason is '" + error + "'", "danger");
        });

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

function changeCardStyleFromWorkItem(workItem) {
    switch (workItem.status) {
        case 'Todo':
            return '#fff3cd';
        case 'Inprogress':
            return '#add8e6';
        case 'Completed':
            return '#77dd77';
    }
}

export function showAlert(message, type) {
    const alertPlaceholder = document.getElementById('alertPlaceholder');
    const alert = document.createElement('div');
    alert.className = `alert alert-${type} alert-dismissible fade show`;
    alert.role = 'alert';
    alert.innerHTML = `
        ${message}
        <button type='button' class='btn-close' data-dismiss='alert' aria-label='Close'></button>
    `;
    alertPlaceholder.appendChild(alert);
    setTimeout(() => alert.remove(), 3000);
}

export function displayWorkItems(workItems) {
    workItems.forEach(item => {
        switch (item.status) {
            case 'Todo':
                addTodoCard(item);
                break;
            case 'Inprogress':
                addInProgressCard(item);
                break;
            case 'Completed':
                addCompletedCard(item);
                break;
        }
    });
}

export async function displayBoardReport() {
    const manager = WorkItemManager.new();
    try {
        const jsStringResponse = await manager.get_board_report()
        const report = JSON.parse(jsStringResponse);
        document.getElementById('divTotalCount').innerText = report.work_items;
        document.getElementById('divTodoCount').innerText = report.todo_items;
        document.getElementById('divInProgressCount').innerText = report.in_progress_items;
        document.getElementById('divCompletedCount').innerText = report.completed_items;
    } catch (error) {
        console.error("Error fetching board items:", error);
        throw error;
    }
}