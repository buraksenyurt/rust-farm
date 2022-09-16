import {BaseHTMLElement, customElement, getFirst, html} from 'dom-native';
import {Task, taskMac} from '../model/task-model';

@customElement("task-view")
class TaskView extends BaseHTMLElement {
    #taskInputElement!:TaskInput;
    #taskListElement!:HTMLElement;

    init(){
        let htmlContent: DocumentFragment = html`
            <div></div>
            <div class="container">
                <h3>Gündem</h3>
                <task-input></task-input>
                <task-list>/<task-list>
            </div>
        `;
        [this.#taskInputElement,this.#taskListElement] = getFirst(htmlContent,'task-input','task-list');
        this.append(htmlContent);
        this.refresh();
    }

    async refresh() {
        let task_list:Task[] = await taskMac.getAllTasks();
        console.log(task_list);
        let htmlContent=document.createDocumentFragment();
        for(const task of task_list){
            const ti=document.createElement('task-item');
            ti.data=task;
            htmlContent.append(ti);
        }

        this.#taskListElement.innerHTML = '';
        this.#taskListElement.append(htmlContent);
    }
}

@customElement("task-input")
class TaskInput extends BaseHTMLElement {
    #inputEl!: HTMLInputElement;

    init(){
        let htmlContent = html`
            <input type="text" placeholder="Sıkıcı bir görev ekleyebilirsin?">
        `;
        this.#inputEl = getFirst(htmlContent,'input');
        this.append(htmlContent);
    }
}

declare global{
    interface HTMLElementTagNameMap{
        'task-input': TaskInput;
    }
}

@customElement("task-item")
class TaskItem extends BaseHTMLElement {
    #titleLabelEl!: HTMLElement;
    #checkboxEl!: HTMLInputElement;
    #data!: Task;

    set data(data:Task){
        let oldData=this.#data;
        this.#data=Object.freeze(data);
        if (this.isConnected){
            this.refresh(oldData);
        }
    }

    get data(){
        return this.#data
    }

    init(){
        let htmlContent = html`
            <div>
                <input type="checkbox" value="" id="taskState">
                <label>Görev Başlığı Gelecek</label>
                <button type="button" class="btn btn-danger">Sil</button>
            </div>
        `;
        this.#titleLabelEl = getFirst(htmlContent,'label');
        this.#checkboxEl = getFirst(htmlContent,'input');
        this.append(htmlContent);
        this.refresh();
    }

    refresh(old?:Task){
//         if (old!=null){
//             this.classList.remove(`Task-${old.id}`);
//             this.classList.remove(old.state);
//         }

        const task=this.#data;
        console.log(task.state);
//         this.classList.add(`Task-${task.id}`);
//         this.classList.add(task.state);
        this.#titleLabelEl.textContent=task.title;
        if(task.state=="Completed"){
            this.#titleLabelEl.classList.add(`text-success`);
            this.#checkboxEl.checked=true;
        }else if (task.state=="Ready"){
            this.#titleLabelEl.classList.add(`text-warning`);
        }
        else if (task.state=="Inprogress"){
            this.#titleLabelEl.classList.add(`text-important`);
        }
    }
}

declare global{
    interface HTMLElementTagNameMap{
        'task-item': TaskItem;
    }
}

