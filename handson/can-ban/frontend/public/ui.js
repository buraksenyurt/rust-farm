import {WorkItemManager} from './pkg/can_ban.js';

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
        <h5 class="card-title">${workItem.id} - ${workItem.title}</h5>
        <p class="card-text">Duration: ${workItem.duration} ${workItem.duration_type}</p>
        <p class="card-text">Size: ${workItem.size}</p>
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
        .then(_response => {
            //console.log('Work item was successfully moved to archive!');
            showAlert('Work item was successfully moved to archive!', 'success');
        })
        .catch(error => {
            //console.log('API call failed on changing status!');
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
        .then(_response => {
            console.log('Work item status was successfully changed!');
            showAlert('Work item status was successfully changed!', 'success');
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
    setTimeout(() => alert.remove(), 5000);
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