import {BaseHTMLElement, customElement, getFirst, html} from 'dom-native';
import {Task} from '../model/task-model';

@customElement("task-view")
class TaskView extends BaseHTMLElement {
    #taskInputElement!:TaskInput;
    #taskListElement!:HTMLElement;

    init(){
        let htmlContent: DocumentFragment = html`
            <div><div>
            <h1>Görevler</h1>
            <task-input></task-input>
            <task-list>/<task-list>
        `;
        [this.#taskInputElement,this.#taskListElement] = getFirst(htmlContent,'task-input','task-list');
        this.append(htmlContent);
        this.refresh();
    }

    async refresh() {
        let task_list:Task[]=[
            {id:1,title:"Bulaşıkları yıka",state:"Ready"},
            {id:2,title:"10 Km koş",state:"Inprogress"}
        ];

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
    #titleEl!: HTMLElement;
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
            <c-check><c-ico name="ico-done"></c-ico></c-check>
            <div>Görev Başlığı Gelecek</div>
            <c-ico name="del"></c-ico>
        `;
        this.#titleEl = getFirst(htmlContent,'div');
        this.append(htmlContent);
        this.refresh();
    }

    refresh(old?:Task){

        if (old!=null){
            this.classList.remove(`Task-${old.id}`);
            this.classList.remove(old.state);
        }

        const task=this.#data;
        this.classList.add(`Task-${task.id}`);
        this.classList.add(task.state);
        this.#titleEl.textContent=task.title;
    }
}

declare global{
    interface HTMLElementTagNameMap{
        'task-item': TaskItem;
    }
}

